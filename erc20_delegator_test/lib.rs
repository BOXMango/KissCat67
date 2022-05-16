#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20_delegator_test {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
        // Vec as StorageVec,
    };
    // use ink_prelude::string::String;
    use erc20::Erc20;
    use erc20_test::Erc20Test;

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOTemplate {
        owner: AccountId,
        erc20_code_hash: Hash,
        erc20_test_code_hash: Hash,
    }

    /// Indicates whether a transaction is already confirmed or needs further confirmations.
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInstance {
        owner: AccountId,
        erc20: Erc20,
        erc20_test: Erc20Test,
    }

    #[ink(storage)]
    pub struct Erc20DelegatorTest {
        owner: AccountId,
        template_index: u64,
        template_map: StorageHashMap<u64, DAOTemplate>,
        instance_index: u64,
        instance_map: StorageHashMap<u64, DAOInstance>,
    }

    impl Erc20DelegatorTest {
        #[ink(constructor)]
        pub fn new(controller: AccountId) -> Self {
            let instance = Self {
                owner: controller,
                template_index: 0,
                template_map: StorageHashMap::new(),
                instance_index: 0,
                instance_map: StorageHashMap::new(),
            };
            instance
        }
     #[ink(message)]
        pub fn add_template(&mut self, erc20_code_hash: Hash, erc20_test_code_hash: Hash) -> bool {
            assert_eq!(self.template_index + 1 > self.template_index, true);
            let from = self.env().caller();
            
            self.template_map.insert(self.template_index, DAOTemplate {
                owner: from,
                erc20_code_hash,
                erc20_test_code_hash,
            });
            self.template_index += 1;
            true
        }

    }
}
