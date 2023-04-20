use deadpool_diesel::postgres::{Manager, Pool};

pub fn get_pool() -> Pool {
    // get database url from environment
    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager)
        .build()
        .unwrap()
}
