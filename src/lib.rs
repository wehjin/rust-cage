#[cfg(test)]
mod tests;
mod frame;
mod offset;

pub use frame::{
    Frame,
    ZERO as FRAME_ZERO,
    UNIT as FRAME_UNIT,
};
pub use offset::{
    Offset,
    ZERO as OFFSET_ZERO,
    UNIT as OFFSET_UNIT,
    CENTI as OFFSET_CENTI,
    MILLI as OFFSET_MILLI,
};

#[derive(Copy, Clone, Debug)]
pub struct Cage {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    far: f32,
    near: f32
}

impl Cage {
    pub fn unit_xy() -> Self {
        Cage { left: -1.0, right: 1.0, bottom: -1.0, top: 1.0, far: 0.0, near: 0.0 }
    }

    pub fn scale_xy(&self, scale_x: f32, scale_y: f32) -> Self {
        Cage {
            left: self.left * scale_x, right: self.right * scale_x,
            bottom: self.bottom * scale_y, top: self.top * scale_y,
            far: self.far, near: self.near
        }
    }

    pub fn shift_x(&self, shift: f32) -> Self {
        Cage {
            left: self.left + shift, right: self.right + shift,
            bottom: self.bottom, top: self.top,
            far: self.far, near: self.near
        }
    }

    pub fn shift_y(&self, shift: f32) -> Self {
        Cage {
            left: self.left, right: self.right,
            bottom: self.bottom + shift, top: self.top + shift,
            far: self.far, near: self.near
        }
    }

    pub fn tuple(&self) -> (f32, f32, f32, f32, f32, f32) {
        (self.left, self.right, self.bottom, self.top, self.far, self.near)
    }
}
