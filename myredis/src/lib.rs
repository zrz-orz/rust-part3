#![feature(impl_trait_in_assoc_type)]
use futures::future::select_all;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tracing::info;
use volo::FastStr;
pub mod arg;

pub struct S {
    pub port: u16,
    pub map: Arc<Mutex<HashMap<String, String>>>,
    pub channels: Mutex<HashMap<String, broadcast::Sender<String>>>,
}

use volo_gen::myredis::{RedisResponse, RequestType, ResponseType};

#[volo::async_trait]
impl volo_gen::myredis::RedisService for S {
    async fn redis_command(
        &self,
        _req: volo_gen::myredis::RedisRequest,
    ) -> ::core::result::Result<volo_gen::myredis::RedisResponse, ::volo_thrift::AnyhowError> {
        info!("enter");
        match _req.request_type {
            RequestType::Set => {
                if _req.expire_time.is_some() {
                    let map_clone = self.map.clone();
                    let key = _req.key.clone().unwrap().into_string();
                    let expire_time = _req.expire_time.unwrap();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(expire_time as u64))
                            .await;
                        let mut map = map_clone.lock().unwrap();
                        map.remove(&key);
                    });
                }
                self.map.lock().unwrap().insert(
                    _req.key.unwrap().into_string(),
                    _req.value.unwrap().into_string(),
                );
                Ok(RedisResponse {
                    value: Some("Ok".into()),
                    response_type: ResponseType::Print,
                })
            }
            RequestType::Get => {
                match self
                    .map
                    .lock()
                    .unwrap()
                    .get(&_req.key.unwrap().into_string())
                {
                    Some(v) => Ok(RedisResponse {
                        value: Some(FastStr::from(v.clone())),
                        response_type: ResponseType::Print,
                    }),
                    None => Ok(RedisResponse {
                        value: Some("nil".into()),
                        response_type: ResponseType::Print,
                    }),
                }
            }
            RequestType::Del => {
                match self
                    .map
                    .lock()
                    .unwrap()
                    .remove(&_req.key.unwrap().into_string())
                {
                    Some(_) => Ok(RedisResponse {
                        value: Some("Ok".into()),
                        response_type: ResponseType::Print,
                    }),
                    None => Ok(RedisResponse {
                        value: Some("nil".into()),
                        response_type: ResponseType::Print,
                    }),
                }
            }
            RequestType::Ping => Ok(RedisResponse {
                value: Some(_req.value.unwrap_or("PONG".into())),
                response_type: ResponseType::Print,
            }),
            RequestType::Subscribe => match _req.block.unwrap() {
                true => {
                    let mut vec = self
                        .channels
                        .lock()
                        .unwrap()
                        .iter()
                        .filter(|(k, _v)| {
                            _req.channels
                                .as_ref()
                                .unwrap()
                                .contains(&FastStr::from((*k).clone()))
                        })
                        .map(|(k, v)| (v.subscribe(), k.clone()))
                        .collect::<Vec<_>>();
                    let (res, index, _) =
                        select_all(vec.iter_mut().map(|(rx, _name)| Box::pin(rx.recv()))).await;
                    match res {
                        Ok(info) => Ok(RedisResponse {
                            value: Some(
                                (String::from("from ") + &vec[index].1 + ": " + &info).into(),
                            ),
                            response_type: ResponseType::Trap,
                        }),
                        Err(_) => Ok(RedisResponse {
                            value: None,
                            response_type: ResponseType::Trap,
                        }),
                    }
                }
                false => {
                    for channel in _req.channels.unwrap() {
                        self
                            .channels
                            .lock()
                            .unwrap().entry(channel.clone().into_string()).or_insert_with(|| {
                            let (tx, _) = broadcast::channel(10);
                            tx
                        });
                    }
                    Ok(RedisResponse {
                        value: Some("Ok".into()),
                        response_type: ResponseType::Trap,
                    })
                }
            },
            RequestType::Publish => {
                let channel = _req.channels.unwrap()[0].clone().into_string();
                let _ = self
                    .channels
                    .lock()
                    .unwrap()
                    .get(&channel)
                    .unwrap()
                    .send(_req.value.unwrap().into_string())
                    .unwrap();
                info!("send over");
                Ok(RedisResponse {
                    value: Some("Ok".into()),
                    response_type: ResponseType::Print,
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
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
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

#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        self.0.call(cx, req).await
    }
}

pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
