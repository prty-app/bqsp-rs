use crate::deserializer::BoxDeserializer;
use super::*;


#[derive(Clone, Debug)]
/// Box representation after data deserialization (bytes are transformed into a desired structure).
///
/// Used for handling and presenting the data.
pub struct BoxDes<T: BoxDeserializer> {
    pub header: Header,
    pub data: T,
}

impl<T: BoxDeserializer> TryFrom<BoxPack<'_>> for BoxDes<T> {
    type Error = T::Error;

    fn try_from(box_pack: BoxPack) -> Result<Self, Self::Error> {
        Ok(Self {
            header: box_pack.header,
            data: T::deserialize_box(box_pack)?,
        })
    }
}
