
#[macro_use(bson, doc)] extern crate bson;
extern crate mongodb;
extern crate iron_with_db;

pub mod prelude {

    pub use iron_with_db::prelude::*;
    pub use iron_with_db::db::{DatabaseConfig, DatabaseConfigKey};
    pub use bson::Bson;
    pub use mongodb::{Client, ThreadedClient};
    pub use mongodb::db::ThreadedDatabase;

    /// key for passing mongo client into the server chain
    #[derive(Copy, Clone)]
    pub struct MongoClientKey;
    impl Key for MongoClientKey {
        type Value = Client;
    }
}

/// module with tools for building handlers with mongo
pub mod handler;

/// module for handling authentication with the mongo server
pub mod auth;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
