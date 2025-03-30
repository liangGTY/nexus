use async_trait::async_trait;
use bytes::Bytes;
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
        Self: Sized
    {
        let plugin = serde_json::from_value(conf).unwrap();
        plugin
    }

    async fn request_filter(&self, _session: &mut Session, _ctx: &mut Ctx) -> pingora::Result<bool> {
        todo!()
    }

    async fn request_body_filter(&self, _session: &mut Session, _body: &mut Option<Bytes>, end_of_stream: bool, ctx: &mut Ctx) -> pingora::Result<()> {
        todo!()
    }
}

register_plugin!(RequestTransformer, RequestTransformer);