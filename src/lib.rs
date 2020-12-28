pub mod types {
    use serde::{Serialize, Deserialize};
    include!(concat!(env!("OUT_DIR"), "/candela.rs"));
}