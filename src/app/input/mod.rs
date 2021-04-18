pub mod button;
pub mod mapping;
pub mod system;

pub use button::*;
pub use mapping::*;
pub(in crate::app) use system::*; // @XXX how to avoid this???

use crate::app::imdraw::ImDraw;


#[derive(Copy, Clone, Debug, ImDraw)]
pub struct ControllerAxisThreshold {
    value: f32,
    direction: ControllerAxisDirection
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControllerAxisDirection {
    GreaterThan,
    LesserThan,
}

impl ControllerAxisThreshold {
    pub fn greater_than(value: f32) -> ControllerAxisThreshold {
        Self {
            value,
            direction: ControllerAxisDirection::GreaterThan,
        }
    }

    pub fn lesser_than(value: f32) -> ControllerAxisThreshold {
        Self {
            value,
            direction: ControllerAxisDirection::LesserThan,
        }
    }
}

impl_imdraw_todo!(ControllerAxisDirection);
