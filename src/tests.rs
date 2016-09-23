use super::*;

#[test]
fn default_cage() {
    let cage: Cage = Default::default();
    assert_eq!((-0.5,0.5,-0.5,0.5,-0.5,0.5), cage.limits())
}

#[test]
fn cage_from_limits() {
    let cage = Cage::from((0.0, 3.0, 0.0, 4.0, 0.0, 5.0));
    assert_eq!((0.0, 3.0, 0.0, 4.0, 0.0, 5.0), cage.limits());
}

#[test]
fn shift_cage() {
    let cage = Cage::from((0.0, 1.0, 0.0, 1.0, 0.0, 1.0)).shift(1.0, 2.0, 3.0);
    assert_eq!((1.0, 2.0, 2.0, 3.0, 3.0, 4.0), cage.limits());
}

#[test]
fn scale_cage() {
    let cage = Cage::from((-1.0, 1.0, -1.0, 1.0, -1.0, 1.0)).scale(1.0, 2.0, 3.0);
    assert_eq!((-1.0, 1.0, -2.0, 2.0, -3.0, 3.0), cage.limits());
}

#[test]
fn cage_contains() {
    let cage = Cage::from((-1.0, 1.0, -1.0, 1.0, -1.0, 1.0));
    assert!(cage.contains(0.0, 0.0, 0.0));
    assert!(!cage.contains(2.0, 0.0, 0.0));
    assert!(!cage.contains(-2.0, 0.0, 0.0));
    assert!(!cage.contains(0.0, 2.0, 0.0));
    assert!(!cage.contains(0.0, -2.0, 0.0));
    assert!(!cage.contains(0.0, 0.0, 2.0));
    assert!(!cage.contains(0.0, 0.0, -2.0));
}

#[test]
fn cage_contains_at_offset() {
    let cage = Cage::from((-10.0, -9.0, -1.0, 1.0, -1.0, 1.0));
    assert!(!cage.contains(0.0, 0.0, 0.0));
}

#[test]
fn tuple_to_frame() {
    let frame = Frame::from((1.0, 2.0, 3.0));
    assert_eq!((1.0,2.0,3.0), frame.tuple())
}

#[test]
fn float_to_frame() {
    let frame = Frame::from(4.0);
    assert_eq!((4.0,4.0,4.0), frame.tuple());
}

#[test]
fn default_frame() {
    let frame: Frame = Default::default();
    assert_eq!((1.0, 1.0, 1.0), (frame.w, frame.h, frame.d));
}

#[test]
fn unit_frame() {
    let frame = FRAME_UNIT;
    assert_eq!((1.0, 1.0, 1.0), frame.tuple());
}

#[test]
fn zero_frame() {
    let frame = FRAME_ZERO;
    assert_eq!((0.0, 0.0, 0.0), frame.tuple());
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
fn default_offset() {
    let offset: Offset = Default::default();
    assert_eq!((0.0,0.0,0.0), offset.tuple());
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