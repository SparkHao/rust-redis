use std::collections::HashMap;
use std::option::Option;
use redis::{Client, Commands, RedisResult, Connection};

fn main() {
    println!("Hello, world!");

    let redis_object = ReidsSaveObject::new();
    let r1: Result<(), redis::RedisError> = redis_object.con.unwrap().set("free", "spark");
    println!("r1: {:?}", r1);
    
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    
    let config: HashMap<String, isize> = redis::cmd("CONFIG").arg("GET").arg("*-max-*-entries").query(&mut con).unwrap();
    println!("config: {:?}", config);
    println!("Max entry limits:");

    println!(
        "  max-intset:        {}",
        config.get("set-max-intset-entries").unwrap_or(&0)
    );
    println!(
        "  hash-max-ziplist:  {}",
        config.get("hash-max-ziplist-entries").unwrap_or(&0)
    );
    println!(
        "  list-max-ziplist:  {}",
        config.get("list-max-ziplist-entries").unwrap_or(&0)
    );
    println!(
        "  zset-max-ziplist:  {}",
        config.get("zset-max-ziplist-entries").unwrap_or(&0)
    );

    let demo_key = "demo_key";
    let save: RedisResult<()> = Ok(redis::cmd("set").arg(demo_key).arg(100).execute(&mut con));
    println!("save: {:?}", save);
    let query: isize = redis::cmd("get").arg(demo_key).query(&mut con).unwrap();
    println!("query: {:?}", query);

    let a: bool = con.set(demo_key, 101).unwrap();
    println!("a: {:?}", a);
    let b: isize = con.get(demo_key).unwrap();
    println!("b: {:?}", b);

    // let mut pub_sub_conn = client.get_async_connection().await.unwrap();
    // let mut publish_conn = client.get_async_connection().await.unwrap().into_pubsub();

    // Ok(())
} 

struct ReidsSaveObject {
    con: Option<Connection>,
}

struct ReidsQueryObject {
    con: Connection,
}

impl ReidsSaveObject {
    pub fn new() -> Self{
        let client = Client::open("redis://127.0.0.1/").unwrap();
        let conn = client.get_connection();
        ReidsSaveObject { con: conn.ok() }
    }
}

impl ReidsQueryObject {
    pub fn new() -> Self{
        let client = Client::open("redis://127.0.0.1/").unwrap();
        let conn = client.get_connection();
        ReidsQueryObject { con: conn.unwrap() }
    }
}