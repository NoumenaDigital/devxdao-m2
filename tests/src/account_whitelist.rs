use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_ACCOUNT_PUBLIC_KEY, DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_types::account::AccountHash;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs};

use crate::utility::constants::{
    ACCOUNT_USER_2, ARG_ACCOUNT_WHITELIST, ENTRY_POINT_ADD_TO_ACCOUNT_WHITELIST,
    ENTRY_POINT_REMOVE_FROM_ACCOUNT_WHITELIST,
};
use crate::utility::{
    constants::{
        ACCOUNT_USER_1, CONTRACT_NAME, NFT_CONTRACT_WASM, NFT_TEST_COLLECTION, NFT_TEST_SYMBOL,
    },
    installer_request_builder::InstallerRequestBuilder,
    support,
};

#[test]
fn should_add_account_to_whitelist() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let (_, public_key_1) = support::create_dummy_key_pair(ACCOUNT_USER_1);
    let (_, public_key_2) = support::create_dummy_key_pair(ACCOUNT_USER_2);

    let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
        .with_collection_name(NFT_TEST_COLLECTION.to_string())
        .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
        .with_total_token_supply(1u64)
        .with_account_whitelist(vec![
            DEFAULT_ACCOUNT_PUBLIC_KEY.clone().to_account_hash(),
            public_key_1.to_account_hash(),
        ])
        .build();

    builder.exec(install_request).expect_success().commit();

    let account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_hash = account
        .named_keys()
        .get(CONTRACT_NAME)
        .cloned()
        .and_then(Key::into_hash)
        .map(ContractHash::new)
        .expect("failed to find nft contract");

    let before: Vec<AccountHash> = builder.get_value(nft_contract_hash, "account_whitelist");
    assert_eq!(before.len(), 2);

    let installer_set_variables_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        ENTRY_POINT_ADD_TO_ACCOUNT_WHITELIST,
        runtime_args! { ARG_ACCOUNT_WHITELIST =>
            Key::Account(public_key_2.to_account_hash())
        },
    )
    .build();

    builder
        .exec(installer_set_variables_request)
        .expect_success()
        .commit();

    let after: Vec<AccountHash> = builder.get_value(nft_contract_hash, "account_whitelist");
    assert_eq!(after.len(), 3);
}

#[test]
fn should_remove_account_from_whitelist() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let (_, public_key_1) = support::create_dummy_key_pair(ACCOUNT_USER_1);

    let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
        .with_collection_name(NFT_TEST_COLLECTION.to_string())
        .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
        .with_total_token_supply(1u64)
        .with_account_whitelist(vec![
            DEFAULT_ACCOUNT_PUBLIC_KEY.clone().to_account_hash(),
            public_key_1.to_account_hash(),
        ])
        .build();

    builder.exec(install_request).expect_success().commit();

    let account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_hash = account
        .named_keys()
        .get(CONTRACT_NAME)
        .cloned()
        .and_then(Key::into_hash)
        .map(ContractHash::new)
        .expect("failed to find nft contract");

    let before: Vec<AccountHash> = builder.get_value(nft_contract_hash, "account_whitelist");
    assert_eq!(before.len(), 2);

    let installer_set_variables_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        ENTRY_POINT_REMOVE_FROM_ACCOUNT_WHITELIST,
        runtime_args! { ARG_ACCOUNT_WHITELIST =>
            Key::Account(public_key_1.to_account_hash())
        },
    )
    .build();

    builder
        .exec(installer_set_variables_request)
        .expect_success()
        .commit();

    let after: Vec<AccountHash> = builder.get_value(nft_contract_hash, "account_whitelist");
    assert_eq!(after.len(), 1);
}
