use super::*;


#[derive(Clone)]
/// Box representation with raw bytes (packed).
///
/// Used mainly for temporary use before receiving or sending it.
pub struct BoxPack<'d> {
    pub header: Header,
    pub data: Data<'d>,
}

pub fn box_to_owned(box_pack: BoxPack<'_>) -> BoxPack<'static> {
    BoxPack {
        header: box_pack.header,
        data: crate::data_to_owned(box_pack.data)
    }
}

impl<'d> BoxPack<'d> {
    /// Creates a new Box.
    pub fn new(data: Data<'d>, data_type: impl Into<u16>, queue: u8) -> Self {
        Self {
            header: Header::build_for_data(&data, data_type, queue),
            data,
        }
    }

    // todo: find a better alternative for these 2 "write" functions

    pub fn write_sync<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.header.into_array())?;
        writer.write_all(self.data.as_ref())
    }

    #[cfg(feature = "async")]
    pub async fn write_async<W: tokio::io::AsyncWriteExt + Unpin>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.header.into_array()).await?;
        writer.write_all(self.data.as_ref()).await
    }
}
