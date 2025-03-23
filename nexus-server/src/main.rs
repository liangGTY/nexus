use async_trait::async_trait;
use nexus_core::proxy::gateway::Gateway;
use nexus_core::proxy::route::{Route, RouteLocator, RouteManager};
use pingora::prelude::Opt;
use pingora::proxy::http_proxy_service;
use pingora::server;
use pingora::server::configuration::ServerConf;
use std::error::Error;
use std::sync::Arc;

struct Some {}

#[async_trait]
impl RouteLocator for Some {
    async fn get_routes(&self) -> Vec<Route> {
        todo!()
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse_args();
    let mut server = server::Server::new(opt)?;
    server.bootstrap();
    let arc = Arc::new(ServerConf::default());
    let mut service = http_proxy_service(
        &arc,
        Gateway {
            route_manager: RouteManager::new(Box::new(Some {})),
        },
    );

    service.add_tcp("0.0.0.0:8080");
    server.add_service(service);
    server.run_forever();
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
