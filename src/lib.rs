#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_xy() {
        let cage = CageXyz::unit_xy();
        assert_eq!((-1.0, 1.0, -1.0, 1.0, 0.0, 0.0), cage.tuple());
    }

    #[test]
    fn scale_xy() {
        let cage = CageXyz::unit_xy().scale_xy(2.0, 3.0);
        assert_eq!((-2.0, 2.0, -3.0, 3.0, 0.0, 0.0), cage.tuple());
    }

    #[test]
    fn shift_x() {
        let cage = CageXyz::unit_xy().shift_x(1.0);
        assert_eq!((0.0, 2.0, -1.0, 1.0, 0.0, 0.0), cage.tuple());
    }

    #[test]
    fn shift_y() {
        let cage = CageXyz::unit_xy().shift_y(-1.0);
        assert_eq!((-1.0, 1.0, -2.0, 0.0, 0.0, 0.0), cage.tuple());
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CageXyz {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    far: f32,
    near: f32
}

impl CageXyz {
    pub fn unit_xy() -> Self {
        CageXyz { left: -1.0, right: 1.0, bottom: -1.0, top: 1.0, far: 0.0, near: 0.0 }
    }

    pub fn scale_xy(&self, scale_x: f32, scale_y: f32) -> Self {
        CageXyz {
            left: self.left * scale_x, right: self.right * scale_x,
            bottom: self.bottom * scale_y, top: self.top * scale_y,
            far: self.far, near: self.near
        }
    }

    pub fn shift_x(&self, shift: f32) -> Self {
        CageXyz {
            left: self.left + shift, right: self.right + shift,
            bottom: self.bottom, top: self.top,
            far: self.far, near: self.near
        }
    }

    pub fn shift_y(&self, shift: f32) -> Self {
        CageXyz {
            left: self.left, right: self.right,
            bottom: self.bottom + shift, top: self.top + shift,
            far: self.far, near: self.near
        }
    }

    pub fn tuple(&self) -> (f32, f32, f32, f32, f32, f32) {
        (self.left, self.right, self.bottom, self.top, self.far, self.near)
    }
}
