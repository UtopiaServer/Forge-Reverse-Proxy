use nom;

/// Structure used to represent a variable 32bit
/// integer sended in Minecraft network exchanges
/// 
/// It use from_slice to recover the value from a byte slice
/// 
/// Protocol details can be found [here](https://wiki.vg/Protocol#VarInt_and_VarLong)
///
/// # Examples
/// ```
/// use network_variables::VarInt;
///
/// let value = VarInt::from_slice(&[255, 1][..]);
/// assert_eq!(value, Ok((&[][..], VarInt::new(255))));
///
/// let value = VarInt::from_slice(&[255, 255, 255, 255, 7][..]);
/// assert_eq!(value, Ok((&[][..], VarInt::new(2147483647))));
///
/// let value = VarInt::from_slice(&[255, 255, 255, 255, 15][..]);
/// assert_eq!(value, Ok((&[][..], VarInt::new(-1))));
/// ```
///
/// # Errors
/// 
/// In case of a wrongly sent packet, two errors may be returned:
/// 
/// ```
/// use network_variables::VarInt;
///
/// let value = VarInt::from_slice(&[255, 255, 255, 255, 255, 28][..]);
/// assert_eq!(value, Err(nom::Err::Incomplete(nom::Needed::Unknown)));
///
/// let value = VarInt::from_slice(&[][..]);
/// assert_eq!(value, Err(nom::Err::Incomplete(nom::Needed::Size(1))));
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct VarInt(pub i32);

impl VarInt {


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
    pub fn new(value: i32) -> Self {
        VarInt(value)
    }



    /// Allow the creation of a VarInt from a byte slice
    ///
    /// # Examples
    /// ```
    /// use network_variables::VarInt;
    /// 
    /// let value = VarInt::from_slice(&[255, 1][..]).unwrap();
    /// 
    /// assert_eq!((value.1).0, 255);
    /// ```
    pub fn from_slice(bytes: &[u8]) ->  nom::IResult<&[u8], Self> {
        let mut result: i32 = 0;

        for (i, byte) in bytes.iter().enumerate() {
            result |= i32::from(byte & 0b111_1111) << (7 * i);
            if (byte & 0b1000_0000) == 0 {
                return Ok((&bytes[i + 1..], VarInt::new(result)))
            } else if i >= 4 {
                return Err(nom::Err::Incomplete(nom::Needed::Unknown))
            }
        }
        Err(nom::Err::Incomplete(nom::Needed::Size(1)))
    }
}

