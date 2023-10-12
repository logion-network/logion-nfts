use openbrush::traits::String;
use crate::traits::error::Error;
use openbrush::modifiers;

#[openbrush::wrapper]
pub type Psp34TraitsContractRef = dyn Psp34TraitsContract;

#[openbrush::trait_definition]
pub trait Psp34TraitsContract {
    /// This function sets the baseURI for the NFT contract. Only Contract Owner can perform this function. baseURI is the location of the metadata files if the NFT collection use external source to keep their NFT artwork. ArtZero uses IPFS by default, the baseURI can have format like this: ipfs://<hash_ID>/
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
    /// This function return the metadata location of an NFT. The format is baseURI/<token_id>.json
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> String;
}
