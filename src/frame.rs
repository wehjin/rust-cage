#[derive(Copy, Clone, Debug)]
pub struct Frame {
    pub w: f32,
    pub h: f32,
    pub d: f32
}

impl Default for Frame {
    fn default() -> Self {
        Frame::unit()
    }
}

impl Frame {
    pub fn unit() -> Self {
        Frame { w: 0.01, h: 0.01, d: 0.01 }
    }
    pub fn zero() -> Self {
        Frame { w: 0.0, h: 0.0, d: 0.0 }
    }

    pub fn scale(&self, scale_w: f32, scale_h: f32, scale_d: f32) -> Self {
        Frame { w: self.w * scale_w, h: self.h * scale_h, d: self.d * scale_d }
    }
}
