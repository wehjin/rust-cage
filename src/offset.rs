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

impl Default for Offset {
    fn default() -> Self {
        CENTI
    }
}

impl Offset {}