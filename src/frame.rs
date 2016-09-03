#[derive(Copy, Clone, Debug)]
pub struct Frame {
    pub w: f32,
    pub h: f32,
    pub d: f32
}

pub const ZERO: Frame = Frame { w: 0.0, h: 0.0, d: 0.0 };
pub const UNIT: Frame = Frame { w: 1.0, h: 1.0, d: 1.0 };

impl Frame {
    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        Frame { w: self.w * sx, h: self.h * sy, d: self.d * sz }
    }

    pub fn tuple(&self) -> (f32, f32, f32) {
        (self.w, self.h, self.d)
    }
    pub fn limits(&self) -> (f32, f32, f32, f32, f32, f32) {
        let (half_w, half_h, half_d) = (self.w / 2.0, self.h / 2.0, self.d / 2.0);
        (-half_w, half_w, -half_h, half_h, -half_d, half_d)
    }
}

impl From<(f32, f32, f32, f32, f32, f32)> for Frame {
    fn from((l, r, b, t, f, n): (f32, f32, f32, f32, f32, f32)) -> Self {
        Frame { w: r - l, h: t - b, d: n - f }
    }
}

impl From<(f32, f32, f32)> for Frame {
    fn from((w, h, d): (f32, f32, f32)) -> Self {
        Frame { w: w, h: h, d: d }
    }
}

impl From<f32> for Frame {
    fn from(l: f32) -> Self {
        Frame { w: l, h: l, d: l }
    }
}

impl Default for Frame {
    fn default() -> Self {
        UNIT
    }
}

