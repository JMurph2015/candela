use super::CandelaSocketController;
use crate::{types, CandelaServer, CandelaServerConfig, Result};

pub struct CandelaSocketServer {}

impl CandelaServer for CandelaSocketServer {
    type Controller = CandelaSocketController;

    fn new<T: CandelaServerConfig>(config: T) -> Result<Self> {
        unimplemented!()
    }

    fn search() -> Vec<types::LedControllerConfig> {
        unimplemented!()
    }
    fn get_controllers(&mut self) -> &mut Vec<Self::Controller> {
        unimplemented!()
    }
    fn connect(config: types::LedControllerConfig) -> Result<()> {
        unimplemented!()
    }
}
