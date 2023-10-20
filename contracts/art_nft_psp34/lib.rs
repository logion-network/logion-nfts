#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
pub mod art_nft_psp34 {
    use logion_contract::impls::logion::*;
    use logion_contract::impls::types as logion;
    use openbrush::contracts::ownable::*;
    use openbrush::modifiers;
    use openbrush::traits::Storage;
    use openbrush::traits::String;
    use psp34_traits::impls::psp34_traits::*;

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

        #[doc = "Mint one NFT for each given account. \n"]
        #[doc = "If t is the total supply at call time, for given accounts[0..n],\n"]
        #[doc = "NFT with id U64(t + 0) is minted to accounts[0],\n"]
        #[doc = "NFT with id U64(t + 1) is minted to accounts[1],\n"]
        #[doc = "...\n"]
        #[doc = "NFT with id U64(t + n) is minted to accounts[n].\n"]
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self, accounts: Vec<AccountId>) -> Result<Vec<(AccountId, Id)>, PSP34Error> {
            let total_supply_after_mint = u64::try_from(PSP34::total_supply(self) + (accounts.len() as u128));
            if total_supply_after_mint.is_err() {
                Err(PSP34Error::Custom(String::from("About to mint too much NFTs, total supply must remain less than u64::MAX")))?
            }
            let mut next_token_index: u64 = PSP34::total_supply(self) as u64;
            let mut token_ids = Vec::new();
            for account in &accounts {
                let token_id = Id::U64(next_token_index);
                psp34::InternalImpl::_mint_to(self, *account, token_id.clone())?;
                next_token_index += 1;
                token_ids.push((*account, token_id));
            };
            Ok(token_ids)
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::test;
        use psp34_traits::traits::error::Error;

        use crate::art_nft_psp34::{PSP34, PSP34Error};

        use super::*;

        const COLLECTION_LOC_ID: u128 = 334801581596596632473758891935041239976;

        #[ink::test]
        fn it_creates_contract_with_default_base_uri() {
            let contract = new_contract();
            assert_eq!(contract.token_uri(0), "undefined0.json".to_string());
        }

        #[ink::test]
        fn it_sets_base_uri() {
            let mut contract = new_contract();
            let result = contract.set_base_uri("http://invalid.domain/nft/".to_string());
            assert_eq!(result, Ok(()));
            assert_eq!(contract.token_uri(0), "http://invalid.domain/nft/0.json".to_string());
        }

        #[ink::test]
        fn it_fails_to_set_base_uri_when_not_contract_owner() {
            let mut contract = new_contract();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let result = contract.set_base_uri("http://invalid.domain/nft/".to_string());
            assert_eq!(result, Err(Error::OwnableError(OwnableError::CallerIsNotOwner)));
        }

        #[ink::test]
        fn it_mints_for_one_account() {
            let mut contract = new_contract();
            let accounts = default_accounts();
            let result = ArtNft::mint(&mut contract, vec![accounts.bob]);
            assert_eq!(result, Ok(vec![(accounts.bob, Id::U64(0))]));
            assert_eq!(PSP34::owner_of(&contract, Id::U64(0)), Some(accounts.bob));
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.bob, 0), Ok(Id::U64(0)));
            assert_eq!(PSP34::balance_of(&contract, accounts.bob), 1);
        }

        #[ink::test]
        fn it_mints_for_many_accounts() {
            let mut contract = new_contract();
            let accounts = default_accounts();
            let result = ArtNft::mint(&mut contract, vec![
                accounts.bob,
                accounts.alice,
                accounts.charlie,
            ]);
            assert_eq!(result, Ok(vec![
                (accounts.bob, Id::U64(0)),
                (accounts.alice, Id::U64(1)),
                (accounts.charlie, Id::U64(2)),
            ]));
            assert_eq!(PSP34::owner_of(&contract, Id::U64(0)), Some(accounts.bob));
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.bob, 0), Ok(Id::U64(0)));
            assert_eq!(PSP34::balance_of(&contract, accounts.bob), 1);
        }

        #[ink::test]
        fn it_fails_to_mint_when_not_contract_owner() {
            let mut contract = new_contract();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let result = ArtNft::mint(&mut contract, vec![accounts.bob]);
            assert_eq!(result, Err(PSP34Error::Custom("O::CallerIsNotOwner".to_string())));
            assert_eq!(PSP34::owner_of(&contract, Id::U64(0)), None);
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.bob, 0), Err(PSP34Error::TokenNotExists));
            assert_eq!(PSP34::balance_of(&contract, accounts.bob), 0);
        }

        #[ink::test]
        fn it_transfers() {
            let mut contract = new_contract();
            let accounts = default_accounts();
            // Alice mints to Bob
            let token_id = Id::U64(0);
            let result = ArtNft::mint(&mut contract, vec![accounts.bob]);
            assert_eq!(result, Ok(vec![(accounts.bob, token_id.clone())]));
            // Bob transfers to Charlie
            set_sender(accounts.bob);
            let result = PSP34::transfer(&mut contract, accounts.charlie, token_id.clone(), Vec::new());
            assert_eq!(result, Ok(()));
            assert_eq!(PSP34::owner_of(&contract, token_id.clone()), Some(accounts.charlie));
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.charlie, 0), Ok(token_id));
            assert_eq!(PSP34::balance_of(&contract, accounts.charlie), 1);
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.bob, 0), Err(PSP34Error::TokenNotExists));
            assert_eq!(PSP34::balance_of(&contract, accounts.bob), 0);
        }

        #[ink::test]
        fn it_fails_to_steal_token() {
            let mut contract = new_contract();
            let accounts = default_accounts();
            // Alice mints to Bob
            let token_id = Id::U64(0);
            let result = ArtNft::mint(&mut contract, vec![accounts.bob]);
            assert_eq!(result, Ok(vec![(accounts.bob, token_id.clone())]));
            // Charlie transfers to Charlie
            set_sender(accounts.charlie);
            let result = PSP34::transfer(&mut contract, accounts.charlie, token_id.clone(), Vec::new());
            assert_eq!(result, Err(PSP34Error::NotApproved));
            assert_eq!(PSP34::owner_of(&contract, token_id.clone()), Some(accounts.bob));
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.bob, 0), Ok(token_id));
            assert_eq!(PSP34::balance_of(&contract, accounts.bob), 1);
            assert_eq!(PSP34Enumerable::owners_token_by_index(&contract, accounts.charlie, 0), Err(PSP34Error::TokenNotExists));
            assert_eq!(PSP34::balance_of(&contract, accounts.charlie), 0);
        }

        fn new_contract() -> ArtNft {
            ArtNft::new(
                "202210131727".to_string(),
                COLLECTION_LOC_ID,
                "certificate.logion.network".to_string(),
            )
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            test::set_caller::<Environment>(sender);
        }
    }
}
