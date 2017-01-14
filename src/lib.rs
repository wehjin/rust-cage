#[cfg(test)]
mod tests;
mod frame;
mod offset;
mod translation;

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

pub use translation::Translation;

#[derive(Copy, Clone, Debug, Default)]
pub struct Cage {
    pub frame: Frame,
    pub offset: Offset,
}

impl Cage {
    /// Construct a cage from its sides
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Self {
        Cage::from((left, right, bottom, top, far, near))
    }

    /// Construct a cage with zero origin and zero frame
    pub fn new_zero() -> Self {
        Cage { frame: FRAME_ZERO, offset: OFFSET_ZERO }
    }

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

    pub fn contains(&self, x: f32, y: f32, z: f32) -> bool {
        let (l, r, b, t, f, n) = self.limits();
        x >= l && x < r && y >= b && y < t && z >= f && z < n
    }

    /// Individually move the sides of a cage
    ///
    /// ```
    /// use cage::{Cage,Translation};
    ///
    /// let translation = Translation {
    ///     left: -1.0, right: 1.0, bottom:-1.5, top:1.5, far:-2.0, near:2.0
    /// };
    /// let translated = Cage::new_zero().translate_sides(translation);
    /// assert_eq!(2.0, translated.frame.w);
    /// assert_eq!(3.0, translated.frame.h);
    /// assert_eq!(4.0, translated.frame.d);
    /// ```
    pub fn translate_sides(&self, translation: Translation) -> Self {
        let (left, right, bottom, top, far, near) = self.limits();
        Cage::new(
            left + translation.left,
            right + translation.right,
            bottom + translation.bottom,
            top + translation.top,
            far + translation.far,
            near + translation.near
        )
    }
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

