use wincode::{DeserializeOwned, Error, Serialize};
use wincode_derive::{SchemaRead, SchemaWrite};

use crate::CustomSerializer;

#[derive(Debug, PartialEq, SchemaRead, SchemaWrite)]
pub struct WincodeData;

impl<T: Serialize<Src = T> + DeserializeOwned<Dst = T>> CustomSerializer<T, Error> for WincodeData {
    fn convert_to_bytes(&self, data: T) -> Result<Vec<u8>, Error> {
        Ok(wincode::serialize(&data)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<T, Error> {
        Ok(wincode::deserialize(&bytes)?)
    }
}
