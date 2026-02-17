use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Error;

use crate::CustomSerializer;

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONData;

impl<T: Serialize + DeserializeOwned> CustomSerializer<T, Error> for JSONData {
    fn convert_to_bytes(&self, data: T) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(&data)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<T, Error> {
        let back_to_txt: T = serde_json::from_slice(&bytes)?;

        Ok(back_to_txt)
    }
}
