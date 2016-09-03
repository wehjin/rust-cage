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

#[test]
fn default_frame() {
    let frame: Frame = Default::default();
    assert_eq!((0.01, 0.01, 0.01), (frame.w, frame.h, frame.d));
}

#[test]
fn unit_frame() {
    let frame = Frame::unit();
    assert_eq!((0.01, 0.01, 0.01), (frame.w, frame.h, frame.d));
}

#[test]
fn zero_frame() {
    let frame = Frame::zero();
    assert_eq!((0.0, 0.0, 0.0), (frame.w, frame.h, frame.d));
}

#[test]
fn scale_frame() {
    let frame = Frame { w: 1.0, h: 1.0, d: 1.0 }.scale(1.0, 2.0, 3.0);
    assert_eq!((1.0, 2.0, 3.0), (frame.w, frame.h, frame.d));
}

#[test]
fn zero_offset() {
    let offset = OFFSET_ZERO;
    assert_eq!((0.0, 0.0,0.0), offset.tuple());
}

#[test]
fn unit_offset() {
    let offset = OFFSET_UNIT;
    assert_eq!((1.0, 1.0,1.0), offset.tuple());
}

#[test]
fn centi_offset() {
    let offset = OFFSET_CENTI;
    assert_eq!((0.01, 0.01, 0.01), offset.tuple());
}

#[test]
fn milli_offset() {
    let offset = OFFSET_MILLI;
    assert_eq!((0.001, 0.001, 0.001), offset.tuple());
}

#[test]
fn scale_offset() {
    let offset = OFFSET_UNIT.scale(1.0, 2.0, 3.0);
    assert_eq!((1.0, 2.0, 3.0), offset.tuple())
}

#[test]
fn tuple_to_offset() {
    let offset = Offset::from((1.0, 2.0, 3.0));
    assert_eq!((1.0, 2.0, 3.0), offset.tuple())
}

#[test]
fn float_to_offset() {
    let offset = Offset::from(4.0);
    assert_eq!((4.0, 4.0, 4.0), offset.tuple())
}

#[test]
fn shift_offset() {
    let offset = Offset::from(1.0).shift(1.0, 2.0, 3.0);
    assert_eq!((2.0, 3.0, 4.0), offset.tuple())
}