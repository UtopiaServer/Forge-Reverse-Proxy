use std::convert::TryFrom;
use super::VarErr;

///
/// Structure used to represent a variable 64bit integer
/// sended in Minecraft network exchanges
/// 
/// It use try_from to recover the value from a byte slice
/// 
/// Protocol details can be found [here](https://wiki.vg/Protocol#VarInt_and_VarLong)
///
/// # Examples
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
/// use network_variables::{VarErr, VarLong};
///
/// let value = VarLong::try_from(&[255, 1][..]);
/// assert_eq!(value, Ok(VarLong::new(255)));
///
/// let value = VarLong::try_from(&[255, 255, 255, 255, 255, 255, 255, 255, 127][..]);
/// assert_eq!(value, Ok(VarLong::new(9223372036854775807)));
///
/// let value = VarLong::try_from(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 1][..]);
/// assert_eq!(value, Ok(VarLong::new(-1)));
/// ```
///
/// # Errors
/// 
/// In case of a wrongly sent packet, two errors may be returned:
/// 
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
/// use network_variables::{VarErr, VarLong};
///
/// let value = VarLong::try_from(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 28][..]);
/// assert_eq!(value, Err(VarErr::TooBig));
///
/// let value = VarLong::try_from(&[][..]);
/// assert_eq!(value, Err(VarErr::NoBytes));
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct VarLong(pub i64);

/// Allow the creation of a VarLong from a i64
///
/// # Examples
/// ```
/// use network_variables::VarLong;
/// 
/// let value = VarLong::new(92233720368547758);
/// 
/// assert_eq!(value.0, 92233720368547758);
/// ```
impl VarLong {
    pub fn new(value: i64) -> Self {
        VarLong(value)
    }
}

/// Allow the creation of a VarLong from a byte slice
///
/// # Examples
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
/// use network_variables::VarLong;
/// 
/// let value = VarLong::try_from(&[255, 1][..]).unwrap();
/// 
/// assert_eq!(value.0, 255);
/// ```
impl TryFrom<&[u8]> for VarLong {
    type Error = VarErr;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut result: i64 = 0;

        for (i, byte) in bytes.iter().enumerate() {
            result |= i64::from(byte & 0b111_1111) << (7 * i);
            if (byte & 0b1000_0000) == 0 {
                return Ok(VarLong::new(result))
            } else if i >= 9 {
                return Err(VarErr::TooBig)
            }
        }
        Err(VarErr::NoBytes)
    }
}

