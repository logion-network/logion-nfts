#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Metadata, PSP34Enumerable)]
#[openbrush::contract]
pub mod art_nft_psp34 {
    use logion_contract::impls::logion::*;
    use logion_contract::impls::types as logion;
    use psp34_traits::impls::psp34_traits::*;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ArtNft {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        enumerable: enumerable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        logion: logion::Data,
    }

    impl Logion for ArtNft {}
    impl Psp34Traits for ArtNft {}

    impl ArtNft {
        #[ink(constructor)]
        pub fn new(nonce: String, collection_loc_id: u128, cert_host: String) -> Self {
            let mut instance = Self::default();
            instance.logion.init(nonce, collection_loc_id, cert_host);
            instance
        }
    }
}