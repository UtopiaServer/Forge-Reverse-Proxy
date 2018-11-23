///
/// Structure used to represent a variable 64bit integer
/// sended in Minecraft network exchanges
/// 
/// It use from_slice to recover the value from a byte slice
/// 
/// Protocol details can be found [here](https://wiki.vg/Protocol#VarInt_and_VarLong)
///
/// # Examples
/// ```
/// use network_variables::VarLong;
///
/// let value = VarLong::from_slice(&[255, 1][..]);
/// assert_eq!(value, Ok((&[][..], VarLong::new(255))));
///
/// let value = VarLong::from_slice(&[255, 255, 255, 255, 255, 255, 255, 255, 127][..]);
/// assert_eq!(value, Ok((&[][..], VarLong::new(9223372036854775807))));
///
/// let value = VarLong::from_slice(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 1][..]);
/// assert_eq!(value, Ok((&[][..], VarLong::new(-1))));
/// ```
///
/// # Errors
/// 
/// In case of a wrongly sent packet, errors may be returned:
/// 
/// ```
/// use nom;
/// use network_variables::VarLong;
///
/// let value = VarLong::from_slice(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 28][..]);
/// assert_eq!(value, Err(nom::Err::Incomplete(nom::Needed::Unknown)));
///
/// let value = VarLong::from_slice(&[][..]);
/// assert_eq!(value, Err(nom::Err::Incomplete(nom::Needed::Size(1))));
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl VarLong {

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
    pub fn new(value: i64) -> Self {
        VarLong(value)
    }


    /// Allow the creation of a VarLong from a byte slice
    ///
    /// # Examples
    /// ```
    /// use network_variables::VarLong;
    /// 
    /// let value = VarLong::from_slice(&[255, 1][..]).unwrap();
    /// 
    /// assert_eq!((value.1).0, 255);
    /// ```
    pub fn from_slice(bytes: &[u8]) ->  nom::IResult<&[u8], Self> {
        let mut result: i64 = 0;

        for (i, byte) in bytes.iter().enumerate() {
            result |= i64::from(byte & 0b111_1111) << (7 * i);
            if (byte & 0b1000_0000) == 0 {
                return Ok((&bytes[i + 1..], VarLong::new(result)))
            } else if i >= 9 {
                return Err(nom::Err::Incomplete(nom::Needed::Unknown))
            }
        }
        Err(nom::Err::Incomplete(nom::Needed::Size(1)))
    }
}

