#![feature(impl_trait_in_assoc_type)]

use std::collections::HashMap;
use std::sync::Mutex;
use volo_gen::mini_redis::{RedisRequest, RedisResponse, RedisService, RequestType, ResponseType};
use anyhow::{Error, Result};
use volo_thrift::AnyhowError;

pub struct S {
	pub data: Mutex<HashMap<String,String>>,
}

#[volo::async_trait]
impl volo_gen::mini_redis::RedisService for S {
	async fn redis_command(
		&self, 
		command: volo_gen::mini_redis::RedisRequest
	) -> ::core::result::Result<volo_gen::mini_redis::RedisResponse, ::volo_thrift::AnyhowError> {
		match command.req_type {
			RequestType::Get => {
                let response_value = self.data.lock().unwrap().get(&command.key.unwrap().into_string())
                    .map(|value| value.clone().into())
                    .unwrap_or_else(|| "Not found.".to_owned().into());

                Ok(RedisResponse {
                    value: Some(response_value),
                    resp_type: ResponseType::Output,
                })
            }
			RequestType::Set => {
                self.data.lock().unwrap().insert(
                    command.key.unwrap().into_string(),
                    command.value.unwrap().into_string(),
                );

                Ok(RedisResponse {
                    value: Some("Set success.".to_owned().into()),
                    resp_type: ResponseType::Output,
                })
            }
			RequestType::Del => {
                let response_value = self.data.lock().unwrap().remove(&command.key.unwrap().into_string())
                    .map(|_| "Delete success.".to_owned().into())
                    .unwrap_or_else(|| "Not found.".to_owned().into());

                Ok(RedisResponse {
                    value: Some(response_value),
                    resp_type: ResponseType::Output,
                })
            }
            RequestType::Ping => {
                let response_value = command.key.map(|key| key.clone().into()).unwrap_or_else(|| "PONG".to_owned().into());

                Ok(RedisResponse {
                    value: Some(response_value),
                    resp_type: ResponseType::Output,
                })
            }
		}
	}
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug + From<Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
		let command = format!("{:?}", &req);
		if command.contains("asd") {
			return Err(S::Error::from(Error::msg("Wrong information")));
		}
        let resp = self.0.call(cx, req).await;
        resp
    }
}
pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;
    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}