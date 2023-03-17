use crate::BoxPack;

/// Used to deserialize the box's raw Data into a specific structure.
/// # Example
/// ```rust
/// use std::string::FromUtf8Error;
/// use bqsp::BoxPack;
/// use bqsp::deserializer::Deserializer;
///
/// struct MyStruct {
///     message: String,
/// }
///
/// impl Deserializer for MyStruct {
///     type Error = FromUtf8Error;
///
///     fn deserialize(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized {
///         let bytes = data.data.as_ref();
///         String::from_utf8(bytes.to_vec())
///             .map(|string| MyStruct { message: string })
///     }
/// }
/// ```
pub trait Deserializer {
    type Error;

    /// Construct Self from BoxPack.
    fn deserialize(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized;
}


// BoxPack can always be deserialized into Vec<u8>
impl Deserializer for Vec<u8> {
    type Error = std::convert::Infallible;

    fn deserialize(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized {
        Ok(data.data.as_ref().to_vec())
    }
}
