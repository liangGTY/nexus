use nexus_core::proxy::gateway::Gateway;
use nexus_core::route::route_refresh::RouteRefresh;
use nexus_core::route::static_file_conf_locator::StaticFileConfLocator;
use nexus_core::route::RouteStore;
use pingora::prelude::{background_service, Opt};
use pingora::proxy::http_proxy_service;
use pingora::server;
use pingora::server::configuration::ServerConf;
use std::error::Error;
use std::sync::Arc;

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse_args();
    let mut server = server::Server::new(opt)?;
    server.bootstrap();
    let arc = Arc::new(ServerConf::default());
    let manager = Arc::new(RouteStore::new());
    let mut service = http_proxy_service(
        &arc,
        Gateway {
            route_manager: manager.clone(),
        },
    );

    let refresh = RouteRefresh::new( StaticFileConfLocator::new("config/route.json"), manager.clone());

    service.add_tcp("0.0.0.0:8080");
    server.add_service(service);
    server.add_service(background_service("s", refresh));
    server.run_forever();
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
