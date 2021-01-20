use async_trait::async_trait;
use super::CandelaSocketController;
use crate::{types, CandelaServer, CandelaServerConfig, Result};

#[derive(Debug)]
pub struct CandelaSocketServer {}

#[async_trait]
impl CandelaServer for CandelaSocketServer {
    type Controller = CandelaSocketController;

    fn new<T: CandelaServerConfig>(_config: T) -> Result<Self> {
        unimplemented!()
    }

    async fn search() -> Vec<types::LedControllerConfig> {
        unimplemented!()
    }

    async fn connect(_config: types::LedControllerConfig) -> Result<()> {
        unimplemented!()
    }

    fn get_controllers(&mut self) -> &mut Vec<Self::Controller> {
        unimplemented!()
    }
}
