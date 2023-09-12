use std::net::SocketAddr;
use std::sync::Mutex;
use std::collections::HashMap;
use volo_example::{LogLayer, S};
use volo_gen::mini_redis::{RedisService, RedisRequest, RedisResponse, RequestType, ResponseType};
use volo::net::Address;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = Address::from(addr);

    let redis_server = S {
        data: Mutex::new(HashMap::<String, String>::new()),
    };

    volo_gen::mini_redis::RedisServiceServer::new(redis_server)
        .layer_front(LogLayer)
        .run(addr)
        .await
        .unwrap();
}
