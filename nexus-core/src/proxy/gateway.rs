use crate::route::{Route, RouteStore};
use async_trait::async_trait;
use pingora::http::ResponseHeader;
use pingora::modules::http::grpc_web::GrpcWeb;
use pingora::modules::http::HttpModules;
use pingora::prelude::{HttpPeer, ProxyHttp, Session};
use std::sync::Arc;

pub struct Gateway {
    pub route_manager: Arc<RouteStore>,
}

#[derive(Default)]
pub struct Ctx {
    pub route: Option<Arc<Route>>,
}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = Ctx;

    fn new_ctx(&self) -> Self::CTX {
        Self::CTX::default()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        todo!()
    }

    fn init_downstream_modules(&self, modules: &mut HttpModules) {
        modules.add_module(Box::new(GrpcWeb));
    }

    /// router locater
    async fn early_request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        self.route_manager
            .match_route(_session.req_header_mut(), _ctx);

        Ok(())
    }

    async fn request_body_filter(
        &self,
        _session: &mut Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        todo!()
    }

    fn upstream_response_filter(
        &self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) {
        todo!()
    }
}
