#![allow(dead_code)]

use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use casper_types::{
    account::AccountHash, bytesrepr::Bytes, runtime_args, CLValue, ContractHash, Key, RuntimeArgs,
};

pub mod consts {
    pub(crate) const NFT_CONTRACT_WASM: &str = "contract.wasm";
    pub(crate) const MINT_SESSION_WASM: &str = "mint_call.wasm";
    pub(crate) const MINTING_CONTRACT_WASM: &str = "minting_contract.wasm";
    pub(crate) const TRANSFER_SESSION_WASM: &str = "transfer_call.wasm";
    pub(crate) const BALANCE_OF_SESSION_WASM: &str = "balance_of_call.wasm";
    pub(crate) const OWNER_OF_SESSION_WASM: &str = "owner_of_call.wasm";
    pub(crate) const GET_APPROVED_WASM: &str = "get_approved_call.wasm";
    pub(crate) const UPDATED_RECEIPTS_WASM: &str = "updated_receipts.wasm";
    pub(crate) const MANGLE_NAMED_KEYS: &str = "mangle_named_keys.wasm";
    pub(crate) const CONTRACT_NAME: &str = "cep78_contract_hash_nft-test";
    pub(crate) const MINTING_CONTRACT_NAME: &str = "minting_contract_hash";
    pub(crate) const NFT_TEST_COLLECTION: &str = "nft-test";
    pub(crate) const NFT_TEST_SYMBOL: &str = "TEST";
    pub(crate) const ENTRY_POINT_INIT: &str = "init";
    pub(crate) const ENTRY_POINT_SET_VARIABLES: &str = "set_variables";
    pub(crate) const ENTRY_POINT_MINT: &str = "mint";
    pub(crate) const ENTRY_POINT_BURN: &str = "burn";
    pub(crate) const ENTRY_POINT_TRANSFER: &str = "transfer";
    pub(crate) const ENTRY_POINT_APPROVE: &str = "approve";
    pub(crate) const ENTRY_POINT_METADATA: &str = "metadata";
    pub(crate) const ENTRY_POINT_SET_APPROVE_FOR_ALL: &str = "set_approval_for_all";
    pub(crate) const ENTRY_POINT_SET_TOKEN_METADATA: &str = "set_token_metadata";
    pub(crate) const ENTRY_POINT_REGISTER_OWNER: &str = "register_owner";
    pub(crate) const ARG_COLLECTION_NAME: &str = "collection_name";
    pub(crate) const ARG_COLLECTION_SYMBOL: &str = "collection_symbol";
    pub(crate) const ARG_TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
    pub(crate) const ARG_ALLOW_MINTING: &str = "allow_minting";
    pub(crate) const ARG_MINTING_MODE: &str = "minting_mode";
    pub(crate) const ARG_HOLDER_MODE: &str = "holder_mode";
    pub(crate) const ARG_WHITELIST_MODE: &str = "whitelist_mode";
    pub(crate) const ARG_CONTRACT_WHITELIST: &str = "contract_whitelist";
    pub(crate) const NUMBER_OF_MINTED_TOKENS: &str = "number_of_minted_tokens";
    pub(crate) const ARG_TOKEN_META_DATA: &str = "token_meta_data";
    pub(crate) const METADATA_CUSTOM_VALIDATED: &str = "metadata_custom_validated";
    pub(crate) const METADATA_CEP78: &str = "metadata_cep78";
    pub(crate) const METADATA_NFT721: &str = "metadata_nft721";
    pub(crate) const METADATA_RAW: &str = "metadata_raw";
    pub(crate) const ARG_TOKEN_OWNER: &str = "token_owner";
    pub(crate) const ARG_NFT_CONTRACT_HASH: &str = "nft_contract_hash";
    pub(crate) const ARG_JSON_SCHEMA: &str = "json_schema";
    pub(crate) const ARG_APPROVE_ALL: &str = "approve_all";
    pub(crate) const ARG_NFT_METADATA_KIND: &str = "nft_metadata_kind";
    pub(crate) const ARG_IDENTIFIER_MODE: &str = "identifier_mode";
    pub(crate) const ARG_METADATA_MUTABILITY: &str = "metadata_mutability";
    pub(crate) const ARG_BURN_MODE: &str = "burn_mode";
    pub(crate) const ARG_OWNER_LOOKUP_MODE: &str = "owner_reverse_lookup_mode";
    pub(crate) const TOKEN_ISSUERS: &str = "token_issuers";
    pub(crate) const ARG_OWNERSHIP_MODE: &str = "ownership_mode";
    pub(crate) const ARG_ADDITIONAL_REQUIRED_METADATA: &str = "additional_required_metadata";
    pub(crate) const ARG_OPTIONAL_METADATA: &str = "optional_metadata";
    pub(crate) const ARG_NFT_KIND: &str = "nft_kind";
    pub(crate) const TOKEN_COUNTS: &str = "balances";
    pub(crate) const TOKEN_OWNERS: &str = "token_owners";
    pub(crate) const BURNT_TOKENS: &str = "burnt_tokens";
    pub(crate) const OPERATOR: &str = "operator";
    pub(crate) const BALANCES: &str = "balances";
    pub(crate) const RECEIPT_NAME: &str = "receipt_name";
    pub(crate) const ARG_OPERATOR: &str = "operator";
    pub(crate) const ARG_TARGET_KEY: &str = "target_key";
    pub(crate) const ARG_SOURCE_KEY: &str = "source_key";
    pub(crate) const ARG_TOKEN_ID: &str = "token_id";
    pub(crate) const ARG_TOKEN_HASH: &str = "token_hash";
    pub(crate) const ARG_KEY_NAME: &str = "key_name";
    pub(crate) const ARG_IS_HASH_IDENTIFIER_MODE: &str = "is_hash_identifier_mode";
    pub(crate) const ARG_NAMED_KEY_CONVENTION: &str = "named_key_convention";
    pub(crate) const ARG_ACCESS_KEY_NAME_1_0_0: &str = "access_key_name";
    pub(crate) const ARG_HASH_KEY_NAME_1_0_0: &str = "hash_key_name";
    pub(crate) const ACCOUNT_USER_1: [u8; 32] = [1u8; 32];
    pub(crate) const ACCOUNT_USER_2: [u8; 32] = [2u8; 32];
    pub(crate) const ACCOUNT_USER_3: [u8; 32] = [3u8; 32];
    pub(crate) const TEST_PRETTY_721_META_DATA: &str = r#"{
      "name": "John Doe",
      "symbol": "abc",
      "token_uri": "https://www.barfoo.com"
    }"#;
    pub(crate) const TEST_PRETTY_UPDATED_721_META_DATA: &str = r#"{
      "name": "John Doe",
      "symbol": "abc",
      "token_uri": "https://www.foobar.com"
    }"#;
    pub(crate) const TEST_PRETTY_CEP78_METADATA: &str = r#"{
      "name": "John Doe",
      "token_uri": "https://www.barfoo.com",
      "checksum": "940bffb3f2bba35f84313aa26da09ece3ad47045c6a1292c2bbd2df4ab1a55fb"
    }"#;
    pub(crate) const TEST_PRETTY_UPDATED_CEP78_METADATA: &str = r#"{
      "name": "John Doe",
      "token_uri": "https://www.foobar.com",
      "checksum": "fda4feaa137e83972db628e521c92159f5dc253da1565c9da697b8ad845a0788"
    }"#;
    pub(crate) const TEST_COMPACT_META_DATA: &str =
        r#"{"name": "John Doe","symbol": "abc","token_uri": "https://www.barfoo.com"}"#;
    pub(crate) const MALFORMED_META_DATA: &str = r#"{
      "name": "John Doe",
      "symbol": abc,
      "token_uri": "https://www.barfoo.com"
    }"#;
    pub(crate) const ACCESS_KEY_NAME_1_0_0: &str = "nft_contract_package_access";
    pub(crate) const CONTRACT_1_0_0_WASM: &str = "1_0_0/contract.wasm";
    pub(crate) const MINT_1_0_0_WASM: &str = "1_0_0/mint_call.wasm";
    pub(crate) const PAGE_SIZE: u64 = 1000;
    pub(crate) const UNMATCHED_HASH_COUNT: &str = "unmatched_hash_count";
    pub(crate) const PAGE_DICTIONARY_PREFIX: &str = "page_";
    pub(crate) const PAGE_LIMIT: &str = "page_limit";
    pub(crate) const HASH_KEY_NAME: &str = "nft_contract_package";
    pub(crate) const ARG_NFT_PACKAGE_HASH: &str = "nft_package_hash";
    pub(crate) const INDEX_BY_HASH: &str = "index_by_hash";
    pub(crate) const PAGE_TABLE: &str = "page_table";
    pub(crate) const ARG_MINTING_CONTRACT_REVERSE_LOOKUP: &str = "reverse_lookup";
}

