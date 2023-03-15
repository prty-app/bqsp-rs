use std::borrow::Cow;
use std::ops::Deref;

#[repr(transparent)]
#[derive(Debug)]
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
