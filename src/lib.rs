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

#[derive(Copy, Clone, Debug, Default)]
pub struct Cage {
    pub frame: Frame,
    pub offset: Offset,
}

impl From<(Frame, Offset)> for Cage {
    fn from((frame, offset): (Frame, Offset)) -> Self {
        Cage { frame: frame, offset: offset }
    }
}

impl From<Frame> for Cage {
    fn from(frame: Frame) -> Self {
        Cage { frame: frame, offset: Default::default() }
    }
}

impl From<(f32, f32, f32, f32, f32, f32)> for Cage {
    fn from(limits: (f32, f32, f32, f32, f32, f32)) -> Self {
        let frame = Frame::from(limits);
        let offset = Offset::from(limits);
        Cage { frame: frame, offset: offset }
    }
}

impl Cage {
    pub fn limits(&self) -> (f32, f32, f32, f32, f32, f32) {
        let (half_w, half_h, half_d) = (self.frame.w / 2.0, self.frame.h / 2.0, self.frame.d / 2.0);
        (
            self.offset.x - half_w, self.offset.x + half_w,
            self.offset.y - half_h, self.offset.y + half_h,
            self.offset.z - half_d, self.offset.z + half_d
        )
    }

    pub fn shift(&self, dx: f32, dy: f32, dz: f32) -> Self {
        Cage { frame: self.frame, offset: self.offset.shift(dx, dy, dz) }
    }

    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        Cage { frame: self.frame.scale(sx, sy, sz), offset: self.offset }
    }
}
