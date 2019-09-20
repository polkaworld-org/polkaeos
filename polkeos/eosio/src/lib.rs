extern crate rustc_serialize;
use rustc_serialize::json;
use parity_codec::{Encode, Decode};
use log::info;
use std::{thread, time};
use primitives::*;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct PermissionLevel {
    pub actor:String,
    pub permission:Vec<u8>
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Action {
    pub account:Vec<u8>,
    pub name:Vec<u8>,
    pub authorization:Vec<PermissionLevel>
    // TODO: datas
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Transaction  {
    pub expiration:Vec<u8>,
    pub ref_block_num:u16,
    pub ref_block_prefix:u32,
    pub max_net_usage_words:u32,
    pub max_cpu_usage_ms:u32,
    pub delay_sec:u32,

    pub context_free_actions:Vec<Action>,
    pub actions:Vec<Action>,
    // TODO: transaction_extensions:Vec<Extension>

    pub signatures:Vec<Vec<u8>>,
    pub context_free_data:Vec<Vec<u8>>
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct TransactionReceipt {
    pub status: Vec<u8>,
    pub cpu_usage_us:u32,
    pub net_usage_words:u32,

    pub id:Vec<u8>,

    pub trx: Transaction,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct SignedBlockHeader {
    pub timestamp: Vec<u8>,
    pub producer: Vec<u8>,
    pub confirmed: u16,
    pub previous: Vec<u8>,
    pub transaction_mroot:Vec<u8>,
    pub action_mroot:Vec<u8>,
    pub schedule_version:u32,
    pub producer_signature:Vec<u8>
    // TODO: ext data
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct BlockVerifyTrace {
    pub header_hash:Vec<u8>,
    pub schedule_producer_hash:Vec<u8>,
    pub sig_digest:Vec<u8>,
    pub blockroot_merkle:Vec<u8>,
    pub producer_key:Vec<u8>
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct TransferActionData {
    pub from:Vec<u8>,
    pub to:Vec<u8>,
    pub amount:i64,
    pub precision:u32,
    pub symbol:Vec<u8>,
    pub memo:Vec<u8>
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct KeyActions {
    pub transfers:Vec<TransferActionData>
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct BlockTraceData  {
    pub id:Vec<u8>,
    pub num:u32,
    pub header: SignedBlockHeader,
    pub verify: BlockVerifyTrace,
    pub key_actions:KeyActions,
    pub transactions:Vec<TransactionReceipt>
}