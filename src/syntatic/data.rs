use std::borrow::Cow;
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug, Clone)]
/// The data part of the Box.
/// # Fields
/// - bytes - owned or borrowed bytes ready to be send
pub struct Data<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> Deref for Data<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.bytes
    }
}

impl<'a> AsRef<Data<'a>> for &Data<'a> {
    fn as_ref(&self) -> &Data<'a> {
        self
    }
}

impl<'a> From<&'a [u8]> for Data<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self {
            bytes: Cow::Borrowed(bytes),
        }
    }
}

impl<'a> From<Vec<u8>> for Data<'a> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Cow::Owned(bytes),
        }
    }
}

impl<'a> From<&'a Vec<u8>> for Data<'a> {
    fn from(bytes: &'a Vec<u8>) -> Self {
        Self {
            bytes: Cow::Borrowed(bytes.as_slice()),
        }
    }
}
