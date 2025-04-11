use async_trait::async_trait;
use bytes::Bytes;
use pingora::http::RequestHeader;
use pingora::prelude::Session;
use serde::Deserialize;
use serde_json::Value;
use crate::plugin::{Plugin, PLUGIN_FACTORY};
use crate::proxy::gateway::Ctx;
use crate::register_plugin;

#[derive(Deserialize)]
pub struct RequestTransformer {
    remove_headers: Vec<String>,
}

#[async_trait]
impl Plugin for RequestTransformer {
    fn new(conf: Value) -> Self
    where
        Self: Sized,
    {
        serde_json::from_value(conf).unwrap()
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Ctx) -> pingora::Result<bool> {
        let req = session.req_header_mut();
        for rm_header in self.remove_headers.iter() {
            req.remove_header(rm_header);
        }
        Ok(false)
    }
}

register_plugin!(RequestTransformer, RequestTransformer);