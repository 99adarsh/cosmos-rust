// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub created_at: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(map = "string, uint64", tag = "1")]
    pub num_of_accounts: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
    #[prost(map = "string, uint64", tag = "2")]
    pub num_of_forwards: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
    #[prost(map = "string, string", tag = "3")]
    pub total_forwarded:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAccountData {
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAccountMemo {
    #[prost(message, optional, tag = "1")]
    pub noble: ::core::option::Option<register_account_memo::RegisterAccountDataWrapper>,
}
/// Nested message and enum types in `RegisterAccountMemo`.
pub mod register_account_memo {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegisterAccountDataWrapper {
        #[prost(message, optional, tag = "1")]
        pub forwarding: ::core::option::Option<super::RegisterAccountData>,
    }
}
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddress {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddressResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub exists: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStats {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStatsResponse {
    #[prost(map = "string, message", tag = "1")]
    pub stats: ::std::collections::HashMap<::prost::alloc::string::String, Stats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStatsByChannel {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStatsByChannelResponse {
    #[prost(uint64, tag = "1")]
    pub num_of_accounts: u64,
    #[prost(uint64, tag = "2")]
    pub num_of_forwards: u64,
    #[prost(message, repeated, tag = "3")]
    pub total_forwarded: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub num_of_accounts: u64,
    #[prost(uint64, tag = "3")]
    pub num_of_forwards: u64,
    #[prost(message, repeated, tag = "4")]
    pub total_forwarded: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccount {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccountResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClearAccount {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClearAccountResponse {}
include!("noble.forwarding.v1.tonic.rs");
// @@protoc_insertion_point(module)
