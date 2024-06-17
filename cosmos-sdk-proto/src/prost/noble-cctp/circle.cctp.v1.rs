// @generated
/// *
/// A public key used to verify message signatures
/// @param attester ECDSA uncompressed public key, hex encoded
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attester {
    #[prost(string, tag = "1")]
    pub attester: ::prost::alloc::string::String,
}
/// *
/// Message format for BurnMessages
/// @param version the message body version
/// @param burn_token the burn token address on source domain as bytes32
/// @param mint_recipient the mint recipient address as bytes32
/// @param amount the burn amount
/// @param message_sender the message sender
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnMessage {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub burn_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub mint_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub message_sender: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Message format for BurningAndMintingPaused
/// @param paused true if paused, false if not paused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurningAndMintingPaused {
    #[prost(bool, tag = "1")]
    pub paused: bool,
}
// *
// Event signatures in the CCTP module

/// *
/// Emitted when an attester is enabled
/// @param attester newly enabled attester
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttesterEnabled {
    #[prost(string, tag = "1")]
    pub attester: ::prost::alloc::string::String,
}
/// *
/// Emitted when an attester is disabled
/// @param attester newly disabled attester
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttesterDisabled {
    #[prost(string, tag = "1")]
    pub attester: ::prost::alloc::string::String,
}
/// *
/// Emitted when threshold number of attestations (m in m/n multisig) is updated
/// @param old_signature_threshold old signature threshold
/// @param new_signature_threshold new signature threshold
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureThresholdUpdated {
    #[prost(uint64, tag = "1")]
    pub old_signature_threshold: u64,
    #[prost(uint64, tag = "2")]
    pub new_signature_threshold: u64,
}
/// *
/// Emitted when owner address is updated
/// @param previous_owner representing the address of the previous owner
/// @param new_owner representing the address of the new owner
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnerUpdated {
    #[prost(string, tag = "1")]
    pub previous_owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_owner: ::prost::alloc::string::String,
}
/// *
/// Emitted when starting the two stage transfer ownership process
/// @param previousOwner representing the address of the previous owner
/// @param newOwner representing the address of the new owner
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipTransferStarted {
    #[prost(string, tag = "1")]
    pub previous_owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_owner: ::prost::alloc::string::String,
}
/// *
/// Emitted when pauser address is updated
/// @param previous_pauser representing the address of the previous pauser
/// @param new_pauser representing the address of the new pauser
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauserUpdated {
    #[prost(string, tag = "1")]
    pub previous_pauser: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_pauser: ::prost::alloc::string::String,
}
/// *
/// Emitted when attester manager address is updated
/// @param previous_attester_manager representing the address of the previous
/// attester manager
/// @param new_attester_manager representing the address of the new attester
/// manager
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttesterManagerUpdated {
    #[prost(string, tag = "1")]
    pub previous_attester_manager: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_attester_manager: ::prost::alloc::string::String,
}
/// *
/// Emitted when token controller address is updated
/// @param previous_token_controller representing the address of the previous
/// token controller
/// @param new_token_controller representing the address of the new token
/// controller
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenControllerUpdated {
    #[prost(string, tag = "1")]
    pub previous_token_controller: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_token_controller: ::prost::alloc::string::String,
}
/// *
/// Emitted when burning and minting tokens is paused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurningAndMintingPausedEvent {}
/// *
/// Emitted when burning and minting tokens is unpaused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurningAndMintingUnpausedEvent {}
/// *
/// Emitted when sending and receiving messages is paused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendingAndReceivingPausedEvent {}
/// *
/// Emitted when sending and receiving messages is paused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendingAndReceivingUnpausedEvent {}
/// *
/// Emitted when a DepositForBurn message is sent
/// @param nonce unique nonce reserved by message
/// @param burn_token address of token burnt on source domain
/// @param amount deposit amount
/// @param depositor address where deposit is transferred from
/// @param mint_recipient address receiving minted tokens on destination domain
/// as bytes32
/// @param destination_domain destination domain
/// @param destination_token_messenger address of TokenMessenger on destination
/// domain as bytes32
/// @param destination_caller authorized caller as bytes32 of receiveMessage() on
/// destination domain, if not equal to bytes32(0). If equal to bytes32(0), any
/// address can call receiveMessage().
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositForBurn {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
    #[prost(string, tag = "2")]
    pub burn_token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub mint_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "6")]
    pub destination_domain: u32,
    #[prost(bytes = "vec", tag = "7")]
    pub destination_token_messenger: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub destination_caller: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when tokens are minted
