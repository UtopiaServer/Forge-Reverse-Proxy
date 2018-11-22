use std::convert::TryFrom;
use super::{VarErr};

///
/// Structure used to represent a variable 32bit
/// integer sended in Minecraft network exchanges
/// 
/// It use try_from to recover the value from a byte slice
/// 
/// Protocol details can be found [here](https://wiki.vg/Protocol#VarInt_and_VarLong)
///
/// # Examples
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
/// use network_variables::{VarErr, VarInt};
///
/// let value = VarInt::try_from(&[255, 1][..]);
/// assert_eq!(value, Ok(VarInt::new(255)));
///
/// let value = VarInt::try_from(&[255, 255, 255, 255, 7][..]);
/// assert_eq!(value, Ok(VarInt::new(2147483647)));
///
/// let value = VarInt::try_from(&[255, 255, 255, 255, 15][..]);
/// assert_eq!(value, Ok(VarInt::new(-1)));
/// ```
///
/// # Errors
/// 
/// In case of a wrongly sent packet, two errors may be returned:
/// 
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
/// use network_variables::{VarErr, VarInt};
///
/// let value = VarInt::try_from(&[255, 255, 255, 255, 255, 28][..]);
/// assert_eq!(value, Err(VarErr::TooBig));
///
/// let value = VarInt::try_from(&[][..]);
/// assert_eq!(value, Err(VarErr::NoBytes));
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

/// Allow the creation of a VarInt from a i32
///
/// # Examples
/// ```
/// use network_variables::VarInt;
/// 
/// let value = VarInt::new(42);
/// 
/// assert_eq!(value.0, 42);
/// ```
impl VarInt {
    pub fn new(value: i32) -> Self {
        VarInt(value)
    }
}


/// Allow the creation of a VarInt from a byte slice
///
/// # Examples
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
/// use network_variables::VarInt;
/// 
/// let value = VarInt::try_from(&[255, 1][..]).unwrap();
/// 
/// assert_eq!(value.0, 255);
/// ```
impl TryFrom<&[u8]> for VarInt {
    type Error = VarErr;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut result: i32 = 0;

        for (i, byte) in bytes.iter().enumerate() {
            result |= i32::from(byte & 0b111_1111) << (7 * i);
            if (byte & 0b1000_0000) == 0 {
                return Ok(VarInt::new(result))
            } else if i >= 4 {
                return Err(VarErr::TooBig)
            }
        }
        Err(VarErr::NoBytes)
    }
}