use consts::{
    ARG_ADDITIONAL_REQUIRED_METADATA, ARG_ALLOW_MINTING, ARG_BURN_MODE, ARG_COLLECTION_NAME,
    ARG_COLLECTION_SYMBOL, ARG_CONTRACT_WHITELIST, ARG_HOLDER_MODE, ARG_IDENTIFIER_MODE,
    ARG_JSON_SCHEMA, ARG_METADATA_MUTABILITY, ARG_MINTING_MODE, ARG_NAMED_KEY_CONVENTION,
    ARG_NFT_KIND, ARG_NFT_METADATA_KIND, ARG_OPTIONAL_METADATA, ARG_OWNERSHIP_MODE,
    ARG_OWNER_LOOKUP_MODE, ARG_TOTAL_TOKEN_SUPPLY, ARG_WHITELIST_MODE, NFT_TEST_COLLECTION,
    NFT_TEST_SYMBOL,
};

use self::consts::TEST_PRETTY_721_META_DATA;

use super::{call_contract_memoized, TestContext};

pub(crate) static TEST_CUSTOM_METADATA_SCHEMA: Lazy<CustomMetadataSchema> = Lazy::new(|| {
    let mut properties = BTreeMap::new();
    properties.insert(
        "deity_name".to_string(),
        MetadataSchemaProperty {
            name: "deity_name".to_string(),
            description: "The name of deity from a particular pantheon.".to_string(),
            required: true,
        },
    );
    properties.insert(
        "mythology".to_string(),
        MetadataSchemaProperty {
            name: "mythology".to_string(),
            description: "The mythology the deity belongs to.".to_string(),
            required: true,
        },
    );
    CustomMetadataSchema { properties }
});

