#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    w: f32,
    h: f32,
    d: f32
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
