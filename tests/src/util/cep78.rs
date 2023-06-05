#![allow(dead_code)]

use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use casper_types::{
    account::AccountHash, bytesrepr::Bytes, runtime_args, CLValue, ContractHash, Key, RuntimeArgs,
};

pub mod consts {
    pub const PREFIX_ACCESS_KEY_NAME: &str = "cep78_contract_package_access";
    pub const PREFIX_CEP78: &str = "cep78";
    pub const PREFIX_CONTRACT_NAME: &str = "cep78_contract_hash";
    pub const PREFIX_CONTRACT_VERSION: &str = "cep78_contract_version";
    pub const PREFIX_HASH_KEY_NAME: &str = "cep78_contract_package";
    pub const PREFIX_PAGE_DICTIONARY: &str = "page";

    pub const ARG_ACCESS_KEY_NAME_1_0_0: &str = "access_key_name";
    pub const ARG_ADDITIONAL_REQUIRED_METADATA: &str = "additional_required_metadata";
    pub const ARG_ALLOW_MINTING: &str = "allow_minting";
    pub const ARG_APPROVE_ALL: &str = "approve_all";
    pub const ARG_BURN_MODE: &str = "burn_mode";
    pub const ARG_COLLECTION_NAME: &str = "collection_name";
    pub const ARG_COLLECTION_SYMBOL: &str = "collection_symbol";
    pub const ARG_CONTRACT_WHITELIST: &str = "contract_whitelist";
    pub const ARG_ACL_WHITELIST: &str = "acl_whitelist";
    pub const ARG_EVENTS_MODE: &str = "events_mode";
    pub const ARG_HASH_KEY_NAME_1_0_0: &str = "hash_key_name";
    pub const ARG_HOLDER_MODE: &str = "holder_mode";
    pub const ARG_IDENTIFIER_MODE: &str = "identifier_mode";
    pub const ARG_JSON_SCHEMA: &str = "json_schema";
    pub const ARG_OPERATOR: &str = "operator";
    pub const ARG_METADATA_MUTABILITY: &str = "metadata_mutability";
    pub const ARG_MINTING_MODE: &str = "minting_mode";
    pub const ARG_NAMED_KEY_CONVENTION: &str = "named_key_convention";
    pub const ARG_NFT_KIND: &str = "nft_kind";
    pub const ARG_NFT_METADATA_KIND: &str = "nft_metadata_kind";
    pub const ARG_NFT_PACKAGE_KEY: &str = "cep78_package_key";
    pub const ARG_OPTIONAL_METADATA: &str = "optional_metadata";
    pub const ARG_OWNERSHIP_MODE: &str = "ownership_mode";
    pub const ARG_OWNER_LOOKUP_MODE: &str = "owner_reverse_lookup_mode";
    pub const ARG_RECEIPT_NAME: &str = "receipt_name";
    pub const ARG_SOURCE_KEY: &str = "source_key";
    pub const ARG_SPENDER: &str = "spender";
    pub const ARG_TARGET_KEY: &str = "target_key";
    pub const ARG_TOKEN_HASH: &str = "token_hash";
    pub const ARG_TOKEN_ID: &str = "token_id";
    pub const ARG_TOKEN_META_DATA: &str = "token_meta_data";
    pub const ARG_TOKEN_OWNER: &str = "token_owner";
    pub const ARG_TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
    pub const ARG_WHITELIST_MODE: &str = "whitelist_mode";
    pub const ARG_TRANSFER_FILTER_CONTRACT: &str = "transfer_filter_contract";

    pub const ENTRY_POINT_APPROVE: &str = "approve";
    pub const ENTRY_POINT_BALANCE_OF: &str = "balance_of";
    pub const ENTRY_POINT_BURN: &str = "burn";
    pub const ENTRY_POINT_GET_APPROVED: &str = "get_approved";
    pub const ENTRY_POINT_INIT: &str = "init";
    pub const ENTRY_POINT_IS_APPROVED_FOR_ALL: &str = "is_approved_for_all";
    pub const ENTRY_POINT_METADATA: &str = "metadata";
    pub const ENTRY_POINT_MIGRATE: &str = "migrate";
    pub const ENTRY_POINT_MINT: &str = "mint";
    pub const ENTRY_POINT_OWNER_OF: &str = "owner_of";
    pub const ENTRY_POINT_REVOKE: &str = "revoke";
    pub const ENTRY_POINT_REGISTER_OWNER: &str = "register_owner";
    pub const ENTRY_POINT_SET_APPROVALL_FOR_ALL: &str = "set_approval_for_all";
    pub const ENTRY_POINT_SET_TOKEN_METADATA: &str = "set_token_metadata";
    pub const ENTRY_POINT_SET_VARIABLES: &str = "set_variables";
    pub const ENTRY_POINT_TRANSFER: &str = "transfer";
    pub const ENTRY_POINT_UPDATED_RECEIPTS: &str = "updated_receipts";