/// @param mint_recipient recipient address of minted tokens
/// @param amount amount of minted tokens
/// @param mint_token contract address of minted token
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintAndWithdraw {
    #[prost(bytes = "vec", tag = "1")]
    pub mint_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub mint_token: ::prost::alloc::string::String,
}
/// *
/// Emitted when a token pair is linked
/// @param local_token local token to support
/// @param remote_domain remote domain
/// @param remote_token token on `remoteDomain` corresponding to `localToken`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenPairLinked {
    #[prost(string, tag = "1")]
    pub local_token: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub remote_domain: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub remote_token: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when a token pair is unlinked
/// @param local_token local token address
/// @param remote_domain remote domain
/// @param remote_token token on `remoteDomain` unlinked from `localToken`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenPairUnlinked {
    #[prost(string, tag = "1")]
    pub local_token: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub remote_domain: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub remote_token: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when a new message is dispatched
/// @param message Raw bytes of message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSent {
    #[prost(bytes = "vec", tag = "1")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when a new message is received
/// @param caller caller (msg.sender) on destination domain
/// @param source_domain the source domain this message originated from
/// @param nonce the nonce unique to this message
/// @param sender the sender of this message
/// @param message_body message body bytes
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReceived {
    #[prost(string, tag = "1")]
    pub caller: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub source_domain: u32,
    #[prost(uint64, tag = "3")]
    pub nonce: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub message_body: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when max message body size is updated
/// @param new_max_message_body_size new maximum message body size, in bytes
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxMessageBodySizeUpdated {
    #[prost(uint64, tag = "1")]
    pub new_max_message_body_size: u64,
}
/// *
/// Emitted when a RemoteTokenMessenger is added
/// @param domain remote domain
/// @param remote_token_messenger RemoteTokenMessenger on domain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteTokenMessengerAdded {
    #[prost(uint32, tag = "1")]
    pub domain: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub remote_token_messenger: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when a RemoteTokenMessenger is removed
/// @param domain remote domain
/// @param remote_token_messenger RemoteTokenMessenger on domain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteTokenMessengerRemoved {
    #[prost(uint32, tag = "1")]
    pub domain: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub remote_token_messenger: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Emitted when max burn amount per message is updated
/// @param local_token
/// @param old_amount old max burn amount
/// @param new_amount new max burn amount
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBurnLimitPerMessage {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub burn_limit_per_message: ::prost::alloc::string::String,
}
/// *
/// Message format for BurningAndMintingPaused
/// @param paused true if paused, false if not paused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxMessageBodySize {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// *
/// The Nonce type functions both to mark receipt of received messages and a
/// counter for sending messages
/// @param source_domain the domain id, used to mark used nonces for received
/// messages
/// @param nonce the nonce number
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nonce {
    #[prost(uint32, tag = "1")]
    pub source_domain: u32,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}
/// *
/// PerMessageBurnLimit is the maximum amount of a certain denom that can be
/// burned in an single burn
/// @param denom the denom
/// @param amount the amount that can be burned (in microunits).  An amount of
/// 1000000 uusdc is equivalent to 1USDC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerMessageBurnLimit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// *
/// @param domain_id
/// @param address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteTokenMessenger {
    #[prost(uint32, tag = "1")]
    pub domain_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Message format for SendingAndReceivingMessagesPaused
/// @param paused true if paused, false if not paused
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendingAndReceivingMessagesPaused {
    #[prost(bool, tag = "1")]
    pub paused: bool,
}
/// *
/// SignatureThreshold is the minimum amount of signatures required to attest to
/// a message (the 'm' in a m/n multisig)
/// @param amount the number of signatures required
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureThreshold {
    #[prost(uint32, tag = "1")]
    pub amount: u32,
}
/// *
/// TokenPair is used to look up the Noble token (i.e. "uusdc") from a remote
/// domain token address Multiple remote_domain + remote_token pairs can map to
/// the same local_token
///
/// @param remote_domain the remote domain_id corresponding to the token
/// @param remote_token the remote token address
/// @param local_token the corresponding Noble token denom in uunits
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenPair {
    #[prost(uint32, tag = "1")]
    pub remote_domain: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub remote_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub local_token: ::prost::alloc::string::String,
}
/// GenesisState defines the cctp module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub attester_manager: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pauser: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token_controller: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub attester_list: ::prost::alloc::vec::Vec<Attester>,
    #[prost(message, repeated, tag = "7")]
    pub per_message_burn_limit_list: ::prost::alloc::vec::Vec<PerMessageBurnLimit>,
    #[prost(message, optional, tag = "8")]
    pub burning_and_minting_paused: ::core::option::Option<BurningAndMintingPaused>,
    #[prost(message, optional, tag = "9")]
    pub sending_and_receiving_messages_paused:
        ::core::option::Option<SendingAndReceivingMessagesPaused>,
    #[prost(message, optional, tag = "10")]
    pub max_message_body_size: ::core::option::Option<MaxMessageBodySize>,
    #[prost(message, optional, tag = "11")]
    pub next_available_nonce: ::core::option::Option<Nonce>,
    #[prost(message, optional, tag = "12")]
    pub signature_threshold: ::core::option::Option<SignatureThreshold>,
    #[prost(message, repeated, tag = "13")]
    pub token_pair_list: ::prost::alloc::vec::Vec<TokenPair>,
    #[prost(message, repeated, tag = "14")]
    pub used_nonces_list: ::prost::alloc::vec::Vec<Nonce>,
    #[prost(message, repeated, tag = "15")]
    pub token_messenger_list: ::prost::alloc::vec::Vec<RemoteTokenMessenger>,
}
/// *
/// Generic message header for all messages passing through CCTP
/// The message body is dynamically-sized to support custom message body
/// formats. Other fields must be fixed-size to avoid hash collisions.
///
/// Padding: uintNN fields are left-padded, and bytesNN fields are right-padded.
///
/// @param version the version of the message format
/// @param source_domain domain of home chain
/// @param destination_domain domain of destination chain
/// @param nonce destination-specific nonce
/// @param sender address of sender on source chain as bytes32
/// @param recipient address of recipient on destination chain as bytes32
/// @param destination_caller address of caller on destination chain as bytes32
/// @param message_body raw bytes of message body
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(uint32, tag = "2")]
    pub source_domain: u32,
    #[prost(uint32, tag = "3")]
    pub destination_domain: u32,
    #[prost(uint64, tag = "4")]
    pub nonce: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub destination_caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub message_body: ::prost::alloc::vec::Vec<u8>,
}
/// *
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryRolesRequest is the request type for the Query/Roles RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRolesRequest {}
/// QueryRolesResponse is the response type for the Query/Roles RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRolesResponse {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub attester_manager: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pauser: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_controller: ::prost::alloc::string::String,
}
/// QueryAttestersRequest is the request type for the Query/Attester RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAttesterRequest {
    #[prost(string, tag = "1")]
    pub attester: ::prost::alloc::string::String,
}
/// QueryAttestersResponse is the response type for the Query/Attester RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetAttesterResponse {
    #[prost(message, optional, tag = "1")]
    pub attester: ::core::option::Option<Attester>,
}
/// QueryAllAttestersRequest is the request type for the Query/Attesters RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAttestersRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllAttestersResponse is the response type for the Query/Attesters RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllAttestersResponse {
    #[prost(message, repeated, tag = "1")]
    pub attesters: ::prost::alloc::vec::Vec<Attester>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPerMessageBurnLimitRequest is the request type for the
