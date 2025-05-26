use super::{ Engine };

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DefaultEngine;

impl Engine for DefaultEngine {}