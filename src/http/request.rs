use crate::http::method::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_to_array(buf: &[u8]) -> Result<Self, String> {
        let string = String::from("abc");
        string.encrypt();
        buf.encrypt();
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
/** Not remove
/*
 Here we create a trait named Encrypt wich will allow us
 to encrypt our received message. This is an extend functionnality.
 It allow us to later implement new function for different types
 */
trait Encrypt {
    fn encrypt(self) -> Self;
}

/*
  Here we implement Encrypt trait for type String
 */
impl Encrypt for String {
    fn encrypt(self) -> Self {
        unimplemented!();
    }
}

/*
  Now we implement for our slice buffer
 */
impl Encrypt for &[u8] {
    fn encrypt(self) -> Self {
        unimplemented!()
    }
}
**/