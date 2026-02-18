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

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize, SchemaRead, SchemaWrite, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u64,
}

fn main() {
    let mut borsh_storage: Storage<Person, WincodeData> = Storage::new(WincodeData);
    let person = Person {
        name: "Helooo".to_string(),
        age: 334,
    };

    borsh_storage.save(person);
    let loaded: Person = borsh_storage.load();
    println!("Loaded: {:?}", loaded);

    println!("{}", borsh_storage.has_data());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_object() -> Person{
        Person { name: String::from("Marshal"), age: 324 }
    }

    #[test]
    fn test_borsh() {
        let mut borsh_storage: Storage<Person, BorshData> = Storage::new(BorshData);

        let person = generate_test_object();
        borsh_storage.save(person.clone());

        let loaded: Person = borsh_storage.load();

        assert_eq!(person, loaded);
        assert_eq!(true, borsh_storage.has_data());
    }

    #[test]
    fn test_wincode() {
        let mut wincode_storage: Storage<Person, WincodeData> = Storage::new(WincodeData);

        let person = generate_test_object();
        wincode_storage.save(person.clone());

        let loaded: Person = wincode_storage.load();

        assert_eq!(person, loaded);
        assert_eq!(true, wincode_storage.has_data());
    }

    #[test]
    fn test_json() {
        let mut json_storage: Storage<Person, JSONData> = Storage::new(JSONData);

        let person = generate_test_object();
        json_storage.save(person.clone());

        let loaded: Person = json_storage.load();

        assert_eq!(person, loaded);
        assert_eq!(true, json_storage.has_data());
    }
}
