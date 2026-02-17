use wincode::{Deserialize, DeserializeOwned, Error, Serialize};
use wincode_derive::{SchemaRead, SchemaWrite};

use crate::CustomSerializer;

#[derive(Debug, PartialEq, SchemaRead, SchemaWrite)]
pub struct WincodeData<T> {
    pub data: T,
}

impl<T: Serialize<Src = T> + DeserializeOwned<Dst = T>> CustomSerializer<WincodeData<T>, Error>
    for WincodeData<T>
{
    fn convert_to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(wincode::serialize(&self)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<WincodeData<T>, Error> {
        Ok(WincodeData::deserialize(&bytes)?)
    }
}
