// @generated
/// GenesisState - initial state of module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Params of this module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Params defines the set of module parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Minimum stores the minimum gas price(s) for all TX on the chain.
    /// When multiple coins are defined then they are accepted alternatively.
    /// The list must be sorted by denoms asc. No duplicate denoms or zero amount
    /// values allowed. For more information see
    /// <https://docs.cosmos.network/main/modules/auth#concepts>
    #[prost(message, repeated, tag = "1")]
    pub minimum_gas_prices: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::DecCoin>,
    #[prost(string, repeated, tag = "2")]
    pub bypass_min_fee_msg_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
include!("noble.globalfee.tonic.rs");
// @@protoc_insertion_point(module)
