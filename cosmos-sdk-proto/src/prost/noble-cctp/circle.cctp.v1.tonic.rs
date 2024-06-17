// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn roles(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRolesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryRolesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/Roles");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "Roles"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn attester(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetAttesterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetAttesterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/Attester");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "Attester"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn attesters(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllAttestersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllAttestersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/Attesters");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "Attesters"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn per_message_burn_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPerMessageBurnLimitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetPerMessageBurnLimitResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/PerMessageBurnLimit");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "PerMessageBurnLimit",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn per_message_burn_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllPerMessageBurnLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllPerMessageBurnLimitsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/PerMessageBurnLimits");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "PerMessageBurnLimits",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burning_and_minting_paused(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetBurningAndMintingPausedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetBurningAndMintingPausedResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Query/BurningAndMintingPaused",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "BurningAndMintingPaused",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sending_and_receiving_messages_paused(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetSendingAndReceivingMessagesPausedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetSendingAndReceivingMessagesPausedResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Query/SendingAndReceivingMessagesPaused",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "SendingAndReceivingMessagesPaused",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn max_message_body_size(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMaxMessageBodySizeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMaxMessageBodySizeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/MaxMessageBodySize");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "MaxMessageBodySize",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn next_available_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetNextAvailableNonceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetNextAvailableNonceResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/NextAvailableNonce");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "NextAvailableNonce",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn signature_threshold(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetSignatureThresholdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetSignatureThresholdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/SignatureThreshold");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "SignatureThreshold",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn token_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetTokenPairRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetTokenPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/TokenPair");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "TokenPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn token_pairs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllTokenPairsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllTokenPairsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/TokenPairs");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "TokenPairs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn used_nonce(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetUsedNonceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetUsedNonceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/UsedNonce");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "UsedNonce"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn used_nonces(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllUsedNoncesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllUsedNoncesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/UsedNonces");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "UsedNonces"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remote_token_messenger(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRemoteTokenMessengerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRemoteTokenMessengerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/RemoteTokenMessenger");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "RemoteTokenMessenger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remote_token_messengers(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRemoteTokenMessengersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRemoteTokenMessengersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/RemoteTokenMessengers");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "RemoteTokenMessengers",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burn_message_version(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBurnMessageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBurnMessageVersionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/BurnMessageVersion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "BurnMessageVersion",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn local_message_version(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLocalMessageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryLocalMessageVersionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/LocalMessageVersion");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Query",
                "LocalMessageVersion",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn local_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLocalDomainRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryLocalDomainResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Query/LocalDomain");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Query", "LocalDomain"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn accept_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAcceptOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgAcceptOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/AcceptOwner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "AcceptOwner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_remote_token_messenger(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddRemoteTokenMessenger>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAddRemoteTokenMessengerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/AddRemoteTokenMessenger");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "AddRemoteTokenMessenger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deposit_for_burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositForBurn>,
        ) -> std::result::Result<tonic::Response<super::MsgDepositForBurnResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/DepositForBurn");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "DepositForBurn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deposit_for_burn_with_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositForBurnWithCaller>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDepositForBurnWithCallerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/DepositForBurnWithCaller",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "DepositForBurnWithCaller",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disable_attester(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDisableAttester>,
        ) -> std::result::Result<tonic::Response<super::MsgDisableAttesterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/DisableAttester");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "DisableAttester"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn enable_attester(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEnableAttester>,
        ) -> std::result::Result<tonic::Response<super::MsgEnableAttesterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/EnableAttester");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "EnableAttester"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn link_token_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgLinkTokenPair>,
        ) -> std::result::Result<tonic::Response<super::MsgLinkTokenPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/LinkTokenPair");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "LinkTokenPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pause_burning_and_minting(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPauseBurningAndMinting>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPauseBurningAndMintingResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/PauseBurningAndMinting");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "PauseBurningAndMinting",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pause_sending_and_receiving_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPauseSendingAndReceivingMessages>,
        ) -> std::result::Result<
            tonic::Response<super::MsgPauseSendingAndReceivingMessagesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/PauseSendingAndReceivingMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "PauseSendingAndReceivingMessages",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReceiveMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgReceiveMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/ReceiveMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "ReceiveMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_remote_token_messenger(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRemoveRemoteTokenMessenger>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRemoveRemoteTokenMessengerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/RemoveRemoteTokenMessenger",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "RemoveRemoteTokenMessenger",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn replace_deposit_for_burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReplaceDepositForBurn>,
        ) -> std::result::Result<
            tonic::Response<super::MsgReplaceDepositForBurnResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/ReplaceDepositForBurn");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "ReplaceDepositForBurn",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn replace_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgReplaceMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgReplaceMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/ReplaceMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "ReplaceMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendMessage>,
        ) -> std::result::Result<tonic::Response<super::MsgSendMessageResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/SendMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "SendMessage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_message_with_caller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendMessageWithCaller>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSendMessageWithCallerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/SendMessageWithCaller");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "SendMessageWithCaller",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlink_token_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnlinkTokenPair>,
        ) -> std::result::Result<tonic::Response<super::MsgUnlinkTokenPairResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UnlinkTokenPair");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "UnlinkTokenPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpause_burning_and_minting(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnpauseBurningAndMinting>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnpauseBurningAndMintingResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/UnpauseBurningAndMinting",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UnpauseBurningAndMinting",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpause_sending_and_receiving_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnpauseSendingAndReceivingMessages>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUnpauseSendingAndReceivingMessagesResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/UnpauseSendingAndReceivingMessages",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UnpauseSendingAndReceivingMessages",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateOwner>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdateOwner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "UpdateOwner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_attester_manager(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAttesterManager>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateAttesterManagerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdateAttesterManager");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateAttesterManager",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_token_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateTokenController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateTokenControllerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdateTokenController");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateTokenController",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_pauser(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdatePauser>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdatePauserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/circle.cctp.v1.Msg/UpdatePauser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("circle.cctp.v1.Msg", "UpdatePauser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_max_message_body_size(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateMaxMessageBodySize>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateMaxMessageBodySizeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/UpdateMaxMessageBodySize",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateMaxMessageBodySize",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_max_burn_amount_per_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetMaxBurnAmountPerMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetMaxBurnAmountPerMessageResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/SetMaxBurnAmountPerMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "SetMaxBurnAmountPerMessage",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_signature_threshold(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateSignatureThreshold>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateSignatureThresholdResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/circle.cctp.v1.Msg/UpdateSignatureThreshold",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "circle.cctp.v1.Msg",
                "UpdateSignatureThreshold",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