/// Query/PerMessageBurnLimit RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPerMessageBurnLimitRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryPerMessageBurnLimitResponse is the response type for the
/// Query/PerMessageBurnLimit RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPerMessageBurnLimitResponse {
    #[prost(message, optional, tag = "1")]
    pub burn_limit: ::core::option::Option<PerMessageBurnLimit>,
}
/// QueryAllPerMessageBurnLimitsRequest is the response type for the
/// Query/PerMessageBurnLimit RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPerMessageBurnLimitsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllPerMessageBurnLimitsRequest is the response type for the
/// Query/PerMessageBurnLimit RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllPerMessageBurnLimitsResponse {
    #[prost(message, repeated, tag = "1")]
    pub burn_limits: ::prost::alloc::vec::Vec<PerMessageBurnLimit>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryBurningAndMintingPausedRequest is the request type for the
/// Query/BurningAndMintingPaused RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetBurningAndMintingPausedRequest {}
/// QueryBurningAndMintingPausedResponse is the response type for the
/// Query/BurningAndMintingPaused RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetBurningAndMintingPausedResponse {
    #[prost(message, optional, tag = "1")]
    pub paused: ::core::option::Option<BurningAndMintingPaused>,
}
/// QuerySendingAndReceivingPausedRequest is the request type for the
/// Query/SendingAndReceivingPaused RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSendingAndReceivingMessagesPausedRequest {}
/// QuerySendingAndReceivingPausedResponse is the response type for the
/// Query/SendingAndReceivingPaused RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSendingAndReceivingMessagesPausedResponse {
    #[prost(message, optional, tag = "1")]
    pub paused: ::core::option::Option<SendingAndReceivingMessagesPaused>,
}
/// QueryMaxMessageBodySizeRequest is the request type for the
/// Query/MaxMessageBodySize RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMaxMessageBodySizeRequest {}
/// QueryMaxMessageBodySizeResponse is the response type for the
/// Query/MaxMessageBodySize RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetMaxMessageBodySizeResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<MaxMessageBodySize>,
}
/// QueryGetNextAvailableNonceRequest is the request type for the
/// Query/NextAvailableNonce RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetNextAvailableNonceRequest {}
/// Query QueryGetNextAvailableNonceResponse is the response type for the
/// Query/NextAvailableNonce RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetNextAvailableNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub nonce: ::core::option::Option<Nonce>,
}
/// QuerySignatureThresholdRequest is the request type for the
/// Query/SignatureThreshold RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSignatureThresholdRequest {}
/// QuerySignatureThresholdResponse is the response type for the
/// Query/SignatureThreshold RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetSignatureThresholdResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<SignatureThreshold>,
}
/// QueryGetTokenPairRequest is the request type for the Query/TokenPair RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetTokenPairRequest {
    #[prost(uint32, tag = "1")]
    pub remote_domain: u32,
    #[prost(string, tag = "2")]
    pub remote_token: ::prost::alloc::string::String,
}
/// QueryGetTokenPairResponse is the response type for the Query/TokenPair RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetTokenPairResponse {
    #[prost(message, optional, tag = "1")]
    pub pair: ::core::option::Option<TokenPair>,
}
/// QueryAllTokenPairsRequest is the request type for the Query/TokenPairs RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTokenPairsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllTokenPairsResponse is the response type for the Query/TokenPairs RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllTokenPairsResponse {
    #[prost(message, repeated, tag = "1")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetUsedNonceRequest is the request type for the Query/UsedNonce RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetUsedNonceRequest {
    #[prost(uint32, tag = "1")]
    pub source_domain: u32,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}
