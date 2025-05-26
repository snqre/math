use crate::q;

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(Clone)]
#[derive(Copy)]
pub struct DefaultEngine;

pub fn new() -> DefaultEngine {
    DefaultEngine
}

toga::blockset! {
    impl DefaultEngine;

    q::Engine {}
}