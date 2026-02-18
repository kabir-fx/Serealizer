use crate::CustomSerializer;
use std::marker::PhantomData;

pub struct Storage<DataType, SerializationType> {
    data_type: PhantomData<DataType>,
    storage_type: SerializationType,
    serialized_data: Option<Vec<u8>>,
}

impl<DataType, SerializationType> Storage<DataType, SerializationType> {
    pub fn new(serializer: SerializationType) -> Self {
        Self {
            data_type: PhantomData,
            storage_type: serializer,
            serialized_data: None,
        }
    }

    pub fn save<ErrorType, UserProvidedStruct>(&mut self, value: UserProvidedStruct)
    where
        SerializationType: CustomSerializer<UserProvidedStruct, ErrorType>,
        ErrorType: std::fmt::Debug,
    {
        self.serialized_data = Some(self.storage_type.convert_to_bytes(value).expect("failed something"));
    }

    pub fn load<ErrorType, UserProvidedStruct>(&self) -> UserProvidedStruct
    where
        SerializationType: CustomSerializer<UserProvidedStruct, ErrorType>,
        ErrorType: std::fmt::Debug,
    {
        self.storage_type
            .convert_from_bytes(self.serialized_data.clone().expect("Value not present"))
            .expect("Failed to deserialize")
    }

    pub fn has_data(&self) -> bool {
        self.serialized_data.is_some()
    }
}
