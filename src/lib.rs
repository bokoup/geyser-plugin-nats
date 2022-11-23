use dashmap::DashSet;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfo, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, Result, SlotStatus,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct Plugin {
    pub account_addresses: DashSet<[u8; 32]>,
    pub transaction_addresses: DashSet<[u8; 32]>,
    pub mint_authority: Pubkey,
    pub metadata_authority: Pubkey,
    pub auction_house_authority: Pubkey,
    pub nats_connection: nats::Connection,
}

// bpl-token-metadata
const BPL_TOKEN_METADATA_ID: &str = "CjSoZrc2DBZTv1UdoMx8fTcCpqEMXCyfm2EuTwy8yiGi";
const MINT_AUTHORITY_PREFIX: &str = "authority";
const METADATA_AUTHORITY_PREFIX: &str = "authority";
const AUCTION_HOUSE_AUTHORITY_ID: &str = "2R7GkXvQQS4iHptUvQMhDvRSNXL8tAuuASNvCYgz3GQW"; // platform_signer id for testing purposes

// mpl-auction-house
const AUCTION_HOUSE_LEN: usize = 459;
const LISTING_RECEIPT_LEN: usize = 236;
const BID_RECEIPT_LEN: usize = 269;
const PURCHASE_RECEIPT_LEN: usize = 193;

// mpl-token-metadata
const MAX_METADATA_LEN: usize = 679;

// spl-token
const MINT_LEN: usize = 82;
const TOKEN_ACCOUNT_LEN: usize = 165;

