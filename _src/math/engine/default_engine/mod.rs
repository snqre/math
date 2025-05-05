::boiler::bundle!("src/math/engine/default_engine");
::boiler::expose!(constructor, engine);

use crate::math::q;
    
#[derive(Debug)]
#[derive(Clone)]
pub struct DefaultEngine;