    pub const ALLOW_MINTING: &str = "allow_minting";
    pub const APPROVED: &str = "approved";
    pub const BURN_MODE: &str = "burn_mode";
    pub const BURNT_TOKENS: &str = "burnt_tokens";
    pub const COLLECTION_NAME: &str = "collection_name";
    pub const COLLECTION_SYMBOL: &str = "collection_symbol";
    pub const CONTRACT_WHITELIST: &str = "contract_whitelist";
    pub const ACL_WHITELIST: &str = "acl_whitelist";
    pub const EVENT_TYPE: &str = "event_type";
    pub const EVENTS: &str = "events";
    pub const EVENTS_MODE: &str = "events_mode";
    pub const HASH_BY_INDEX: &str = "hash_by_index";
    pub const HOLDER_MODE: &str = "holder_mode";
    pub const IDENTIFIER_MODE: &str = "identifier_mode";
    pub const INDEX_BY_HASH: &str = "index_by_hash";
    pub const INSTALLER: &str = "installer";
    pub const JSON_SCHEMA: &str = "json_schema";
    pub const METADATA_CEP78: &str = "metadata_cep78";
    pub const METADATA_CUSTOM_VALIDATED: &str = "metadata_custom_validated";
    pub const METADATA_MUTABILITY: &str = "metadata_mutability";
    pub const METADATA_NFT721: &str = "metadata_nft721";
    pub const METADATA_RAW: &str = "metadata_raw";
    pub const MIGRATION_FLAG: &str = "migration_flag";
    pub const MINTING_MODE: &str = "minting_mode";
    pub const NFT_KIND: &str = "nft_kind";
    pub const NFT_METADATA_KIND: &str = "nft_metadata_kind";
    pub const NFT_METADATA_KINDS: &str = "nft_metadata_kinds";
    pub const NUMBER_OF_MINTED_TOKENS: &str = "number_of_minted_tokens";
    pub const OPERATOR: &str = "operator";
    pub const OPERATORS: &str = "operators";
    pub const OWNED_TOKENS: &str = "owned_tokens";
    pub const OWNER: &str = "owner";
    pub const OWNERSHIP_MODE: &str = "ownership_mode";
    pub const PAGE_LIMIT: &str = "page_limit";
    pub const PAGE_TABLE: &str = "page_table";
    pub const RECEIPT_NAME: &str = "receipt_name";
    pub const RECIPIENT: &str = "recipient";
    pub const REPORTING_MODE: &str = "reporting_mode";
    pub const RLO_MFLAG: &str = "rlo_mflag";
    pub const SENDER: &str = "sender";
    pub const SPENDER: &str = "spender";
    pub const TOKEN_COUNT: &str = "balances";
    pub const TOKEN_ID: &str = "token_id";
    pub const TOKEN_ISSUERS: &str = "token_issuers";
    pub const TOKEN_OWNERS: &str = "token_owners";
    pub const TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
    pub const UNMATCHED_HASH_COUNT: &str = "unmatched_hash_count";
    pub const WHITELIST_MODE: &str = "whitelist_mode";
    pub const TRANSFER_FILTER_CONTRACT: &str = "transfer_filter_contract";
    pub const TRANSFER_FILTER_CONTRACT_METHOD: &str = "can_transfer";

    pub const NFT_TEST_COLLECTION: &str = "nft-test";
    pub const NFT_TEST_SYMBOL: &str = "TEST";

    pub const TEST_PRETTY_721_META_DATA: &str = r#"{
    "name": "John Doe",
    "symbol": "abc",
    "token_uri": "https://www.barfoo.com"
  }"#;
}

use consts::{
    ARG_ADDITIONAL_REQUIRED_METADATA, ARG_ALLOW_MINTING, ARG_BURN_MODE, ARG_COLLECTION_NAME,
    ARG_COLLECTION_SYMBOL, ARG_CONTRACT_WHITELIST, ARG_HOLDER_MODE, ARG_IDENTIFIER_MODE,
    ARG_JSON_SCHEMA, ARG_METADATA_MUTABILITY, ARG_MINTING_MODE, ARG_NAMED_KEY_CONVENTION,
    ARG_NFT_KIND, ARG_NFT_METADATA_KIND, ARG_OPTIONAL_METADATA, ARG_OWNERSHIP_MODE,
    ARG_OWNER_LOOKUP_MODE, ARG_TOTAL_TOKEN_SUPPLY, ARG_WHITELIST_MODE, NFT_TEST_COLLECTION,
    NFT_TEST_SYMBOL,
};

