#![allow(dead_code)]

use std::num::ParseIntError;
use std::str::FromStr;
use std::string::FromUtf8Error;
use bqsp::*;
use deserializer::BoxDeserializer;

#[repr(u16)]
/// Represents which header data type number corresponds to which Payload variant
/// # Example
/// `number "1" means that the box data is a "Number" Payload variant`
enum DataType {
    Message = 0,
    Number = 1,
    User = 2,
}

impl TryFrom<u16> for DataType {
    type Error = DesError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => DataType::Message,
            1 => DataType::Number,
            2 => DataType::User,
            _ => return Err(DesError::InvalidType),
        })
    }
}

#[derive(Debug)]
enum Payload {
    Message(String),
    User {
        name: String,
        surname: String,
        age: u8,
    },
    Number(u32),
}

#[derive(Debug)]
enum DesError {
    InvalidType,
    MessageNotValidUTF8(FromUtf8Error),
    NumberNotValidSize,
    UserInvalidStructure,
    UserIO(std::io::Error),
    UserNotValidUTF8(FromUtf8Error),
    UserNotValidAge(ParseIntError),
}

impl BoxDeserializer for Payload {
    type Error = DesError;

    fn deserialize_box(data: BoxPack) -> Result<Self, Self::Error> where Self: Sized {
        let box_type = DataType::try_from(data.header.get_data_type())?;

        let payload = match box_type {
            DataType::Message => {
                let bytes = data.data.as_ref();
                match String::from_utf8(bytes.to_vec()) {
                    Ok(string) => Payload::Message(string),
                    Err(error) => return Err(DesError::MessageNotValidUTF8(error)),
                }
            }
            DataType::Number => {
                let mut buffer = [0u8; std::mem::size_of::<u32>()];
                if buffer.len() != data.data.len() {
                    return Err(DesError::NumberNotValidSize);
                }

                buffer.copy_from_slice(data.data.as_ref());

                Payload::Number(u32::from_be_bytes(buffer))
            }
            DataType::User => {
                let bytes = data.data.as_ref();
                let mut split = bytes.split(|byte| *byte == 0u8);

                let name = split
                    .next()
                    .ok_or(DesError::UserInvalidStructure)?
                    .to_vec();

                let surname = split
                    .next()
                    .ok_or(DesError::UserInvalidStructure)?
                    .to_vec();

                let age = split
                    .next()
                    .ok_or(DesError::UserInvalidStructure)?
                    .to_vec();

                let name = String::from_utf8(name)
                    .map_err(|error| DesError::UserNotValidUTF8(error))?;

                let surname = String::from_utf8(surname)
                    .map_err(|error| DesError::UserNotValidUTF8(error))?;

                let age = String::from_utf8(age)
                    .map_err(|error| DesError::UserNotValidUTF8(error))?;

                let age = u8::from_str(&age)
                    .map_err(|error| DesError::UserNotValidAge(error))?;

                Payload::User {
                    name,
                    surname,
                    age,
                }
            }
        };

        Ok(payload)
    }
}

#[test]
fn invalid_type() {
    let box_pack = BoxPack::new(
        Data::from(&b"BQSP"[..]),
        u16::MAX,
        1
    );

    let des_error = BoxDes::<Payload>::try_from(box_pack).unwrap_err();

    match des_error {
        DesError::InvalidType => {}
        error => panic!("Invalid error: {:?}", error),
    }
}

#[test]
fn message() {
    let string = "Hello World!";

    let box_pack = BoxPack::new(
        Data::from(string.as_bytes()),
        DataType::Message as u16,
        1
    );

    let box_des = BoxDes::<Payload>::try_from(box_pack).unwrap();

    match box_des.data {
        Payload::Message(s) => assert_eq!(s, string),
        _ => panic!("Invalid payload."),
    }
}

#[test]
fn number() {
    let number = 62341_u32;
    let number_bytes = number.to_be_bytes();

    let box_pack = BoxPack::new(
        Data::from(&number_bytes[..]),
        DataType::Number as u16,
        1
    );

    let box_des = BoxDes::<Payload>::try_from(box_pack).unwrap();

    match box_des.data {
        Payload::Number(n) => assert_eq!(n, number),
        _ => panic!("Invalid payload."),
    }
}

#[test]
fn user() {
    let user = b"Franek\0Ganek\024";

    let box_pack = BoxPack::new(
        Data::from(user.as_slice()),
        DataType::User as u16,
        1
    );

    let box_des = BoxDes::<Payload>::try_from(box_pack).unwrap();

    match box_des.data {
        Payload::User {
            name,
            surname,
            age,
        } => {
            assert_eq!(name, "Franek");
            assert_eq!(surname, "Ganek");
            assert_eq!(age, 24);
        }
        _ => panic!("Invalid payload."),
    }
}
