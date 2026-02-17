use crate::{borsh::BorshData, json::JSONData, wincode::WincodeData};

pub mod borsh;
pub mod json;
pub mod wincode;

trait CustomSerializer<T, E> {
    // converts data to bytes
    fn convert_to_bytes(&self) -> Result<Vec<u8>, E>;

    // converts bytes back to data
    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<T, E>;
}

fn main() {
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>.");

    let person = BorshData { data: 545 };
    let bytes = person.convert_to_bytes().expect("Failed to serialise");
    println!("Bytes: {:?}", bytes);
    println!(
        "Bytes: {:?}",
        person
            .convert_from_bytes(bytes)
            .expect("Failed to deserialize")
    );

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>.");

    let person = WincodeData { data: 545 };
    let bytes = person.convert_to_bytes().expect("Failed to serialise");
    println!("Bytes: {:?}", bytes);
    println!(
        "Bytes: {:?}",
        person
            .convert_from_bytes(bytes)
            .expect("Failed to deserialize")
    );

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>.");
    let person = JSONData { data: 545 };
    let bytes = person.convert_to_bytes().expect("Failed to serialise");
    println!("Bytes: {:?}", bytes);
    println!(
        "Bytes: {:?}",
        person
            .convert_from_bytes(bytes)
            .expect("Failed to deserialize")
    );

    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>.");
}
