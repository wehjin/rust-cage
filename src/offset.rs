#[derive(Copy, Clone, Debug)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const ZERO: Offset = Offset { x: 0.0, y: 0.0, z: 0.0 };
pub const UNIT: Offset = Offset { x: 1.0, y: 1.0, z: 1.0 };
pub const CENTI: Offset = Offset { x: 0.01, y: 0.01, z: 0.01 };
pub const MILLI: Offset = Offset { x: 0.001, y: 0.001, z: 0.001 };

impl From<(f32, f32, f32, f32, f32, f32)> for Offset {
    fn from((l, r, b, t, f, n): (f32, f32, f32, f32, f32, f32)) -> Self {
        Offset { x: (l + r) / 2.0, y: (b + t) / 2.0, z: (f + n) / 2.0 }
    }
}

impl From<(f32, f32, f32)> for Offset {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Offset { x: x, y: y, z: z }
    }
}

impl From<f32> for Offset {
    fn from(a: f32) -> Self {
        Offset { x: a, y: a, z: a }
    }
}

impl Default for Offset {
    fn default() -> Self {
        ZERO
    }
}

impl Offset {
    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Self {
        Offset { x: self.x * sx, y: self.y * sy, z: self.z * sz }
    }
    pub fn shift(&self, dx: f32, dy: f32, dz: f32) -> Self {
        Offset { x: self.x + dx, y: self.y + dy, z: self.z + dz }
    }
    pub fn tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

