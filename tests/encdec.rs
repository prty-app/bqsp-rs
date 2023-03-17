use bqsp::*;

static PAYLOAD: &'static [u8; 4] = b"BQSP";
static HEADER_SAMPLE: [u8; Header::SIZE] = [4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 5];

#[repr(u16)]
enum Type {
    String = 1,
}

#[test]
fn box_encoding() {
    let data = Data::from(&PAYLOAD[..]);

    assert_eq!(data.as_ref(), PAYLOAD);

    let box_pack = BoxPack::new(
        data,
        Type::String as u16,
        5
    );

    assert_eq!(box_pack.header.get_data_size(), 4);
    assert_eq!(box_pack.header.get_data_type(), 1);
    assert_eq!(box_pack.header.get_data_queue(), 5);
}

#[test]
fn header_decoding() {
    let header = Header::from_array(HEADER_SAMPLE);

    assert_eq!(header.get_data_size(), 4);
    assert_eq!(header.get_data_type(), 1);
    assert_eq!(header.get_data_queue(), 5);
}
