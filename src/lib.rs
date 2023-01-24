use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{
    env, ext_contract, require, near_bindgen, PanicOnDefault, AccountId, Promise, PromiseResult, Gas, PublicKey,
};

/// Gas attached to the callback from account creation.
pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: Gas = Gas(20_000_000_000_000);

/// Keypom Args struct to be sent to external contracts
#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomArgs {
    pub account_id_field: Option<String>,
    pub drop_id_field: Option<String>,
    pub key_id_field: Option<String>,
    pub funder_id_field: Option<String>,
}

#[ext_contract(ext_self)]
pub trait ExtLinkDrop {
    /// Callback after plain account creation.
    fn on_account_created(&mut self, predecessor_account_id: AccountId, amount: U128) -> bool;
}

fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct AccountFactory {
    pub owner_id: AccountId,
    pub approved_creators: LookupSet<AccountId>,
    pub approved_funders: LookupSet<AccountId>,
}

#[near_bindgen]
impl AccountFactory {
    
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            approved_creators: LookupSet::new(b"c"),
            approved_funders: LookupSet::new(b"f")
        }
    }

    #[payable]
    pub fn create_account(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey,
        funder_id: AccountId,
        keypom_args: KeypomArgs
    ) -> Promise {
        require!(
            self.approved_creators.contains(&env::predecessor_account_id())
        );

        // Ensure the incoming args are correct from Keypom
        require!(keypom_args.funder_id_field.unwrap() == "funder_id".to_string());
        require!(self.approved_funders.contains(&funder_id));

        let amount = env::attached_deposit();
        Promise::new(new_account_id)
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(amount)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(ON_CREATE_ACCOUNT_CALLBACK_GAS)
                    .on_account_created(
                        env::predecessor_account_id(),
                        amount.into()
                    )
            )
    }

    /// Callback after executing `create_account`.
    pub fn on_account_created(&mut self, predecessor_account_id: AccountId, amount: U128) -> bool {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Callback can only be called from the contract"
        );
        let creation_succeeded = is_promise_success();
        if !creation_succeeded {
            // In case of failure, send funds back.
            Promise::new(predecessor_account_id).transfer(amount.into());
        }
        creation_succeeded
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
