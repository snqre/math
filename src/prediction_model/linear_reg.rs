use crate::common::int;
use crate::common::q;
use crate::coordinate::point_2d;

// --- --- ---

pub struct LinearReg<const A: u8, B, C, const D: usize> 
where
    B: int::Int,
    C: q::Engine {
    data: [point_2d::Point2D<A, B, C>; D],
    x_weight: q::Q<A, B, C>,
    y_weight: q::Q<A, B, C>,
    bias: q::Q<A, B, C>
}

impl<const A: u8, B, C, const D: usize> LinearReg<A, B, C, D>
where
    B: int::Int,
    C: q::Engine {
    
    pub fn predict(&self, where_x: q::Q<A, B, C>) -> q::Result<q::Q<A, B, C>> {
        // Apply the learned x_weight to the input x and add the bias
        let x_term = (self.x_weight * where_x)?;
        let prediction = (x_term + self.bias)?;

        Ok(prediction)
    }

    pub fn predict_point(&self, point: &point_2d::Point2D<A, B, C>) -> q::Result<q::Q<A, B, C>> {
        let x_term = (self.x_weight * point.x)?;
        let y_term = (self.y_weight * point.y)?;
        Ok((x_term + y_term)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn predict() {
        let model: LinearReg<2, i32, q::DefaultEngine, 2> = LinearReg {
            data: [
                point_2d::Point2D { x: 1_00.into(), y: 20_00.into() },
                point_2d::Point2D { x: 2_00.into(), y: 21_00.into() }
            ],
            x_weight: 1_00.into(),
            y_weight: 1_00.into(),
            bias: 0_00.into()
        };
        let prediction = model.predict(3_00.into()).unwrap();
        assert_eq!(prediction, 22_00.into());
    }
}