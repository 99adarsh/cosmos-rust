// @generated
/// Params defines the set of params for the distribution module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// share is % of tx fees or rewards allocated to distribution_entities
    #[prost(string, tag = "1")]
    pub share: ::prost::alloc::string::String,
    /// % of tx fees or rewards allocated to a set of global distribution entities
    /// these shares must add up to 1
    #[prost(message, repeated, tag = "2")]
    pub distribution_entities: ::prost::alloc::vec::Vec<DistributionEntity>,
    #[prost(string, tag = "3")]
    pub transfer_fee_bps: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transfer_fee_max: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub transfer_fee_denom: ::prost::alloc::string::String,
}
/// DistributionEntity defines a distribution entity
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionEntity {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub share: ::prost::alloc::string::String,
}
/// GenesisState defines the tariff module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
include!("noble.tariff.tonic.rs");
// @@protoc_insertion_point(module)
