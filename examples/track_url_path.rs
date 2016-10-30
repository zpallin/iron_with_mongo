
extern crate iron_with_mongo;

// unfortunately, one cannot easily reexport macros in rust without unstable
// I opt not to use the unstable version
#[macro_use(bson, doc)] extern crate bson;

use iron_with_mongo::prelude::*;

// Generic Route Handler
fn route_handler(req: &mut Request) -> IronResult<Response> {

    // reference to url query
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    
    // querystr for multiple uses
    let querystr: String = String::from(*query);

    // configuration for db and db connection taken from persistent state objs
    let db_conf = req.extensions.get::<Read<DatabaseConfigKey>>().unwrap();
    let db_conn = req.extensions.get::<Read<MongoClientKey>>().unwrap();

    // grabbing collection
    let coll = db_conn.db(&db_conf.dbname).collection("hits");
    
    // will be used as search parameters in the db search
    let hit_counter_search = doc! {
        "query" => querystr
    };

    // get the result from the db query
    let mut cursor = coll.find(Some(hit_counter_search), None).ok().expect("Couldn't Find");

    let item = cursor.next();

    match item {
        Some(Ok(x)) => {
            let output = format!("{}: {}", x.get("title").unwrap(), *query);
            return Ok(Response::with((status::Ok, output)));
        },   
        Some(Err(_)) => { 
            return Ok(Response::with((status::Ok, "Error getting data")));
        },
        None => {
            return Ok(Response::with((status::Ok, format!("{}: {}", "<NONE>", *query))));     
        },
    }
    
    //  db_conf.display();

}


fn main() {

    let db_conf = DatabaseConfig::new("localhost:27017/test");
    let db_conn = Client::connect(&db_conf.hostname, db_conf.port)
        .ok().expect("Failed to initialize db connection");

    let mut router = Router::new();
    router.get("/", route_handler, "index");
    router.get("/:query", route_handler, "query");

    let mut config = Chain::new(router);
    config.link(Read::<DatabaseConfigKey>::both(db_conf));
    config.link(Read::<MongoClientKey>::both(db_conn));

    println!("Running app on localhost:3000");
    Iron::new(config).http("localhost:3000").unwrap();
}
