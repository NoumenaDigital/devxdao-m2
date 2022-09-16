# CEP-78 with account whitelisting (modified, enhanced NFT)
This contract is a modified version of Casper's existing `CEP-78 enhanced NFT` contract. The original can be found here:
https://github.com/casper-ecosystem/cep-78-enhanced-nft

## Table of Contents
1. Background Information
   1. Context
   2. Summary of Core Requirements
   3. Why CEP-78?
   4. CEP-78 Changes and Parametrisation
   5. Fulfillment of Core Requirements
2. Code Documentation
   1. Purpose
   2. Modifications
   3. Building and testing on the local environment
   4. Manual Testing


## Background Information

### Context

In the scope of this project, a prototype of a real estate investment platform that enables democratization and decentralization
of real estate investments is being implemented. This platform utilizes off-chain smart contracts implemented in the Noumena Protocol
Language (NPL) to handle complex business logic and sensitive data. These off-chain smart-contracts are used to orchestrate
on-chain smart contracts which control actions on the Casper blockchain itself e.g. mint, burn, transfer of tokens representing
the real estate assets, etc.

Milestone 2 aims at creating tokens representing ownership shares of the real estate investment special purpose vehicles (SPVs),
that a) fulfill the business requirements of the involved parties and b) comply with the Swiss DLT law.
This was accomplished by adding functionality to the newly created CEP-78 enhanced NFT standard on the Casper blockchain. It is important
to note, however, that the token is only part of the solution that was developed in order to fulfill these requirements and that
the picture will be completed with the delivery of Milestone 3, 4 and 5.

For a detailed description of the user journeys, please refer to Milestone 1 of this grant. A prototype of the investment platform
will be delivered with Milestone 3 and 4, while a comprehensive white paper on how to utilize smart contracts and tokens for the
digitization and democratization of Real Estate Investments in Switzerland will be delivered with Milestone 5.

### Summary of Core Requirements

The following core requirements have been gathered from a) the involved parties (primarily project developers, investment managers,
banks and investors) and b) the Swiss DLT legislation:

2. #### Obfuscation/Anonymity
   The Token on the blockchain may not hold sensitive information, to ensure the investment and identity related information remains private.
   Most companies for example must not and don't want to disclose their cap table and most investors do not want to disclose their holdings
   or their trades.

3. #### Transfer restrictions
   Legal restrictions with regards to investors such as having to be a professional investor under MIFID II, federal land ownership
   restrictions and other restrictions have to be adhered to.

4. #### Swiss DLT Law
   With the amendment of the law in 2020, Swiss Parliament introduced the register value right (also referred to as Swiss DLT law).
   A register value right must be recorded in a register and a register value right then can only be transferred and enforced via the register.

   The register must meet certain legal requirements and the legislature had DLT systems in mind when it formulated the requirements for the
   register of value. The intention of the DLT law is to make investors as independent as possible from the issuer and any third party.
   In particular they should be able to initiate transactions and look up their holdings directly via the register (DLT) without involvement
   of the issuer and any third party.

   In more detail for a DLT system to be recognized as a register of value, it must meet at least the following four legal requirements
   (Art. 973d para. 2 nOR):
   1. Power of disposal: the holder of the register value right (token holder) must have the power of disposal over his tokens. This
      power of disposition must not only be contractually promised, but also technically guaranteed.
   2. Integrity: The integrity of the stored value register must be protected against unauthorized access by appropriate technical and
      organizational measures against unauthorized changes. As an example, the law mentions the joint management of the register of rights
      to value by several independent parties.
   3. Transparency: the register value right is preceded by an agreement between the parties involved, according to which the register
      right can only be transferred and enforced via the register. The content of the right "securitized" by the register value right
      as well as its features must be clearly formulated and made available to each party either in the DLT system itself or via a link
      that can be called up in the DLT system.
   4. Independent access: the token holder must always be able to access the information and data relating to him without a third party,
      to view and verify the information and register entries relating to him or her.

5. #### Controller action
   A further requirement, specific for tokenized assets, such as shares, is that the issuer of the shares/tokens (contract-installer
   account) may execute specific actions on shares that are held by investors. This allows the controller to perform corporate actions
   (share splits, etc.) on said tokenized assets.

### Why CEP-78?

