use crate::proxy::route::{Route, RouteLocator, RouteManager};
use async_trait::async_trait;
use pingora::http::ResponseHeader;
use pingora::modules::http::grpc_web::GrpcWeb;
use pingora::modules::http::HttpModules;
use pingora::prelude::{HttpPeer, ProxyHttp, Session};

pub struct Gateway {
    pub route_manager: RouteManager<Box<dyn RouteLocator>>,
}

#[derive(Default)]
pub struct Ctx {
    route: Option<Route>,
}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = Ctx;

    fn new_ctx(&self) -> Self::CTX {
        Self::CTX::default()
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
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
