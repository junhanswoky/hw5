use lazy_static::lazy_static;
use std::net::SocketAddr;
use volo_example::LogLayer;
use std::process;
use std::sync::Arc;
use volo::FastStr;
use volo_gen::mini_redis::{RedisRequest,RequestType,RedisService, ResponseType};
use std::io;
use anyhow::{Error, Result};
use volo_thrift::AnyhowError;
use volo::Service;

lazy_static! {
    static ref CLIENT: volo_gen::mini_redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
            .layer_outer(LogLayer)    
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("input command:");
    loop {
        print!(">");
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
        let tmp = tmp.strip_suffix("\n").unwrap().to_string();

        let c: Vec<String> = tmp.split(' ').map(|str| str.to_string()).collect();

        let command = match c[0].as_str() {
            "exit" => {
                process::exit(0);
            }
            "set" => RedisRequest {
                key: Some(FastStr::from(Arc::new(c.get(1).unwrap().clone()))),
                value: Some(c.get(2).unwrap().clone().into()),
                extime: None,
                req_type: RequestType::Set,
            },
            "get" => RedisRequest {
                key: Some(FastStr::from(Arc::new(c.get(1).unwrap().clone()))),
                value: None,
                extime: None,
                req_type: RequestType::Get,
            },
            "del" => RedisRequest {
                key: Some(FastStr::from(Arc::new(c.get(1).unwrap().clone()))),
                value: None,
                extime: None,
                req_type: RequestType::Del,
            },
            "ping" => {
                let key = if c.len() > 1 {
                    Some(FastStr::from(Arc::new(c.get(1).unwrap().clone())))
                } else {
                    None
                };
                RedisRequest {
                    key,
                    value: None,
                    extime: None,
                    req_type: RequestType::Ping,
                }
            }
            _ => {
                panic!("Error");
            }
        };
        let ret = CLIENT.redis_command(command).await;
        match ret {
            Ok(info) => println!("{:?}", info.value.unwrap()),
            Err(e) => tracing::error!("{:?}", e),
        }
    }
    
}