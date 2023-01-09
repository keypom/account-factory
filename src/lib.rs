use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet};
use near_sdk::{
    env, require, near_bindgen, PanicOnDefault, AccountId, Promise, PublicKey,
};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct AccountFactory {
    pub owner_id: AccountId,
    pub approved_creators: LookupSet<AccountId>,
}

#[near_bindgen]
impl AccountFactory {
    
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            approved_creators: LookupSet::new(b"c"),
        }
    }

    #[payable]
    pub fn create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
    ) -> Promise {
        assert!(
            env::is_valid_account_id(new_account_id.as_bytes()),
            "Invalid account id"
        );
        require!(
            self.approved_creators.contains(&env::predecessor_account_id())
        );
        let amount = env::attached_deposit();
        Promise::new(new_account_id)
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(amount)
    }

    /// approved creators mgmt
    pub fn add_approved_creator(&mut self, account_id: AccountId) {
        self.assert_contract_owner();
        self.approved_creators.insert(&account_id);
    }

    pub fn remove_approved_creator(&mut self, account_id: AccountId) {
        self.assert_contract_owner();
        self.approved_creators.remove(&account_id);
    }

    pub fn is_approved_creator(&self, account_id: AccountId) -> bool {
        self.approved_creators.contains(&account_id)
    }

    fn assert_contract_owner(&mut self) {
        assert!(
            self.owner_id == env::predecessor_account_id(),
            "only contract owner"
        )
    }
}
