use crate::num::q;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct DefaultEngine;

pub fn new() -> DefaultEngine {
    DefaultEngine
}

impl q::Engine for DefaultEngine {}