use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use wincode_derive::{SchemaRead, SchemaWrite};

use crate::{borsh_type::BorshData, generic_storage::Storage, json::JSONData, wincode::WincodeData};

pub mod borsh_type;
pub mod generic_storage;
pub mod json;
pub mod wincode;

pub trait CustomSerializer<T, E> {
    // converts data to bytes
    fn convert_to_bytes(&self, data: T) -> Result<Vec<u8>, E>;

    // converts bytes back to data
    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<T, E>;
}

#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize, SchemaRead, SchemaWrite, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u64,
}

fn main() {
    let mut borsh_storage = Storage::new(WincodeData);
    let person = Person {
        name: "Helooo".to_string(),
        age: 334,
    };

    borsh_storage.save(person);
    let loaded: Person = borsh_storage.load();
    println!("Loaded: {:?}", loaded);

    println!("{}", borsh_storage.has_data());
}
