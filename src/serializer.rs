use crate::{box_to_owned, BoxPack, Data};

/// Used to serialize a specific structure into raw Data.
pub trait Serializer {
    type Error: std::error::Error;

    fn serialize(
        &self,
        data_type: impl Into<u16>,
        queue: u8
    ) -> Result<BoxPack<'_>, Self::Error>
        where Self: Sized;

    fn serialize_owned(
        self,
        data_type: impl Into<u16>,
        queue: u8
    ) -> Result<BoxPack<'static>, Self::Error>
        where Self: Sized
    {
        let box_pack = self.serialize(
            data_type,
            queue
        )?;

        Ok(box_to_owned(box_pack))
    }
}


// Vec<u8> can always be serialized into BoxPack
impl Serializer for Vec<u8> {
    type Error = std::convert::Infallible;

    fn serialize(
        &self,
        data_type: impl Into<u16>,
        queue: u8
    ) -> Result<BoxPack<'_>, Self::Error>
        where Self: Sized
    {
        Ok(BoxPack::new(
            Data::from(self),
            data_type,
            queue
        ))
    }

    fn serialize_owned(
        self,
        data_type: impl Into<u16>,
        queue: u8
    ) -> Result<BoxPack<'static>, Self::Error>
        where Self: Sized
    {
        Ok(BoxPack::new(
            Data::from(self),
            data_type,
            queue
        ))
    }
}
