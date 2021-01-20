// TODO(murphyj) Remove once this hits MVP
#![allow(dead_code)]
use async_trait::async_trait;

pub mod types {
    use serde::{Deserialize, Serialize};
    include!(concat!(env!("OUT_DIR"), "/candela.rs"));
}

pub mod error;
#[cfg(any(feature = "zmq-client", feature = "zmq-server"))]
pub mod sockets;
mod strip;

pub type Error = error::CandelaError;
pub type Result<T> = std::result::Result<T, error::CandelaError>;

pub use strip::CandelaStrip;

/**
 * There are four elements to each pixel corresponding to red, green, blue and
 * N/A respectively.  The trailing element of the array is unused, but there to
 * ensure proper conversion to little endian u32.
 */
pub type Pixel = [u8; 4];

pub trait CandelaConfig {
    fn get_ips(&self) -> Vec<[u8; 4]>;
    fn get_subnet(&self) -> [u8; 4];
    fn get_netmask(&self) -> [u8; 4];
    fn get_setup_port(&self) -> u16;
}

pub trait CandelaServerConfig: CandelaConfig {}

pub trait CandelaClientConfig: CandelaConfig {
    fn get_name(&self) -> String;
    fn get_strip_configs(&self) -> Vec<types::LedStripConfig>;
}

// Handle for the overall setup and control of connections to clients
#[async_trait]
pub trait CandelaServer: std::fmt::Debug {
    type Controller: CandelaController;
    fn new<T: CandelaServerConfig>(config: T) -> Result<Self>
    where
        Self: Sized;
    async fn search() -> Vec<types::LedControllerConfig>;
    async fn connect(config: types::LedControllerConfig) -> Result<()>;
    fn get_controllers(&mut self) -> &mut Vec<Self::Controller>;
}

// Handle for representing a single controller (aka client) serverside.
// Most actions are taken against strips which are then synced by syncing the
// whole controller.
pub trait CandelaController {
    fn get_strips(&mut self) -> &mut Vec<CandelaStrip>;
}

#[async_trait]
pub trait CandelaClient {
    fn new<T: CandelaClientConfig>(config: T) -> Result<Self>
    where
        Self: Sized;
    async fn setup(&mut self) -> Result<()>;
    async fn recv(&mut self) -> Result<types::ClientMessage>;
}
