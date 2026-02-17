use std::io::Error;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::CustomSerializer;

#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BorshData {
    pub name: String,
}

impl CustomSerializer<BorshData, Error> for BorshData {
    fn convert_to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(borsh::to_vec(&self)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<BorshData, Error> {
        Ok(BorshData::try_from_slice(&bytes)?)
    }
}
