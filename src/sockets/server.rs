use super::CandelaSocketController;
use crate::{Result, types, CandelaServer, CandelaServerConfig};

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
