use wincode::{Deserialize, Error};
use wincode_derive::{SchemaRead, SchemaWrite};

use crate::CustomSerializer;

#[derive(Debug, PartialEq, SchemaRead, SchemaWrite)]
pub struct WincodeData {
    pub name: String,
}

impl CustomSerializer<WincodeData, Error> for WincodeData {
    fn convert_to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(wincode::serialize(&self)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<WincodeData, Error> {
        Ok(WincodeData::deserialize(&bytes)?)
    }
}
