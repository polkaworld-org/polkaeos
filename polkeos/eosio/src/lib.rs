use rstd::prelude::*;
extern crate rustc_serialize;
use serde::{Serialize, Deserialize};
use parity_codec::{Encode, Decode};


#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct PermissionLevel {
    pub actor:String,
    pub permission:String
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Action {
    pub account:String,
    pub name:String,
    pub authorization:Vec<PermissionLevel>,
    // TODO: datas
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Transaction  {
    pub expiration:String,
    pub ref_block_num:u16,
    pub ref_block_prefix:u32,
    pub max_net_usage_words:u32,
    pub max_cpu_usage_ms:u32,
    pub delay_sec:u32,

    pub context_free_actions:Vec<Action>,
    pub actions:Vec<Action>,
    // TODO: transaction_extensions:Vec<Extension>

    pub signatures:Vec<String>,
    pub context_free_data:Vec<String>
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct TransactionReceipt {
    pub status: String,
    pub cpu_usage_us:u32,
    pub net_usage_words:u32,

    pub id:String,

    pub trx: Transaction,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct SignedBlockHeader {
    pub timestamp: String,
    pub producer: String,
    pub confirmed: u16,
    pub previous: String,
    pub transaction_mroot:String,
    pub action_mroot:String,
    pub schedule_version:u32,
    pub producer_signature:String
    // TODO: ext data
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct BlockVerifyTrace {
    pub header_hash:String,
    pub schedule_producer_hash:String,
    pub sig_digest:String,
    pub blockroot_merkle:String,
    pub producer_key:String
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct TransferActionData {
    pub from:String,
    pub to:String,
    pub amount:i64,
    pub precision:u32,
    pub symbol:String,
    pub memo:String
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct KeyActions {
    pub transfers:Vec<TransferActionData>
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct BlockTraceData  {
    pub id:String,
    pub num:u32,
    pub header: SignedBlockHeader,
    pub verify: BlockVerifyTrace,
    pub key_actions:KeyActions,
    pub transactions:Vec<TransactionReceipt>
}


#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct BlockHeaderTraceData  {
    pub num:u32,
}

// Extrinsic for test-runtime.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum Extrinsic {
	BlockHeader(BlockHeaderTraceData),
	Block(BlockTraceData),
	TransferAction(TransferActionData),
}

impl Extrinsic {
	pub fn block(&self) -> &BlockTraceData {
		match self {
			Extrinsic::Block(ref b) => b,
			_ => panic!("cannot convert to block ref"),
		}
	}
}