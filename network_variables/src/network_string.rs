use nom;
use super::VarInt;

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkString(pub String);

impl NetworkString {

    /// Allow the creation of a VarInt from a i32
    ///
    /// # Examples
    /// ```
    /// use network_variables::NetworkString;
    ///
    /// let value = NetworkString::new(String::from("foo"));
    ///
    /// assert_eq!(value.0, String::from("foo"));
    /// ```
    pub fn new(value: String) -> Self {
        NetworkString(value)
    }

    /// Allow the creation of a VarInt from a byte slice
    ///
    /// # Examples
    /// ```
    /// use network_variables::NetworkString;
    ///
    /// let value = NetworkString::from_slice(&[3, 'f' as u8, 'o' as u8, 'o' as u8][..]).unwrap();
    /// assert_eq!((value.1).0, String::from("foo"));
    /// ```
    pub fn from_slice(bytes: &[u8]) -> nom::IResult<&[u8], Self> {
        let length = VarInt::from_slice(bytes).unwrap();
        let bytes = length.0;

        if bytes.len() < (length.1).0 as usize {
            Err(nom::Err::Incomplete(nom::Needed::Size((length.1).0 as usize)))
        } else {
            let (string, remaining) = bytes.split_at((length.1).0 as usize);
            Ok((
                remaining,
                NetworkString::new(String::from_utf8(Vec::from(string)).unwrap())
            ))
        }
    }
}