pub(crate) static TEST_CUSTOM_METADATA: Lazy<BTreeMap<String, String>> = Lazy::new(|| {
    let mut attributes = BTreeMap::new();
    attributes.insert("deity_name".to_string(), "Baldur".to_string());
    attributes.insert("mythology".to_string(), "Nordic".to_string());
    attributes
});
pub(crate) static TEST_CUSTOM_UPDATED_METADATA: Lazy<BTreeMap<String, String>> = Lazy::new(|| {
    let mut attributes = BTreeMap::new();
    attributes.insert("deity_name".to_string(), "Baldur".to_string());
    attributes.insert("mythology".to_string(), "Nordic".to_string());
    attributes.insert("enemy".to_string(), "Loki".to_string());
    attributes
});

#[repr(u8)]
pub enum WhitelistMode {
    Unlocked = 0,
    Locked = 1,
}

#[repr(u8)]
pub enum NFTHolderMode {
    Accounts = 0,
    Contracts = 1,
    Mixed = 2,
}

#[repr(u8)]
pub enum MintingMode {
    /// The ability to mint NFTs is restricted to the installing account only.
    Installer = 0,
    /// The ability to mint NFTs is not restricted.
    Public = 1,
}

#[repr(u8)]
#[derive(Debug)]
pub enum OwnershipMode {
    Minter = 0,       // The minter owns it and can never transfer it.
    Assigned = 1,     // The minter assigns it to an address and can never be transferred.
    Transferable = 2, // The NFT can be transferred even to an recipient that does not exist.
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum NFTKind {
    Physical = 0,
    Digital = 1, // The minter assigns it to an address and can never be transferred.
    Virtual = 2, // The NFT can be transferred even to an recipient that does not exist
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct MetadataSchemaProperty {
    name: String,
    description: String,
    required: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CustomMetadataSchema {
    properties: BTreeMap<String, MetadataSchemaProperty>,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    symbol: String,
    token_uri: String,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum NFTMetadataKind {
    CEP78 = 0,
    NFT721 = 1,
    Raw = 2,
    CustomValidated = 3,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum NFTIdentifierMode {
    Ordinal = 0,
    Hash = 1,
}

#[repr(u8)]
pub enum MetadataMutability {
    Immutable = 0,
    Mutable = 1,
}

#[repr(u8)]
pub enum BurnMode {
    Burnable = 0,
    NonBurnable = 1,
}

#[repr(u8)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum OwnerReverseLookupMode {
    NoLookUp = 0,
    Complete = 1,
    TransfersOnly = 2,
}

#[repr(u8)]
pub enum NamedKeyConventionMode {
    DerivedFromCollectionName = 0,
    V1_0Standard = 1,
    V1_0Custom = 2,
}

#[derive(Debug)]
pub(crate) struct InstallerRequestBuilder {
    account_hash: AccountHash,
    session_file: String,
    collection_name: CLValue,
    collection_symbol: CLValue,
    total_token_supply: CLValue,
    allow_minting: CLValue,
    minting_mode: CLValue,
    ownership_mode: CLValue,
    nft_kind: CLValue,
    holder_mode: CLValue,
    whitelist_mode: CLValue,
    contract_whitelist: CLValue,
    json_schema: CLValue,
    nft_metadata_kind: CLValue,
    identifier_mode: CLValue,
    metadata_mutability: CLValue,
    burn_mode: CLValue,
    reporting_mode: CLValue,
    named_key_convention: CLValue,
    additional_required_metadata: CLValue,
    optional_metadata: CLValue,
}

impl InstallerRequestBuilder {
    pub(crate) fn new(account_hash: AccountHash, session_file: &str) -> Self {
        Self::default()
            .with_account_hash(account_hash)
            .with_session_file(session_file.to_string())
    }

    pub(crate) fn default() -> Self {
        InstallerRequestBuilder {
            account_hash: AccountHash::default(),
            session_file: String::default(),
            collection_name: CLValue::from_t(NFT_TEST_COLLECTION.to_string())
                .expect("name is legit CLValue"),
            collection_symbol: CLValue::from_t(NFT_TEST_SYMBOL)
                .expect("collection_symbol is legit CLValue"),
            total_token_supply: CLValue::from_t(1u64).expect("total_token_supply is legit CLValue"),
            allow_minting: CLValue::from_t(true).unwrap(),
            minting_mode: CLValue::from_t(MintingMode::Installer as u8).unwrap(),
            ownership_mode: CLValue::from_t(OwnershipMode::Minter as u8).unwrap(),
            nft_kind: CLValue::from_t(NFTKind::Physical as u8).unwrap(),
            holder_mode: CLValue::from_t(NFTHolderMode::Mixed as u8).unwrap(),
            whitelist_mode: CLValue::from_t(WhitelistMode::Unlocked as u8).unwrap(),
            contract_whitelist: CLValue::from_t(Vec::<ContractHash>::new()).unwrap(),
            json_schema: CLValue::from_t("test".to_string())
                .expect("test_metadata was created from a concrete value"),
            nft_metadata_kind: CLValue::from_t(NFTMetadataKind::NFT721 as u8).unwrap(),
            identifier_mode: CLValue::from_t(NFTIdentifierMode::Ordinal as u8).unwrap(),
            metadata_mutability: CLValue::from_t(MetadataMutability::Mutable as u8).unwrap(),
            burn_mode: CLValue::from_t(BurnMode::Burnable as u8).unwrap(),
            reporting_mode: CLValue::from_t(OwnerReverseLookupMode::Complete as u8).unwrap(),
            named_key_convention: CLValue::from_t(
                NamedKeyConventionMode::DerivedFromCollectionName as u8,
            )
            .unwrap(),
            additional_required_metadata: CLValue::from_t(Bytes::new()).unwrap(),
            optional_metadata: CLValue::from_t(Bytes::new()).unwrap(),
        }
    }

    pub(crate) fn with_account_hash(mut self, account_hash: AccountHash) -> Self {
        self.account_hash = account_hash;
        self
    }

    pub(crate) fn with_session_file(mut self, session_file: String) -> Self {
        self.session_file = session_file;
        self
    }

    pub(crate) fn with_collection_name(mut self, collection_name: String) -> Self {
        self.collection_name =
            CLValue::from_t(collection_name).expect("collection_name is legit CLValue");
        self
    }

    pub(crate) fn with_invalid_collection_name(mut self, collection_name: CLValue) -> Self {
        self.collection_name = collection_name;
        self
    }

    pub(crate) fn with_collection_symbol(mut self, collection_symbol: String) -> Self {
        self.collection_symbol =
            CLValue::from_t(collection_symbol).expect("collection_symbol is legit CLValue");
        self
    }

    pub(crate) fn with_invalid_collection_symbol(mut self, collection_symbol: CLValue) -> Self {
        self.collection_symbol = collection_symbol;
        self
    }

    pub(crate) fn with_total_token_supply(mut self, total_token_supply: u64) -> Self {
        self.total_token_supply =
            CLValue::from_t(total_token_supply).expect("total_token_supply is legit CLValue");
        self
    }

    pub(crate) fn with_invalid_total_token_supply(mut self, total_token_supply: CLValue) -> Self {
        self.total_token_supply = total_token_supply;
        self
    }

    // Why Option here? The None case should be taken care of when running default
    pub(crate) fn with_allowing_minting(mut self, allow_minting: bool) -> Self {
        self.allow_minting =
            CLValue::from_t(allow_minting).expect("allow minting is legit CLValue");
        self
    }

    pub(crate) fn with_minting_mode(mut self, minting_mode: MintingMode) -> Self {
        self.minting_mode =
            CLValue::from_t(minting_mode as u8).expect("public minting is legit CLValue");
        self
    }

    pub(crate) fn with_ownership_mode(mut self, ownership_mode: OwnershipMode) -> Self {
        self.ownership_mode = CLValue::from_t(ownership_mode as u8).unwrap();
        self
    }

    pub(crate) fn with_holder_mode(mut self, holder_mode: NFTHolderMode) -> Self {
        self.holder_mode = CLValue::from_t(holder_mode as u8).unwrap();
        self
    }

    pub(crate) fn with_whitelist_mode(mut self, whitelist_mode: WhitelistMode) -> Self {
        self.whitelist_mode = CLValue::from_t(whitelist_mode as u8).unwrap();
        self
    }

    pub(crate) fn with_contract_whitelist(mut self, contract_whitelist: Vec<ContractHash>) -> Self {
        self.contract_whitelist = CLValue::from_t(contract_whitelist).unwrap();
        self
    }

    pub(crate) fn with_nft_metadata_kind(mut self, nft_metadata_kind: NFTMetadataKind) -> Self {
        self.nft_metadata_kind = CLValue::from_t(nft_metadata_kind as u8).unwrap();
        self
    }

    pub(crate) fn with_additional_required_metadata(
        mut self,
        additional_required_metadata: Vec<u8>,
    ) -> Self {
        self.additional_required_metadata =
            CLValue::from_t(Bytes::from(additional_required_metadata)).unwrap();
        self
    }

    pub(crate) fn with_optional_metadata(mut self, optional_metadata: Vec<u8>) -> Self {
        self.optional_metadata = CLValue::from_t(Bytes::from(optional_metadata)).unwrap();
        self
    }

    pub(crate) fn with_json_schema(mut self, json_schema: String) -> Self {
        self.json_schema = CLValue::from_t(json_schema).expect("json_schema is legit CLValue");
        self
    }

    pub(crate) fn with_identifier_mode(mut self, identifier_mode: NFTIdentifierMode) -> Self {
        self.identifier_mode = CLValue::from_t(identifier_mode as u8).unwrap();
        self
    }

    pub(crate) fn with_metadata_mutability(
        mut self,
        metadata_mutability: MetadataMutability,
    ) -> Self {
        self.metadata_mutability = CLValue::from_t(metadata_mutability as u8).unwrap();
        self
    }

    pub(crate) fn with_burn_mode(mut self, burn_mode: BurnMode) -> Self {
        self.burn_mode = CLValue::from_t(burn_mode as u8).unwrap();
        self
    }

    pub(crate) fn with_reporting_mode(mut self, reporting_mode: OwnerReverseLookupMode) -> Self {
        self.reporting_mode = CLValue::from_t(reporting_mode as u8).unwrap();
        self
    }

    pub(crate) fn build(self) -> RuntimeArgs {
        let mut runtime_args = RuntimeArgs::new();
        runtime_args.insert_cl_value(ARG_COLLECTION_NAME, self.collection_name);
        runtime_args.insert_cl_value(ARG_COLLECTION_SYMBOL, self.collection_symbol);
        runtime_args.insert_cl_value(ARG_TOTAL_TOKEN_SUPPLY, self.total_token_supply);
        runtime_args.insert_cl_value(ARG_ALLOW_MINTING, self.allow_minting);
        runtime_args.insert_cl_value(ARG_MINTING_MODE, self.minting_mode.clone());
        runtime_args.insert_cl_value(ARG_OWNERSHIP_MODE, self.ownership_mode);
        runtime_args.insert_cl_value(ARG_NFT_KIND, self.nft_kind);
        runtime_args.insert_cl_value(ARG_HOLDER_MODE, self.holder_mode);
        runtime_args.insert_cl_value(ARG_WHITELIST_MODE, self.whitelist_mode);
        runtime_args.insert_cl_value(ARG_CONTRACT_WHITELIST, self.contract_whitelist);
        runtime_args.insert_cl_value(ARG_JSON_SCHEMA, self.json_schema);
        runtime_args.insert_cl_value(ARG_NFT_METADATA_KIND, self.nft_metadata_kind);
        runtime_args.insert_cl_value(ARG_IDENTIFIER_MODE, self.identifier_mode);
        runtime_args.insert_cl_value(ARG_METADATA_MUTABILITY, self.metadata_mutability);
        runtime_args.insert_cl_value(ARG_BURN_MODE, self.burn_mode);
        runtime_args.insert_cl_value(ARG_OWNER_LOOKUP_MODE, self.reporting_mode);
        runtime_args.insert_cl_value(ARG_NAMED_KEY_CONVENTION, self.named_key_convention);
        runtime_args.insert_cl_value(
            ARG_ADDITIONAL_REQUIRED_METADATA,
            self.additional_required_metadata,
        );
        runtime_args.insert_cl_value(ARG_OPTIONAL_METADATA, self.optional_metadata);
        runtime_args
    }
}

pub fn register_owner(context: &mut TestContext, contract: ContractHash, owner: Key) {
    call_contract_memoized(
        context,
        contract,
        "register_owner",
        runtime_args! {
            "token_owner" => owner,
        },
    )
}

pub fn mint(context: &mut TestContext, contract: ContractHash, owner: Key) -> (String, Key, u64) {
    let (a, b, c) = call_contract_memoized::<(String, Key, String)>(
        context,
        contract,
        "mint",
        runtime_args! {
            "token_meta_data" => TEST_PRETTY_721_META_DATA.to_string(),
            "token_owner" => owner,
        },
    );

    (a, b, c.parse().unwrap())
}

pub fn owner_of(context: &mut TestContext, contract: ContractHash, token_id: u64) -> Key {
    call_contract_memoized(
        context,
        contract,
        "owner_of",
        runtime_args! {
            "token_id" => token_id,
        },
    )
}