Initially the project aimed at implementing the real estate token using the ERC-1400 token standard and its concepts
of partitions to obfuscate how many shares of a real estate SPV an investor (account) owns. Partitions are intended to define 
different types of shares, for example ordinary and participation shares. The idea here was, however, to use a partition to represent buckets
of shares, rather than types of shares. This means that a partition becomes equivalent to a token, which represents a
number of shares. The actual number of shares only has to be known by the platform, not the blockchain. It also means, however, that 
various features of the ERC-1400 contract no longer make sense because you cannot specify a number of shares or percentage without 
disclosing some information on the blockchain about how many shares of a real estate SPV an investor (account) owns.

Features that no longer make sense include: balanceOfByPartition, transferWithData, transferFromWithData, transferByPartition,
operatorTransferByPartition, controllerTransfer, controllerRedeem, etc. Basically, any feature that takes an amount of shares
as an argument.

Given that, we found that the best way to maintain privacy is to use a fixed unchangeable token. On creation a number of shares
is assigned to the token via the platform, then on transfer the token must either be transferred as a whole (in case the full 
amount is sold) or burned and two new tokens need to be minted: one transferred to the buyer and the other remaining with the seller (in case a partial amount is sold).

It is also important that tokens should be merged when an account owns more than one token of the same type because otherwise, given
enough trading we tend towards one token representing one share. A way to do this is to use an NFT, and the CEP-78 standard had some 
enhancements on CEP-48 that were useful for us.

While the CEP-47 standard uses `groups`, the CEP-78 standard introduces a new concept of a `whitelist` to control the contracts which
are allowed to mint tokens. While these features were used in very different use cases, there is one fundamental difference. `groups`
control entryPoints and actions based on adding a tag to an account, while whitelists add a list to the given contract, that grants
permissions to the accounts on the list.

Following the existing pattern of the newer CEP-78 standard, the option of using `whitelists`was chosen. However, the standard
whitelist only applied to contracts and the entryPoint of minting. We added an additional whitelist concept to make sure that transfers of
tokens can only succeed for accounts that are whitelisted. The extended `whitelist` concept applies to accounts
and additional entryPoints such as transfer.

### CEP-78 Changes and Parametrisation

This Milestone extends the CEP-78 standard with the following functionality:
- Allow Mint, MetadataUpdate and Burn only for contract-installer account.
- Add whitelist of accounts that tokens can be transferred to.
- Modified transfer method so that it can be invoked by the contract-installer account, given that the target account is on
  the whitelist.

The following list provides a list of the modalities of the CEP-78 standard, the values chosen in this delivery and their rationalization.

| Modality            | Implemented value | Reasoning                                                                                                                                                                                                                                                                               |
|---------------------|-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Ownership_Mode      | Transferable      | A core requirement of this project is to transfer the CEP-78 NFT between investors qualified for the respective Real Estate Investment (SPV)                                                                                                                                            |
| NFTKind             | Virtual           | The tokens are a digital representation of shares representing an ownership stake of a given real estate project                                                                                                                                                                        |
| NFTHolderMode       | Accounts          | The respective NFTs are held by Investor accounts                                                                                                                                                                                                                                       |
| WhitelistMode       | Locked            | This is only applicable to the contract whitelist that manages the minting of tokens and not to the whitelist that manages the accounts which are cleared for token transfers. Since the minting and burning can only be done by the installer account, the whitelist should be locked. |
| Minting             | Installer         | The ability to mint new tokens is limited to the account that installed the contract                                                                                                                                                                                                    |
| NFTMetadataKind     | Custom            | The custom configuration allows for the storing of a predefined json_schema. This schema will ultimately store the protocol references to the off-chain smart contracts on the Noumena platform                                                                                         |
| NFTIdentifierMode   | Ordinal           | While the original method is more human readable, it also allows for an indication of how many NFTs have already been minted                                                                                                                                                            |
| Metadata_Mutability | Immutable         | The metadata carries information such as the external protocol reference. It is thus essential, that it be immutable                                                                                                                                                                    |
| JSON_Schema         | DummyData         | For the purpose of this Milestone, pure dummy data was provided                                                                                                                                                                                                                         |

### Fulfillment of the Core Requirements

With the enhanced implementation of the CEP-78 standard, we believe that the following core requirements needed for this
project/grant can be achieved:

1. #### Obfuscation/Anonymity
   The Token on the blockchain may not hold sensitive information to ensure the investment and identity related information
   remains private. The NFT in form of the enhanced CEP-78 standard ensures this by representing the total number of shares the investor
   owns and stores this information not on the chain itself, but in a privacy preserving off-chain smart contract on the investment
   platform. The token thus guarantees the ownership, while obfuscating the investment and personal data associated with it.

