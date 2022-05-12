#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;
pub use self::org::OrgManager;

#[ink::contract]
mod org {
    use alloc::string::String;
    use ink_prelude::collections::BTreeMap;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };


    use auth::Auth;
    #[ink(storage)]
    pub struct OrgManager {

        moderators: StorageHashMap<AccountId, String>,
        members: StorageHashMap<AccountId, String>,
        applying_members: StorageHashMap<AccountId, String>,
        owner: AccountId,
        org_id:u64,
        can_free_add_member: bool,
        is_member: bool,
        is_moderator:bool,
        is_owner:bool,
        auth_contract_address:AccountId,
    }


    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        DaoModeratorExistAlready,
        DaoMemberExistAlready,
        DaoMemberNotExist,
        DaoModeratorNotExist,
    }

    #[ink(event)]
    pub struct AddDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        org_id:u64,
    }

    #[ink(event)]
    pub struct RemoveDAOModeratorEvent {
        #[ink(topic)]
        moderator: AccountId,
        #[ink(topic)]
        org_id:u64,
    }


    #[ink(event)]
    pub struct AddDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
    }

    #[ink(event)]
    pub struct RemoveDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
    }



    #[ink(event)]
    pub struct ApplyDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
    }

    #[ink(event)]
    pub struct ApproveDAOMemberEvent {
        #[ink(topic)]
        member: AccountId,
        #[ink(topic)]
        org_id:u64,
        #[ink(topic)]
        approver: AccountId,
    }


    impl OrgManager {

        

    }

        
}
