use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::CustomSerializer;

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONData {
    pub name: String,
}

impl CustomSerializer<JSONData, Error> for JSONData {
    fn convert_to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(&self)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<JSONData, Error> {
        let back_to_txt: JSONData = serde_json::from_slice(&bytes)?;

        Ok(back_to_txt)
    }
}