2. #### Transfer restrictions
   Through the enhancement with the whitelist of accounts, it can be ensured that legal restrictions with regards to investors
   such as having to be a professional investor under MIFID II, federal land ownership restrictions and other restrictions are adhered to.

3. #### Swiss DLT Law compliance
   Using the above-mentioned implementation of the CEP-78 standard with the whitelist functionality, it is possible to ensure
   adherence to the Swiss DLT Law for example in initiating a transfer of tokenized assets on the chain without a dependency on third
   party systems (power of disposal) while ensuring adherence to other regulations such as those with regards to qualified investors.
   The CEP-78 standard also allows to store fingerprints and links to independent services that provide detailed asset information
   (transparency) and owned shares (independent access).

4. #### Controller action
   A further requirement, specific for tokenized assets, such as shares, is that the issuer of the shares/tokens (contract-installer
   account) may execute specific actions on shares that are held by investors. This allows the controller to perform corporate actions
   (share splits, etc.) on said tokenized assets.

This milestone thus not only implemented an enhanced version of the CEP-78 standard, but also conducted a deep analysis on the pros
and cons of using NFTs in the form of CEP-78 instead of ERC-1400 tokens on the blockchain to achieve obfuscation/anonymity and adhere
to transfer restrictions, while maintaining the proof of ownership guaranteed through the token.

## Code Documentation

### Purpose
This contract was implemented to facilitate the tokenization of real estate assets. 

### Modifications
The contract extends the CEP-78 standard with the following functionality:
 - Allow `Mint`, `MetadataUpdate` and `Burn` only for contract-installer account. 
 - Add `whitelist` of accounts that tokens can be transferred to.
 - Add `transferFrom` method that can be invoked only by contract-installer account.

### Building and testing on the local environment
Building and testing of the added functionality on a local machine. For detailed instructions on how to deploy and test the 
contract on testnet, see "Manual Testing" below.
```bash
# Set up the Rust toolchain
make prepare
# Compile the smart contract
make build-contract
# Run integration tests
make clean test
```

### Manual Testing

This set of tests covers the extended functionalities of the smart contract: Deployment of the contract, minting, burning and 
transfer of tokens. Several tests are designed to fail, to prove correct functioning of the restrictions imposed by the whitelist 
etc.

Before being able to execute the tests with the smart contract, you will need an adequate test environment, with casper client 
and accounts through Casper Signer, all on the Casper testnet. The first four points (under "Account Setup") cover the setup of 
your environment. The functional test as described above can be found under "Tests" below the account setup.

You will need to create 3 sets of public keys and their Accounts to do this manual testing.  
So, steps 2,3 and 4 below need done 3 times.  So you should have 3 accounts:

| account   | account hash reference          | private key reference                | test account hash                                                 |
|-----------|---------------------------------|--------------------------------------|-------------------------------------------------------------------|
| account 1 | account-hash-\<ACCOUNT_HASH_1\> | \<PATH_TO_ACCOUNT_SECRET_KEY_1\>.pem | d71e6a5b0d1c0a58ce8a82ff72f4191668518861d86ee7f6520c009ced8fb41a  |
| account 2 | account-hash-\<ACCOUNT_HASH_2\> | \<PATH_TO_ACCOUNT_SECRET_KEY_2\>.pem | ac26d47df5d4ecbf4ee9a1c1fac04262b4252e43c578563c03d949cb035cd849  |
| account 3 | account-hash-\<ACCOUNT_HASH_3\> | \<PATH_TO_ACCOUNT_SECRET_KEY_3\>.pem | f4c6b6a8aa440674c291be01266ebf299ed9c91b6a7060f5c32a4c7aa756ae95  |

The accounts with which the whole manual testing described below has been performed can be found in 'test account hash' column.

##### Account Setup
1. Install casper-client: https://docs.casperlabs.io/workflow/setup/#the-casper-command-line-client
2. Create keys: https://docs.casperlabs.io/dapp-dev-guide/keys/#option-1-key-generation-using-the-casper-client (if `tree ed25519-keys/` 
doesn't work use `ls ed25519-keys/`)
3. Get Casper Signer: https://chrome.google.com/webstore/detail/casper-signer/djhndpllfiibmcdbnmaaahkhchcoijce and use the keys 
from “2.” to create account.
4. Go to https://testnet.cspr.live and then to Tools/Faucet in order to get some CSPR.
To verify the account is set up correctly, login to it on https://testnet.cspr.live

