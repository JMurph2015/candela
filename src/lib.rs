pub mod types {
    use serde::{Deserialize, Serialize};
    include!(concat!(env!("OUT_DIR"), "/candela.rs"));
}

mod error;
mod sockets;
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
    fn get_ips() -> Vec<[u8; 4]>;
    fn get_subnet() -> [u8; 4];
    fn get_netmask() -> [u8; 4];
    fn get_setup_port() -> u32;
}

pub trait CandelaServerConfig: CandelaConfig {}

pub trait CandelaClientConfig: CandelaConfig {
    fn get_name(&self) -> String;
    fn get_setup_port(&self) -> u16;
    fn get_strip_configs(&self) -> Vec<types::LedStripConfig>;
}

pub trait CandelaServer {
    type Controller: CandelaController;
    fn new<T: CandelaServerConfig>(config: T) -> Result<Self>
    where
        Self: Sized;
    fn search() -> Vec<types::LedControllerConfig>;
    fn connect(config: types::LedControllerConfig) -> Result<()>;
    fn get_controllers(&mut self) -> &mut Vec<Self::Controller>;
}
pub trait CandelaController {
    fn get_strips(&mut self) -> &mut Vec<CandelaStrip>;
}

pub trait CandelaClient {
    fn new<T: CandelaClientConfig>(config: T) -> Result<Self>
    where
        Self: Sized;
    fn setup(&mut self) -> Result<()>;
    fn recv(&mut self) -> Result<types::ClientMessage>;
}
