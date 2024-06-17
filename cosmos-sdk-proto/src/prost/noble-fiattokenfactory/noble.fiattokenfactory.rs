// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blacklisted {
    #[prost(bytes = "vec", tag = "1")]
    pub address_bz: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blacklister {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterMinter {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinterController {
    #[prost(string, tag = "1")]
    pub minter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minters {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub allowance: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintingDenom {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paused {
    #[prost(bool, tag = "1")]
    pub paused: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pauser {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// GenesisState defines the fiattokenfactory module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub blacklisted_list: ::prost::alloc::vec::Vec<Blacklisted>,
    #[prost(message, optional, tag = "3")]
    pub paused: ::core::option::Option<Paused>,
    #[prost(message, optional, tag = "4")]
    pub master_minter: ::core::option::Option<MasterMinter>,
    #[prost(message, repeated, tag = "5")]
    pub minters_list: ::prost::alloc::vec::Vec<Minters>,
    #[prost(message, optional, tag = "6")]
    pub pauser: ::core::option::Option<Pauser>,
    #[prost(message, optional, tag = "7")]
    pub blacklister: ::core::option::Option<Blacklister>,
    #[prost(message, optional, tag = "8")]
    pub owner: ::core::option::Option<Owner>,
    #[prost(message, repeated, tag = "9")]
    pub minter_controller_list: ::prost::alloc::vec::Vec<MinterController>,
    #[prost(message, optional, tag = "10")]
    pub minting_denom: ::core::option::Option<MintingDenom>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetBlacklistedRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetBlacklistedResponse {
    #[prost(message, optional, tag = "1")]
    pub blacklisted: ::core::option::Option<Blacklisted>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBlacklistedRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllBlacklistedResponse {
    #[prost(message, repeated, tag = "1")]
    pub blacklisted: ::prost::alloc::vec::Vec<Blacklisted>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPausedRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPausedResponse {
    #[prost(message, optional, tag = "1")]
    pub paused: ::core::option::Option<Paused>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMasterMinterRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMasterMinterResponse {
    #[prost(message, optional, tag = "1")]
    pub master_minter: ::core::option::Option<MasterMinter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMintersRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMintersResponse {
    #[prost(message, optional, tag = "1")]
    pub minters: ::core::option::Option<Minters>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMintersRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMintersResponse {
    #[prost(message, repeated, tag = "1")]
    pub minters: ::prost::alloc::vec::Vec<Minters>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPauserRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPauserResponse {
    #[prost(message, optional, tag = "1")]
    pub pauser: ::core::option::Option<Pauser>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetBlacklisterRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetBlacklisterResponse {
    #[prost(message, optional, tag = "1")]
    pub blacklister: ::core::option::Option<Blacklister>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetOwnerRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetOwnerResponse {
    #[prost(message, optional, tag = "1")]
    pub owner: ::core::option::Option<Owner>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMinterControllerRequest {
    #[prost(string, tag = "1")]
    pub controller_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMinterControllerResponse {
    #[prost(message, optional, tag = "1")]
    pub minter_controller: ::core::option::Option<MinterController>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMinterControllerRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMinterControllerResponse {
    #[prost(message, repeated, tag = "1")]
    pub minter_controller: ::prost::alloc::vec::Vec<MinterController>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMintingDenomRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMintingDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub minting_denom: ::core::option::Option<MintingDenom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateMasterMinter {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateMasterMinterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdatePauser {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdatePauserResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBlacklister {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateBlacklisterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateOwner {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateOwnerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptOwner {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptOwnerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfigureMinter {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub allowance: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfigureMinterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveMinter {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveMinterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBlacklist {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBlacklistResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnblacklist {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnblacklistResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPause {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPauseResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnpause {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnpauseResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfigureMinterController {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub minter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConfigureMinterControllerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveMinterController {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveMinterControllerResponse {}
include!("noble.fiattokenfactory.tonic.rs");
// @@protoc_insertion_point(module)
