#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::erc20::{
    Erc20,
};

#[ink::contract]
mod erc20 {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout },

    };

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        name: String,
        symbol: String,
        total_supply: u64,
        decimals:u8,
        owner:AccountId,
        balances:StorageHashMap<AccountId, u64>,
        allowances:StorageHashMap<(AccountId, AccountId), u64>,
    }

    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value:u64, 
    }

    #[ink(event)]
    pub struct Approval{
        #[ink(topic)]
        owner:AccountId,
        #[ink(topic)]
        spender:AccountId,
        #[ink(topic)]
        value:u64,
    }

    
    #[derive(scale::Encode,scale::Decode,Clone,SpreadLayout,PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo,ink_storage::traits::StorageLayout)
    )]

    pub struct DisplayInfo {
        name:String,
        symbol:String,
        total_supply:u64,
        decimals:u8,
        owner:AccountId,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(name:String ,symbol:String ,initial_supply:u64, decimals:u8, controller: AccountId) -> Self {
            let balances = StorageHashMap::new();
            // let mut voteholders = StorageHashMap::new();
            let mut instance = Self {
                name: name,
                symbol:symbol,
                total_supply:0,
                decimals,
                owner:controller,
                balances,
                // voteholders,
                allowances: StorageHashMap::new(),
            };
            instance._mint_token(controller,initial_supply);
            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(),Default::default(),Default::default(),Default::default(),Default::default())
        }


        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        } 
        #[ink(message)]
        pub fn total_supply(&self) -> u64 {
            self.total_supply
        }
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals
        }
        #[ink(message)]
        pub fn owner(&self) -> AccountId{
            self.owner
        }

        #[ink(message)]
        pub fn query_info(&self) -> DisplayInfo {
            DisplayInfo{
                name:self.name.clone(),
                symbol:self.symbol.clone(),
                total_supply:self.total_supply,
                decimals:self.decimals,
                owner:self.owner
            }
        }

        #[ink(message, payable)]
        pub fn into_info(&mut self,amount: u64) -> bool {
            
            true
        }

        #[ink(message)]
        pub fn balance_of(&self, owner:AccountId) -> u64 {
            self.balance_of_or_zero(&owner)
        }
        #[ink(message)]
        pub fn allowance(&self, owner:AccountId, spender:AccountId) ->u64 {
            self.allowance_of_or_zero(&owner, &spender)
        }
       
    }