impl Plugin {
    pub fn new() -> Self {
        let gpl_token_program_id = Pubkey::from_str(BPL_TOKEN_METADATA_ID).unwrap();

        let mint_authority = Pubkey::find_program_address(
            &[MINT_AUTHORITY_PREFIX.as_bytes()],
            &gpl_token_program_id,
        )
        .0;
        info!("Mint authority: {mint_authority}");

        let metadata_authority = Pubkey::find_program_address(
            &[METADATA_AUTHORITY_PREFIX.as_bytes()],
            &gpl_token_program_id,
        )
        .0;
        info!("Metadata authority: {metadata_authority}");

        let auction_house_authority = Pubkey::from_str(AUCTION_HOUSE_AUTHORITY_ID).unwrap();
        info!("AuctionHouse authority: {auction_house_authority}");

        let nats_connection = nats::connect("localhost:4222").unwrap();

        Self {
            account_addresses: DashSet::new(),
            transaction_addresses: DashSet::new(),
            mint_authority,
            metadata_authority,
            auction_house_authority,
            nats_connection,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountMessageData<'a> {
    #[serde(borrow)]
    pub account: AccountData<'a>,
    pub slot: u64,
    pub is_startup: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionMessageData {
    pub signature: Signature,
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>,
    pub slot: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageData<'a> {
    #[serde(borrow)]
    Account(AccountMessageData<'a>),
    Transaction(TransactionMessageData),
}

impl<'a> From<&ReplicaAccountInfo<'a>> for AccountData<'a> {
    fn from(account: &ReplicaAccountInfo<'a>) -> Self {
        Self {
            pubkey: account.pubkey,
            lamports: account.lamports,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            data: account.data,
            write_version: account.write_version,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData<'a> {
    #[serde(with = "serde_bytes")]
    pub pubkey: &'a [u8],
    pub lamports: u64,
    #[serde(with = "serde_bytes")]
    pub owner: &'a [u8],
    pub executable: bool,
    pub rent_epoch: u64,
    #[serde(with = "serde_bytes")]
    pub data: &'a [u8],
    pub write_version: u64,
}

impl GeyserPlugin for Plugin {
    fn name(&self) -> &'static str {
        "GeyserPluginNats"
    }

    // TODO: add functionality to query mint and auction_house addresses to add to config.json file.
    // For now make sure to do manually.
    fn on_load(&mut self, config_file: &str) -> Result<()> {
        solana_logger::setup_with_default("debug");
        info!(
            "Loading {:?} from config_file {:?}",
            self.name(),
            config_file
        );

        let data = fs::read_to_string(config_file).unwrap();
        let config: serde_json::Value = serde_json::from_str(&data).unwrap();

        fn addresses_to_dashset(addresses: &Vec<Value>) -> DashSet<[u8; 32]> {
            DashSet::from_iter(addresses.iter().map(|val| {
                let val = val.as_str().unwrap().to_string();
                let mut output = [0; 32];
                bs58::decode(val).into(&mut output).unwrap();
                output
            }))
        }

        if let Some(addresses) = config["account_addresses"].as_array() {
            self.account_addresses
                .extend(addresses_to_dashset(addresses));
        };

        if let Some(addresses) = config["transaction_addresses"].as_array() {
            self.transaction_addresses
                .extend(addresses_to_dashset(addresses));
        };

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin {:?}", self.name())
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> Result<()> {
        let account: AccountData = match account {
            ReplicaAccountInfoVersions::V0_0_1(account) => account.into(),
        };

        if !self.account_addresses.contains(account.pubkey)
            & !self.account_addresses.contains(account.owner)
        {
            return Ok(());
        }

        match account.data.len() {
            // mpl_token_metadata
            MAX_METADATA_LEN => {
                if !(&account.data[1..33] == self.metadata_authority.as_ref()) {
                    return Ok(());
                }
            }

            // mpl_auction_house
            AUCTION_HOUSE_LEN => {
                if !(self.auction_house_authority.as_ref() == &account.data[168..200]) {
                    return Ok(());
                } else {
                    self.account_addresses
                        .insert(account.pubkey.as_ref().try_into().unwrap());
                }
            }
            BID_RECEIPT_LEN | LISTING_RECEIPT_LEN => {
                if !self
                    .account_addresses
                    .contains::<[u8; 32]>(account.data[72..104].try_into().unwrap())
                {
                    return Ok(());
                }
            }
            PURCHASE_RECEIPT_LEN => {
                if !self
                    .account_addresses
                    .contains::<[u8; 32]>(account.data[104..136].try_into().unwrap())
                {
                    return Ok(());
                }
            }

            // spl_token
            MINT_LEN => {
                if !(&account.data[4..36] == self.mint_authority.as_ref()) {
                    return Ok(());
                } else {
                    self.account_addresses
                        .insert(account.pubkey.as_ref().try_into().unwrap());
                }
            }
            TOKEN_ACCOUNT_LEN => {
                if !self
                    .account_addresses
                    .contains::<[u8; 32]>(account.data[..32].try_into().unwrap())
                {
                    return Ok(());
                }
            }
            _ => (),
        }

        let m = MessageData::Account(AccountMessageData {
            account,
            slot,
            is_startup,
        });

        self.nats_connection
            .publish("messages.account", bincode::serialize(&m).unwrap())
            .unwrap();
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: SlotStatus,
    ) -> Result<()> {
        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> Result<()> {
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction_info: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> Result<()> {
        match transaction_info {
            ReplicaTransactionInfoVersions::V0_0_1(transaction_info) => {
                if !transaction_info.is_vote {
                    let account_keys = transaction_info.transaction.message().account_keys();
                    for (pubkey, instruction) in transaction_info
                        .transaction
                        .message()
                        .program_instructions_iter()
                    {
                        if self.transaction_addresses.contains(pubkey.as_ref()) {
                            let t = MessageData::Transaction(TransactionMessageData {
                                signature: transaction_info.signature.clone(),
                                program_id: pubkey.clone(),
                                accounts: instruction
                                    .accounts
                                    .iter()
                                    .map(|i| account_keys.get(*i as usize).unwrap().clone())
                                    .collect(),
                                data: instruction.data.clone(),
                                slot,
                            });
                            self.nats_connection
                                .publish("messages.transaction", bincode::serialize(&t).unwrap())
                                .unwrap();
                        }
                    }
                }
            }
        };

        Ok(())
    }

    fn notify_block_metadata(&mut self, block_info: ReplicaBlockInfoVersions) -> Result<()> {
        Ok(())
    }

    /// Check if the plugin is interested in account data
    /// Default is true -- if the plugin is not interested in
    /// account data, please return false.
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    /// Check if the plugin is interested in transaction data
    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the Plugin pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Plugin::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