use self::consts::{
    ARG_ACL_WHITELIST, ARG_EVENTS_MODE, ARG_TRANSFER_FILTER_CONTRACT, TEST_PRETTY_721_META_DATA,
};

use super::{call_contract_with_result, TestContext};

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

#[repr(u8)]
#[derive(PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum EventsMode {
    NoEvents = 0,
    CEP47 = 1,
    CES = 2,
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
    contract_whitelist: CLValue, // Deprecated in 1.4
    acl_whitelist: CLValue,
    json_schema: CLValue,
    nft_metadata_kind: CLValue,
    identifier_mode: CLValue,
    metadata_mutability: CLValue,
    burn_mode: CLValue,
    reporting_mode: CLValue,
    named_key_convention: CLValue,
    additional_required_metadata: CLValue,
    optional_metadata: CLValue,
    events_mode: CLValue,
    transfer_filter_contract: Option<CLValue>,
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
            acl_whitelist: CLValue::from_t(Vec::<Key>::new()).unwrap(),
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
            events_mode: CLValue::from_t(EventsMode::CES as u8).unwrap(),
            transfer_filter_contract: None,
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

    // Deprecated in 1.4
    pub(crate) fn with_contract_whitelist(mut self, contract_whitelist: Vec<ContractHash>) -> Self {
        self.contract_whitelist = CLValue::from_t(contract_whitelist).unwrap();
        self
    }

    pub(crate) fn with_acl_whitelist(mut self, acl_whitelist: Vec<Key>) -> Self {
        self.acl_whitelist = CLValue::from_t(acl_whitelist).unwrap();
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

    pub(crate) fn with_events_mode(mut self, events_mode: EventsMode) -> Self {
        self.events_mode = CLValue::from_t(events_mode as u8).unwrap();
        self
    }

    pub(crate) fn with_transfer_filter_contract(mut self, transfer_filter_contract: Key) -> Self {
        self.transfer_filter_contract = Some(CLValue::from_t(transfer_filter_contract).unwrap());
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
        runtime_args.insert_cl_value(ARG_CONTRACT_WHITELIST, self.contract_whitelist); // Deprecated in 1.4
        runtime_args.insert_cl_value(ARG_ACL_WHITELIST, self.acl_whitelist);
        runtime_args.insert_cl_value(ARG_NFT_METADATA_KIND, self.nft_metadata_kind);
        runtime_args.insert_cl_value(ARG_IDENTIFIER_MODE, self.identifier_mode);
        runtime_args.insert_cl_value(ARG_METADATA_MUTABILITY, self.metadata_mutability);
        runtime_args.insert_cl_value(ARG_BURN_MODE, self.burn_mode);
        runtime_args.insert_cl_value(ARG_OWNER_LOOKUP_MODE, self.reporting_mode);
        runtime_args.insert_cl_value(ARG_NAMED_KEY_CONVENTION, self.named_key_convention);
        runtime_args.insert_cl_value(ARG_EVENTS_MODE, self.events_mode);
        runtime_args.insert_cl_value(
            ARG_ADDITIONAL_REQUIRED_METADATA,
            self.additional_required_metadata,
        );
        runtime_args.insert_cl_value(ARG_OPTIONAL_METADATA, self.optional_metadata);
        let json_schema = self
            .json_schema
            .clone()
            .into_t::<String>()
            .unwrap_or_default();
        if !json_schema.is_empty() {
            runtime_args.insert_cl_value(ARG_JSON_SCHEMA, self.json_schema);
        }

        if let Some(transfer_filter_contract) = self.transfer_filter_contract {
            runtime_args.insert_cl_value(ARG_TRANSFER_FILTER_CONTRACT, transfer_filter_contract);
        }
        runtime_args
    }
}

pub fn register_owner(context: &mut TestContext, contract: ContractHash, owner: Key) {
    call_contract_with_result(
        context,
        contract,
        "register_owner",
        runtime_args! {
            "token_owner" => owner,
        },
    )
}

pub fn mint(context: &mut TestContext, contract: ContractHash, owner: Key) -> (String, Key, u64) {
    let (a, b, c) = call_contract_with_result::<(String, Key, String)>(
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
    call_contract_with_result(
        context,
        contract,
        "owner_of",
        runtime_args! {
            "token_id" => token_id,
        },
    )
}

pub fn approve(context: &mut TestContext, contract: ContractHash, token_id: u64, spender: Key) {
    call_contract_with_result(
        context,
        contract,
        "approve",
        runtime_args! {
            "token_id" => token_id,
            "spender" => spender,
        },
    )
}
