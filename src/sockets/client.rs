use crate::{types, CandelaClient, CandelaClientConfig, Result};

pub struct CandelaSocketClient {}

impl CandelaClient for CandelaSocketClient {
    fn new<T: CandelaClientConfig>(config: T) -> Result<Self> {
        unimplemented!()
    }
    fn setup() -> Result<()> {
        unimplemented!()
    }
    fn recv() -> Result<types::ClientMessage> {
        unimplemented!()
    }
}
