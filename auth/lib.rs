#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::auth::Auth;

#[ink::contract]
mod auth {


    use alloc::string::String;
    use alloc::vec::Vec;
    
    // #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };


    type ActionId = u32;

    #[derive(scale::Encode,scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct Action {
        action_id: ActionId,
        action_title: String,
        contract_name: String,
        function_name: String,
    }

    #[ink(storage)]
    pub struct Auth {
        owner: AccountId,
        action_id: ActionId,
        actions: StorageHashMap<(String, String),Action>,
        actions_auths: StorageHashMap<(AccountId, ActionId), Action>,
    }

    impl Auth {

        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                action_id: 0,
                actions: StorageHashMap::new(),
                actions_auths: StorageHashMap::new(),
            }
        }
        
        
    }

   

}