/// QueryGetUsedNonceResponse is the response type for the Query/UsedNonce RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetUsedNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub nonce: ::core::option::Option<Nonce>,
}
/// QueryAllUsedNonceRequest is the request type for the Query/UsedNonces RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllUsedNoncesRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllUsedNonceResponse is the response type for the Query/UsedNonces RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllUsedNoncesResponse {
    #[prost(message, repeated, tag = "1")]
    pub used_nonces: ::prost::alloc::vec::Vec<Nonce>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryRemoteTokenMessengerRequest is the request type for the
/// Query/RemoteTokenMessenger RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRemoteTokenMessengerRequest {
    #[prost(uint32, tag = "1")]
    pub domain_id: u32,
}
/// QueryRemoteTokenMessengerResponse is the response type for the
/// Query/RemoteTokenMessenger RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRemoteTokenMessengerResponse {
    #[prost(message, optional, tag = "1")]
    pub remote_token_messenger: ::core::option::Option<RemoteTokenMessenger>,
}
/// QueryRemoteTokenMessengersRequest is the request type for the
/// Query/RemoteTokenMessengers RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRemoteTokenMessengersRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryRemoteTokenMessengersResponse is the response type for the
/// Query/RemoteTokenMessengers RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRemoteTokenMessengersResponse {
    #[prost(message, repeated, tag = "1")]
    pub remote_token_messengers: ::prost::alloc::vec::Vec<RemoteTokenMessenger>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryBurnMessageVersionRequest is the request type for the
