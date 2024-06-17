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
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklisted(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetBlacklistedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetBlacklistedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Blacklisted");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "Blacklisted",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklisted_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllBlacklistedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllBlacklistedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/noble.fiattokenfactory.Query/BlacklistedAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "BlacklistedAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn paused(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPausedRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPausedResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Paused");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Query", "Paused"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn master_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMasterMinterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMasterMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/MasterMinter");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "MasterMinter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minters(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMintersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMintersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Minters");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Query", "Minters"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minters_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMintersRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllMintersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/MintersAll");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "MintersAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pauser(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPauserRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPauserResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Pauser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Query", "Pauser"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklister(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetBlacklisterRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetBlacklisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Blacklister");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "Blacklister",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/Owner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Query", "Owner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minter_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMinterControllerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetMinterControllerResponse>,
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
                "/noble.fiattokenfactory.Query/MinterController",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "MinterController",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minter_controller_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMinterControllerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllMinterControllerResponse>,
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
                "/noble.fiattokenfactory.Query/MinterControllerAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "MinterControllerAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn minting_denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetMintingDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetMintingDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Query/MintingDenom");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Query",
                "MintingDenom",
            ));
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
        pub async fn update_master_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateMasterMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateMasterMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/noble.fiattokenfactory.Msg/UpdateMasterMinter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "UpdateMasterMinter",
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
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/UpdatePauser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "UpdatePauser",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_blacklister(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateBlacklister>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateBlacklisterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/noble.fiattokenfactory.Msg/UpdateBlacklister",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "UpdateBlacklister",
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
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/UpdateOwner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "UpdateOwner"));
            self.inner.unary(req, path, codec).await
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
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/AcceptOwner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "AcceptOwner"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn configure_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfigureMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgConfigureMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/ConfigureMinter");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "ConfigureMinter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_minter(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRemoveMinter>,
        ) -> std::result::Result<tonic::Response<super::MsgRemoveMinterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/RemoveMinter");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "RemoveMinter",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMint>,
        ) -> std::result::Result<tonic::Response<super::MsgMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/Mint");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "Mint"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBurn>,
        ) -> std::result::Result<tonic::Response<super::MsgBurnResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/Burn");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "Burn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBlacklist>,
        ) -> std::result::Result<tonic::Response<super::MsgBlacklistResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/Blacklist");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "Blacklist"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unblacklist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnblacklist>,
        ) -> std::result::Result<tonic::Response<super::MsgUnblacklistResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/Unblacklist");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "Unblacklist"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPause>,
        ) -> std::result::Result<tonic::Response<super::MsgPauseResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/Pause");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "Pause"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unpause(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUnpause>,
        ) -> std::result::Result<tonic::Response<super::MsgUnpauseResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/noble.fiattokenfactory.Msg/Unpause");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("noble.fiattokenfactory.Msg", "Unpause"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn configure_minter_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConfigureMinterController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgConfigureMinterControllerResponse>,
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
                "/noble.fiattokenfactory.Msg/ConfigureMinterController",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "ConfigureMinterController",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_minter_controller(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRemoveMinterController>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRemoveMinterControllerResponse>,
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
                "/noble.fiattokenfactory.Msg/RemoveMinterController",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "noble.fiattokenfactory.Msg",
                "RemoveMinterController",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
