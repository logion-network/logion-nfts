#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
pub mod art_nft_psp34 {
    use logion_contract::impls::logion::*;
    use logion_contract::impls::types as logion;
    use psp34_traits::impls::psp34_traits::*;
    use openbrush::traits::Storage;
    use openbrush::traits::String;
    use openbrush::modifiers;
    use openbrush::contracts::ownable::*;

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
        ownable: ownable::Data,
        #[storage_field]
        logion: logion::Data,
    }


    #[overrider(PSP34Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
        psp34::InternalImpl::_mint_to(self, account, id)
    }

    impl Logion for ArtNft {}
    impl Psp34Traits for ArtNft {}

    impl ArtNft {

        #[ink(constructor)]
        pub fn new(nonce: String, collection_loc_id: u128, cert_host: String) -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance.logion.init(nonce, collection_loc_id, cert_host);
            match instance.set_base_uri(String::from("undefined")) {
                Ok(_) => instance,
                _ => panic!("Failed to set default base_uri")
            }
        }
    }
}