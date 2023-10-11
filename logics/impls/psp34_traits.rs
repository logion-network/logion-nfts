use ink::prelude::{
    string::{
        String,
        ToString,
    },
};
use openbrush::{
    contracts::psp34::extensions::{
        metadata::{Internal, PSP34Impl, PSP34MetadataImpl},
    },
};

use crate::traits::error::Error;

#[openbrush::trait_definition]
pub trait Psp34Traits: PSP34Impl + PSP34MetadataImpl + Internal {

    /// Change baseURI
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
        let collection_id = PSP34Impl::collection_id(self);
        self._set_attribute(collection_id, String::from("baseURI"), uri);
        Ok(())
    }

    /// Get URI from token ID
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> String {
        let collection_id = PSP34Impl::collection_id(self);
        let value = self.get_attribute(collection_id, String::from("baseURI"));
        let mut token_uri = String::from_utf8(value.unwrap().into()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &String::from(".json");
        token_uri
    }

}
