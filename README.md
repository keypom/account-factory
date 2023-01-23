# Account Factory Contract

Simplified and owner controlled version of `create_account` from Linkdrop Contract.

## Quick How To / Testing

1. Build the contract locally
2. Login to [YOUR_ACCOUNT_FACTORY_ID] using near-cli
```
near login
```
3. Deploy the wasm `./res/account_factory.wasm`
```
near deploy [YOUR_ACCOUNT_FACTORY_ID] ./res/account_factory.wasm

// init the contract
near call [YOUR_ACCOUNT_FACTORY_ID] new --accountId=[YOUR_ACCOUNT_FACTORY_ID]
```
4. This account is the `owner` which can add `approved_creators`
5. Add an approved creator
```
near call [YOUR_ACCOUNT_FACTORY_ID] add_approved_creator '{"account_id":"[SOME_APPROVED_CREATOR_ACCOUNT_ID]"}' --accountId=[YOUR_ACCOUNT_FACTORY_ID]
```
6. Login to [SOME_APPROVED_CREATOR_ACCOUNT_ID] using near-cli
```
near login
```
7. Create an account from the approved creator account
```
near call [YOUR_ACCOUNT_FACTORY_ID] create_account '{"new_account_id":"foo.testnet","new_public_key":"ed25519:BonsbmfRMRNzRwn92827kfGCwkNLjNDritF1LwrbZKn2"}' --deposit=1 --accountId=[SOME_APPROVED_CREATOR_ACCOUNT_ID]
```
