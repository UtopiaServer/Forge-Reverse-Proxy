///
/// Tests for VarInt
///

use super::VarLong;

#[test]
fn test_zero() {
    let slice: &[u8] = &[0];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(0))));
}

#[test]
fn test_one() {
    let slice: &[u8] = &[1];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(1))));
}

#[test]
fn test_two() {
    let slice: &[u8] = &[2];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(2))));
}

#[test]
fn test_one_byte() {
    let slice: &[u8] = &[127];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(127))));
}

#[test]
fn test_two_byte() {
    let slice: &[u8] = &[128, 1];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(128))));
}

#[test]
fn test_two_byte_max() {
    let slice: &[u8] = &[255, 1];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(255))));
}

#[test]
fn test_maxlong() {
    let slice: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255, 127];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(9223372036854775807))));
}


#[test]
fn test_minus() {
    let slice: &[u8] = &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(-1))));
}

#[test]
fn test_minint() {
    let slice: &[u8] = &[128, 128, 128, 128, 248, 255, 255, 255, 255, 1];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(-2147483648))));
}

#[test]
fn test_minlong() {
    let slice: &[u8] = &[128, 128, 128, 128, 128, 128, 128, 128, 128, 1];
    let test_0 = VarLong::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarLong::new(-9223372036854775808))));
}
