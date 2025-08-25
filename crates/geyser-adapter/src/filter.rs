use std::{collections::HashMap, fs};

use anyhow::Result;
use serde::Deserialize;
use yellowstone_grpc_proto::geyser::{
    SubscribeRequest, SubscribeRequestAccountsDataSlice, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterAccountsFilter, SubscribeRequestFilterAccountsFilterMemcmp,
    SubscribeRequestFilterBlocks, SubscribeRequestFilterSlots,
    SubscribeRequestFilterTransactions,
    subscribe_request_filter_accounts_filter::Filter as AccountsFilterOneof,
    subscribe_request_filter_accounts_filter_memcmp::Data as MemcmpData,
};

#[derive(Debug, Clone, Deserialize)]
pub struct TxFilter {
    pub vote: Option<bool>,
    pub failed: Option<bool>,
    pub account_include: Vec<String>,
    pub account_exclude: Vec<String>,
    pub account_required: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountMemcmp {
    pub offset: u64,
    pub base58: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Filters {
    /// accounts to watch
    pub accounts: Vec<String>,
    /// owners to watch
    pub owners: Vec<String>,

    /// optional memcmp for accounts
    #[serde(default)]
    pub accounts_memcmp: Vec<AccountMemcmp>,

    /// optional data size for accounts
    #[serde(default)]
    pub accounts_datasize: Option<u64>,

    /// whether to subscribe blocks and which inclusions
    #[serde(default)]
    pub include_blocks: bool,

    /// slots to watch
    #[serde(default)]
    pub include_slots:bool,

    #[serde(default)]
    pub blocks_include_transactions: Option<bool>,
    #[serde(default)]
    pub blocks_include_accounts: Option<bool>,
    #[serde(default)]
    pub blocks_include_entries: Option<bool>,

    /// Whether to subscribe to transactions and how to filter them
    #[serde(default)]
    pub transactions: Option<TxFilter>,
}

impl Filters {
    // this function loads filters from the json file
    pub fn from_file(path: &str) -> Result<Self> {
        let s: String = fs::read_to_string(path)?;
        serde_json::from_str::<Self>(&s)
            .map_err(|e| anyhow::anyhow!("Failed to parse filters from file {}: {}", path, e))
    }

    pub fn to_subscribe_request(&self) -> SubscribeRequest {
        // Accounts
        let mut accounts: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
        if !self.accounts.is_empty()
            || !self.owners.is_empty()
            || self.accounts_memcmp.is_empty()
            || self.accounts_datasize.is_none()
        {
            let mut filters = vec![];
            if let Some(size) = self.accounts_datasize {
                filters.push(SubscribeRequestFilterAccountsFilter {
                    filter: Some(AccountsFilterOneof::Datasize(size)),
                });
            }

            for m in &self.accounts_memcmp {
                filters.push(SubscribeRequestFilterAccountsFilter {
                    filter: Some(AccountsFilterOneof::Memcmp(
                        SubscribeRequestFilterAccountsFilterMemcmp {
                            offset: m.offset,
                            data: Some(MemcmpData::Base58(m.base58.clone())),
                        },
                    )),
                });
            }
            accounts.insert(
                "client".to_owned(),
                SubscribeRequestFilterAccounts {
                    account: self.accounts.clone(),
                    owner: self.owners.clone(),
                    filters,
                    nonempty_txn_signature: None,
                },
            );
        }

        // Transactions
        let mut transactions: HashMap<String, SubscribeRequestFilterTransactions> = HashMap::new();
        if let Some(tx) = &self.transactions {
            transactions.insert(
                "client".to_owned(),
                SubscribeRequestFilterTransactions {
                    vote: tx.vote,
                    failed: tx.failed,
                    signature: None,
                    account_include: tx.account_include.clone(),
                    account_exclude: tx.account_exclude.clone(),
                    account_required: tx.account_required.clone(),
                },
            );
        }

        let mut blocks = HashMap::new();
        if self.include_blocks {
            blocks.insert(
                "client".to_owned(),
                SubscribeRequestFilterBlocks {
                    account_include: vec![],
                    include_accounts: self.blocks_include_accounts,
                    include_entries: self.blocks_include_entries,
                    include_transactions: self.blocks_include_transactions,
                },
            );
        }

        let mut slots = HashMap::new();
        if self.include_slots {
            slots.insert(
                "client".to_owned(),
                SubscribeRequestFilterSlots {
                    filter_by_commitment: None,
                    interslot_updates: None,
                },
            );
        }

        // Construct and return the SubscribeRequest
        SubscribeRequest {
            slots,
            accounts,
            blocks,
            blocks_meta: HashMap::new(),
            transactions,
            transactions_status: HashMap::new(),
            entry: HashMap::new(),
            accounts_data_slice: vec![SubscribeRequestAccountsDataSlice {
                offset: 0,
                length: u32::MAX as u64,
            }],
            commitment: None,
            from_slot: None,
            ping: None,
        }
    }
}
