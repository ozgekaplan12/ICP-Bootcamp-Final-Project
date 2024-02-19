use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use candid::{CandidType};
use ic_cdk::export::candid;
use ic_cdk::storage;

#[derive(CandidType, Deserialize, Clone)]
pub struct WeatherResult {
    pub temperature: i32,
    pub pressure: i32,
    pub humidity: i32,
    pub wind_speed: i32,
}

#[derive(CandidType, Deserialize)]
pub struct StableState {
    pub state: State,
}

#[derive(CandidType, Deserialize)]
pub struct State {
    pub weather_data: WeatherResult,
    pub hashes: Vec<(String, Hash)>,
    pub name: String,
    pub symbol: String,
    pub txid: u128,
}

#[derive(CandidType, Deserialize)]
pub struct Hash {
    // Hash struct definition can be added according to your specific requirements
    // For simplicity, assuming it's a dummy type
    pub hash_value: String,
}

#[derive(CandidType, Deserialize, Default)]
pub struct Nft {
    pub owner: Principal,
    pub id: u64,
    pub metadata: MetadataDesc,
    pub content: Vec<u8>,
}

pub type MetadataDesc = Vec<MetadataPart>;
pub type MetadataDescRef<'a> = &'a [MetadataPart];

#[derive(CandidType, Deserialize)]
pub struct MetadataPart {
    pub purpose: MetadataPurpose,
    pub key_val_data: HashMap<String, MetadataVal>,
    pub data: Vec<u8>,
    pub custom_data: Option<MetadataVal>,
}

#[derive(CandidType, Deserialize, PartialEq)]
pub enum MetadataPurpose {
    Preview,
    Rendered,
}

#[derive(CandidType, Deserialize)]
pub enum MetadataVal {
    TextContent(String),
    BlobContent(Vec<u8>),
    NatContent(u128),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    CustomContent(Cow<'static, str>),
}

impl State {
    pub fn next_txid(&mut self) -> u128 {
        let txid = self.txid;
        self.txid += 1;
        txid
    }
}

#[derive(CandidType, Deserialize)]
pub enum InterfaceId {
    Approval,
    TransactionHistory,
    Mint,
    Burn,
    TransferNotification,
}

#[derive(CandidType, Deserialize)]
pub enum ConstrainedError {
    Unauthorized,
}

#[derive(CandidType)]
pub struct ExtendedMetadataResult<'a> {
    pub metadata_desc: MetadataDescRef<'a>,
    pub token_id: u64,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub logo: WeatherResult,
    pub name: String,
    pub symbol: String,
}

#[derive(CandidType, Deserialize)]
pub enum Error {
    Unauthorized,
    InvalidTokenId,
    ZeroAddress,
    Other,
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidTokenId
    }
}

pub type Result<T = u128, E = Error> = StdResult<T, E>;
