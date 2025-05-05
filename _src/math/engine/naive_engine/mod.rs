::boiler::bundle!("src/math/engine/naive_engine");
::boiler::expose!(constructor, engine);

use crate::math::q;
use crate::common::int;

/// A faster but less precise engine. Based on benchmarks
/// it is at least 20% faster than the default engine.
#[derive(Debug)]
#[derive(Clone)]
pub struct NaiveEngine;