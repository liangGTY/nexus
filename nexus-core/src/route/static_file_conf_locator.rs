use crate::route::route_conf_locator::RouteConfLocator;
use crate::route::RouteConf;
use async_trait::async_trait;
use tokio::fs;

/// 静态配置文件加载
pub struct StaticFileConfLocator {
    file: String,
}

impl StaticFileConfLocator {
    pub fn new(path: &str) -> StaticFileConfLocator {
        StaticFileConfLocator {
            file: path.to_string(),
        }
    }
}

#[async_trait]
impl RouteConfLocator for StaticFileConfLocator {
    async fn get_routes(&self) -> Vec<RouteConf> {
        let json_string = fs::read_to_string(self.file.clone()).await.unwrap();

        serde_json::from_str(json_string.as_str()).expect("json deserialization error")
    }
}
