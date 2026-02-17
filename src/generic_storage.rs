use crate::CustomSerializer;
// use std::marker::PhantomData;

pub struct Storage<SerializationType> {
    data: SerializationType,
    s_data: Option<Vec<u8>>,
}

impl<SerializationType> Storage<SerializationType> {
    pub fn new(serializer: SerializationType) -> Self {
        Self {
            data: serializer,
            s_data: None,
        }
    }

    pub fn save<ErrorType, UserProvidedType>(&mut self, value: UserProvidedType)
    where
        SerializationType: CustomSerializer<UserProvidedType, ErrorType>,
        ErrorType: std::fmt::Debug,
    {
        self.s_data = Some(self.data.convert_to_bytes(value).expect("failed something"));
    }

    pub fn load<ErrorType, UserProvidedType>(&self) -> UserProvidedType
    where
        SerializationType: CustomSerializer<UserProvidedType, ErrorType>,
        ErrorType: std::fmt::Debug,
    {
        self.data
            .convert_from_bytes(self.s_data.clone().expect("Value not present"))
            .expect("Failed to deserialize")
    }

    pub fn has_data(&self) -> bool {
        self.s_data.is_some()
    }
}
