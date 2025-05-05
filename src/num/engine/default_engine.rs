use crate::num::q;

#[derive(Debug)]
#[derive(Clone)]
pub struct DefaultEngine;

pub fn new() -> DefaultEngine {
    DefaultEngine
}

impl q::Engine for DefaultEngine {}