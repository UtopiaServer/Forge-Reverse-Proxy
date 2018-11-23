///
/// Tests for VarInt
///

use super::VarInt;
use nom;

#[test]
fn test_zero() {
    let slice: &[u8] = &[0];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(0))));
}

#[test]
fn test_one() {
    let slice: &[u8] = &[1];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(1))));
}

#[test]
fn test_two() {
    let slice: &[u8] = &[2];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(2))));
}

#[test]
fn test_one_byte() {
    let slice: &[u8] = &[127];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(127))));
}

#[test]
fn test_two_byte() {
    let slice: &[u8] = &[128, 1];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(128))));
}

#[test]
fn test_two_byte_max() {
    let slice: &[u8] = &[255, 1];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(255))));
}


#[test]
fn test_max() {
    let slice: &[u8] = &[255, 255, 255, 255, 7];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(2147483647))));
}


#[test]
fn test_minus() {
    let slice: &[u8] = &[255, 255, 255, 255, 15];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(-1))));
}

#[test]
fn test_min() {
    let slice: &[u8] = &[128, 128, 128, 128, 8];
    let test_0 = VarInt::from_slice(slice);
    assert_eq!(test_0, Ok((&[][..], VarInt::new(-2147483648))));
}
