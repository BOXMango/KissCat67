#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::base::Base;

#[ink::contract]
mod base {

    use alloc::string::String;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DisplayBase {
        creator: AccountId,
        name: String,
        logo: String,
        desc: String,
    }
    
    #[ink(storage)]
    pub struct Base {
        creator: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    impl Base {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                name: String::default(),
                logo: String::default(),
                desc: String::default(),
                creator: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }
    }
}
