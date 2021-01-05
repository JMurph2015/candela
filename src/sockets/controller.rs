use crate::{CandelaController, CandelaStrip};

pub struct CandelaSocketController {}

impl CandelaController for CandelaSocketController {
    fn get_strips(&mut self) -> &mut Vec<CandelaStrip> {
        unimplemented!()
    }
}
