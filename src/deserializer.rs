use crate::BoxPack;

/// Used to deserialize the box's raw Data into a specific structure.
/// # Example
/// ```rust
/// use std::string::FromUtf8Error;
/// use bqsp::BoxPack;
/// use bqsp::deserializer::BoxDeserializer;
///
/// struct MyStruct {
///     message: String,
/// }
///
/// impl BoxDeserializer for MyStruct {
///     type Error = FromUtf8Error;
///
///     fn deserialize_box(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized {
///         let bytes = data.data.as_ref();
///         String::from_utf8(bytes.to_vec())
///             .map(|string| MyStruct { message: string })
///     }
/// }
/// ```
pub trait BoxDeserializer {
    type Error;

    /// Construct Self from BoxPack.
    fn deserialize_box(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized;
}


// BoxPack can always be deserialized into Vec<u8>
impl BoxDeserializer for Vec<u8> {
    type Error = std::convert::Infallible;

    fn deserialize_box(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized {
        Ok(data.data.as_ref().to_vec())
    }
}