/// Query/BurnMessageVersion RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBurnMessageVersionRequest {}
/// QueryBurnMessageVersionResponse is the response type for the
/// Query/BurnMessageVersion RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBurnMessageVersionResponse {
    /// version is the burn message version of the local domain.
    #[prost(uint32, tag = "1")]
    pub version: u32,
}
/// QueryLocalMessageVersionRequest is the request type for the
/// Query/LocalMessageVersion RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLocalMessageVersionRequest {}
/// QueryLocalMessageVersionResponse is the response type for the
/// Query/LocalMessageVersion RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLocalMessageVersionResponse {
    /// version is the message version of the local domain.
    #[prost(uint32, tag = "1")]
    pub version: u32,
}
/// QueryLocalDomainRequest is the request type for the Query/LocalDomain RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLocalDomainRequest {}
/// QueryLocalDomainResponse is the response type for the Query/LocalDomain RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLocalDomainResponse {
    /// domain_id is the id of the local domain.
    #[prost(uint32, tag = "1")]
    pub domain_id: u32,
}
/// TODO add comments
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateOwner {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_owner: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateOwnerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAttesterManager {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_attester_manager: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAttesterManagerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTokenController {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_token_controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTokenControllerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdatePauser {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_pauser: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdatePauserResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptOwner {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptOwnerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEnableAttester {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub attester: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEnableAttesterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDisableAttester {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub attester: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDisableAttesterResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPauseBurningAndMinting {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPauseBurningAndMintingResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnpauseBurningAndMinting {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnpauseBurningAndMintingResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPauseSendingAndReceivingMessages {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPauseSendingAndReceivingMessagesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnpauseSendingAndReceivingMessages {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnpauseSendingAndReceivingMessagesResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateMaxMessageBodySize {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub message_size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateMaxMessageBodySizeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMaxBurnAmountPerMessage {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub local_token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMaxBurnAmountPerMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositForBurn {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub destination_domain: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub mint_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub burn_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositForBurnResponse {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositForBurnWithCaller {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub destination_domain: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub mint_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub burn_token: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub destination_caller: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositForBurnWithCallerResponse {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReplaceDepositForBurn {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub original_message: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub original_attestation: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub new_destination_caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub new_mint_recipient: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReplaceDepositForBurnResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReceiveMessage {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub attestation: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReceiveMessageResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendMessage {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub destination_domain: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub message_body: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendMessageResponse {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendMessageWithCaller {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub destination_domain: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub message_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub destination_caller: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendMessageWithCallerResponse {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReplaceMessage {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub original_message: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub original_attestation: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub new_message_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub new_destination_caller: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgReplaceMessageResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSignatureThreshold {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub amount: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateSignatureThresholdResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLinkTokenPair {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub remote_domain: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub remote_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub local_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLinkTokenPairResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnlinkTokenPair {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub remote_domain: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub remote_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub local_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnlinkTokenPairResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddRemoteTokenMessenger {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub domain_id: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddRemoteTokenMessengerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveRemoteTokenMessenger {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub domain_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveRemoteTokenMessengerResponse {}
include!("circle.cctp.v1.tonic.rs");
// @@protoc_insertion_point(module)
