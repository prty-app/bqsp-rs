use bqsp::{BoxPack, Data, Header};

#[test]
fn byte_repr() {
    let box_pack = BoxPack::new(
        Data::from([0x00_u8; 16].as_slice()),
        1u16,
        1,
    );

    let header_array: [u8; Header::SIZE] = box_pack.header.into_array();
    let header_array_expected: [u8; Header::SIZE] = [16, 0, 0, 0, 1, 0, 1];
    assert_eq!(
        header_array,
        header_array_expected,
        "Header encoding mismatch!",
    );

    let header_decoded = Header::from_array(header_array_expected);
    assert_eq!(
        box_pack.header,
        header_decoded,
        "Header decoding mismatch!",
    );
}