##### Tests
0. `First` clone the repo and build the contract locally to produce the deployable wasm file
```bash
  git clone <REPOSITORY_LOCATION>
  make prepare first
  make clean test

```
1. `deploy`
```bash
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 165000000000 \
    --session-path ./contract/target/wasm32-unknown-unknown/release/contract.wasm \
    --session-arg "collection_name:string='devxdao'" \
    --session-arg "collection_symbol:string='DXD'" \
    --session-arg "total_token_supply:u64='1000'" \
    --session-arg "ownership_mode:u8='2'" \
    --session-arg "whitelist_mode:u8='1'" \
    --session-arg "nft_kind:u8='2'" \
    --session-arg "nft_metadata_kind:u8='3'" \
    --session-arg "json_schema:string='{\"properties\":{\"type\":{\"name\":\"type\",\"description\":\"How token is created: INITIAL, SPLIT or MERGED\",\"required\":true}}}'" \
    --session-arg "identifier_mode:u8='0'" \
    --session-arg "metadata_mutability:u8='0'"
    
  # NB:
  # All casper-client commands are expected to return immediately with a json String containing a deploy_hash.  
  # e.g.
  {
      "id": -7201873181009066000,
      "jsonrpc": "2.0",
      "result": {
        "api_version": "1.4.7",
        "deploy_hash": "8bba4460a145e4a9d8e815bd741de9ad4974db2df0cc9d685f455e3d81d38717"
      }
  }
  # The actual deploy takes a few minutes to complete and can be seen in https://testnet.cspr.live under the "Deploys" 
  # tab for the Account corresponding to the secret-key used in the deploy command.
  # A test that's expected to fail will show a failed Deploy.  To see the failure reason, click on the deploy hash of 
  # the failed deploy.
  # `deploy_hash` can be used to check the status of the command with casper-client:
  casper-client get-deploy --node-address http://136.243.187.84:7777 <DEPLOY_HASH>


  # Verify: find your contract hash under the "Named Keys" tab of your account home page on https://testnet.cspr.live, under the Key `nft_contract`.  
  # The value will start with a `hash-` and look like this: `hash-3e83b0854e6a4932e4766ed9e1b1f1751e65668d5864f19bd6529b9ff32d8ab0`
```
2. `mint` token from different account - should fail (with User error 36 (InvalidMinter), because only the contract's installer can mint tokens)
```bash
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_2>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "mint" \
    --session-arg "token_owner:key='account-hash-<ACCOUNT_HASH_1>'" \
    --session-arg "token_meta_data:string='{\"type\":\"INITIAL\"}'"
    
  # Verify: on https://testnet.cspr.live by logging in as account 2,
  # and checking there is a failed `mint` deploy, click the deploy hash and check the User error is 36
  
  # Also: find an account's hash by issuing the command 
  casper-client account-address --public-key <PATH_TO_ACCOUNT_PUBLIC_KEY>.pem
```
3. `mint` a token (contract-installer account)
```bash
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "mint" \
    --session-arg "token_owner:key='account-hash-<ACCOUNT_HASH_1>'" \
    --session-arg "token_meta_data:string='{\"type\":\"INITIAL\"}'"
    
  # Verify: on https://testnet.cspr.live by navigating to the contract's `token_owners` Named Key, 
  # search for token_id `0` and check the account-hash matches "account-hash-<ACCOUNT_HASH_1>"
```
4. `burn` token from different account - should fail (with User error 1 (InvalidAccount), because only the contract's installer can burn tokens)
```bash
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_2>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "burn" \
    --session-arg "token_id:u64='0'"
    
  # Verify: on https://testnet.cspr.live by logging in as account 2,
  # and checking there is a failed `burn` deploy, click the deploy hash and check the User error is 1
```
5. `burn` a token (contract-installer account)
```bash
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "burn" \
    --session-arg "token_id:u64='0'"
    
  # Verify: on https://testnet.cspr.live by navigating to the contract's `burnt_tokens` Named Key, 
  # search for token_id `0` and check it exists in this list
```
6. `transfer/whitelist`: transfer token from contract-installer account to account not on whitelist - should fail
```bash
  # (i) Mint another token
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "mint" \
    --session-arg "token_owner:key='account-hash-<ACCOUNT_HASH_1>'" \
    --session-arg "token_meta_data:string='{\"type\":\"INITIAL\"}'"

  # (ii) Attempt to transfer the token, which should fail (with User error 107 (UnlistedAccountHash), because it's not in the whitelist)
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "transfer" \
    --session-arg "token_id:u64='1'" \
    --session-arg "source_key:key='account-hash-<ACCOUNT_HASH_1>'" \
    --session-arg "target_key:key='account-hash-<ACCOUNT_HASH_2>'"
    
  # Verify: on https://testnet.cspr.live by logging in as account 1,
  # and checking there is a failed `transfer` deploy, click the deploy hash and check the User error is 107
```
7. `transfer/whitelist`: transfer token from contract-installer account to account on whitelist
```bash
  # (i) Add the target account to the whitelist
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "add_to_account_whitelist" \
    --session-arg "account_whitelist:key='account-hash-<ACCOUNT_HASH_2>'"
    
  # Verify: on https://testnet.cspr.live by navigating to the contract's `account_whitelist` Named Key,
  # it should contain <ACCOUNT_HASH_2> (without the `account-hash-` prefix)
  
  # (ii) Transfer token from account 1 to account 2
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "transfer" \
    --session-arg "token_id:u64='1'" \
    --session-arg "source_key:key='account-hash-<ACCOUNT_HASH_1>'" \
    --session-arg "target_key:key='account-hash-<ACCOUNT_HASH_2>'"
    
  # Verify: on https://testnet.cspr.live by navigating to the contract's `token_owners` Named Key, 
  # search for token_id `1` and check the account-hash matches "account-hash-<ACCOUNT_HASH_2>"
```
8. `transfer/whitelist`: token owner (not contract-installer) may transfer token to another account on whitelist
```bash
  # (i) Mint a new token 
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "mint" \
    --session-arg "token_owner:key='account-hash-<ACCOUNT_HASH_2>'" \
    --session-arg "token_meta_data:string='{\"type\":\"INITIAL\"}'"
  # This is the third token we have minted so it's token_id should be 2 (token_id numbering starts at 0)
  
  # (ii) Add target account to the whitelist
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "add_to_account_whitelist" \
    --session-arg "account_whitelist:key='account-hash-<ACCOUNT_HASH_3>'"
    
  # (iii) Transfer token from account-1 to account-2
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_2>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "transfer" \
    --session-arg "token_id:u64='2'" \
    --session-arg "source_key:key='account-hash-<ACCOUNT_HASH_2>'" \
    --session-arg "target_key:key='account-hash-<ACCOUNT_HASH_3>'"
    
  # Verify: on https://testnet.cspr.live by navigating to the contract's `token_owners` Named Key, 
  # search for token_id `2` and check the account-hash matches "account-hash-<ACCOUNT_HASH_3>"
```
9. `transfer/whitelist`: contract-installer may transfer token from one account (not his) to another account both on whitelist
```bash
 
  # Transfer token from account-2 back to account-1
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_1>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "transfer" \
    --session-arg "token_id:u64='2'" \
    --session-arg "source_key:key='account-hash-<ACCOUNT_HASH_3>'" \
    --session-arg "target_key:key='account-hash-<ACCOUNT_HASH_2>'"
    
  # Verify: on https://testnet.cspr.live by navigating to the contract's `token_owners` Named Key, 
  # search for token_id `2` and check the account-hash matches "account-hash-<ACCOUNT_HASH_2>"
```
10. `whitelist`: add account not by contract-installer - should fail (with User error 1 (InvalidAccount), because only 
the contract-installer can add accounts to the whitelist)
```bash
  casper-client put-deploy \
    --node-address http://136.243.187.84:7777 \
    --chain-name casper-test \
    --secret-key <PATH_TO_ACCOUNT_SECRET_KEY_2>.pem \
    --payment-amount 1000000000 \
    --session-hash "hash-<YOUR_CONTRACT_HASH>" \
    --session-entry-point "add_to_account_whitelist" \
    --session-arg "account_whitelist:key='account-hash-<ACCOUNT_HASH_2>'"
    
  # Verify: on https://testnet.cspr.live by logging in as account 2,
  # and checking there is a failed `add_to_account_whitelist` deploy, click the deploy hash and check the User error is 1
```
