use super::*;

#[test]
fn unit_xy() {
    let cage = Cage::unit_xy();
    assert_eq!((-1.0, 1.0, -1.0, 1.0, 0.0, 0.0), cage.tuple());
}

#[test]
fn scale_xy() {
    let cage = Cage::unit_xy().scale_xy(2.0, 3.0);
    assert_eq!((-2.0, 2.0, -3.0, 3.0, 0.0, 0.0), cage.tuple());
}

#[test]
fn shift_x() {
    let cage = Cage::unit_xy().shift_x(1.0);
    assert_eq!((0.0, 2.0, -1.0, 1.0, 0.0, 0.0), cage.tuple());
}

#[test]
fn shift_y() {
    let cage = Cage::unit_xy().shift_y(-1.0);
    assert_eq!((-1.0, 1.0, -2.0, 0.0, 0.0, 0.0), cage.tuple());
}
