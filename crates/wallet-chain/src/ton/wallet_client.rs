// @generated
/// Generated client implementations.
pub mod wallet_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WalletClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletClient<tonic::transport::Channel> {
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
    impl<T> WalletClient<T>
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
        ) -> WalletClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAccountById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AccountBalanceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAccountBalance"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_balance_trace(
            &mut self,
            request: impl tonic::IntoRequest<super::block_balance_trace::BlockIdentifier>,
        ) -> std::result::Result<
            tonic::Response<super::BlockBalanceTrace>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockBalanceTrace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockBalanceTrace"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_transaction2(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateTransaction2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateTransaction2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn broadcast_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> std::result::Result<tonic::Response<super::Return>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/BroadcastTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "BroadcastTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUpdateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_account_id(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAccountIdContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/SetAccountId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "SetAccountId"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_account2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUpdateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAccount2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateAccount2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn vote_witness_account(
            &mut self,
            request: impl tonic::IntoRequest<super::VoteWitnessContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/VoteWitnessAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "VoteWitnessAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateSetting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateSetting"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_energy_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnergyLimitContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateEnergyLimit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateEnergyLimit"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn vote_witness_account2(
            &mut self,
            request: impl tonic::IntoRequest<super::VoteWitnessContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/VoteWitnessAccount2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "VoteWitnessAccount2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_asset_issue(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetIssueContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAssetIssue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateAssetIssue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_asset_issue2(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetIssueContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAssetIssue2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateAssetIssue2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_witness(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessUpdateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateWitness",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateWitness"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_witness2(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessUpdateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateWitness2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateWitness2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_account(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountCreateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_account2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAccount2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateAccount2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_witness(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessCreateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateWitness",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateWitness"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_witness2(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateWitness2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateWitness2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferAssetContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TransferAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "TransferAsset"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_asset2(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TransferAsset2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "TransferAsset2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn participate_asset_issue(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipateAssetIssueContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ParticipateAssetIssue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ParticipateAssetIssue"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn participate_asset_issue2(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipateAssetIssueContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ParticipateAssetIssue2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ParticipateAssetIssue2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn freeze_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::FreezeBalanceContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/FreezeBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "FreezeBalance"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn freeze_balance2(
            &mut self,
            request: impl tonic::IntoRequest<super::FreezeBalanceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/FreezeBalance2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "FreezeBalance2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn freeze_balance_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::FreezeBalanceV2Contract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/FreezeBalanceV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "FreezeBalanceV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unfreeze_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeBalanceContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UnfreezeBalance"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unfreeze_balance2(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeBalanceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeBalance2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UnfreezeBalance2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unfreeze_balance_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeBalanceV2Contract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeBalanceV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UnfreezeBalanceV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unfreeze_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeAssetContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UnfreezeAsset"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unfreeze_asset2(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeAsset2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UnfreezeAsset2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawBalanceContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/WithdrawBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "WithdrawBalance"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw_balance2(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawBalanceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/WithdrawBalance2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "WithdrawBalance2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw_expire_unfreeze(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawExpireUnfreezeContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/WithdrawExpireUnfreeze",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "WithdrawExpireUnfreeze"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delegate_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegateResourceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/DelegateResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "DelegateResource"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn un_delegate_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::UnDelegateResourceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnDelegateResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UnDelegateResource"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_all_unfreeze_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelAllUnfreezeV2Contract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CancelAllUnfreezeV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CancelAllUnfreezeV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateAsset"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_asset2(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAsset2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateAsset2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn proposal_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposalCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ProposalCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ProposalCreate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn proposal_approve(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposalApproveContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ProposalApprove",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ProposalApprove"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn proposal_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposalDeleteContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ProposalDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ProposalDelete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn buy_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::BuyStorageContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/BuyStorage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "BuyStorage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn buy_storage_bytes(
            &mut self,
            request: impl tonic::IntoRequest<super::BuyStorageBytesContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/BuyStorageBytes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "BuyStorageBytes"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn sell_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::SellStorageContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/SellStorage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "SellStorage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn exchange_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ExchangeCreate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn exchange_inject(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeInjectContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeInject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ExchangeInject"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn exchange_withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeWithdrawContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeWithdraw",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ExchangeWithdraw"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn exchange_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeTransactionContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ExchangeTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_sell_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketSellAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/MarketSellAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "MarketSellAsset"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketCancelOrderContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/MarketCancelOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "MarketCancelOrder"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_order_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::MarketOrder>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketOrderById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMarketOrderById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_order_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketOrderByAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMarketOrderByAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_price_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> std::result::Result<
            tonic::Response<super::MarketPriceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketPriceByPair",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMarketPriceByPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_order_list_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketOrderListByPair",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMarketOrderListByPair"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_pair_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderPairList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketPairList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMarketPairList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NodeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListNodes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("protocol.Wallet", "ListNodes"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueByAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAssetIssueByAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_net(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<
            tonic::Response<super::AccountNetMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountNet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAccountNet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<
            tonic::Response<super::AccountResourceMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAccountResource"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAssetIssueByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_list_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueListByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAssetIssueListByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAssetIssueById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_now_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNowBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetNowBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_now_block2(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNowBlock2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetNowBlock2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockByNum"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_num2(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByNum2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockByNum2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_count_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionCountByBlockNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "GetTransactionCountByBlockNum"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_limit_next(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockLimit>,
        ) -> std::result::Result<tonic::Response<super::BlockList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLimitNext",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockByLimitNext"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_limit_next2(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockLimit>,
        ) -> std::result::Result<
            tonic::Response<super::BlockListExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLimitNext2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockByLimitNext2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_latest_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLatestNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockByLatestNum"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_latest_num2(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<
            tonic::Response<super::BlockListExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLatestNum2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBlockByLatestNum2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetTransactionById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deploy_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/DeployContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "DeployContract"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::SmartContract>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetContract"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_contract_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::SmartContractDataWrapper>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetContractInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetContractInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn trigger_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TriggerContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "TriggerContract"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn trigger_constant_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TriggerConstantContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "TriggerConstantContract"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_energy(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateEnergyMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/EstimateEnergy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "EstimateEnergy"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn clear_contract_abi(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearAbiContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ClearContractABI",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ClearContractABI"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_witnesses(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::WitnessList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListWitnesses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ListWitnesses"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDelegatedResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetDelegatedResource"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDelegatedResourceV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetDelegatedResourceV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource_account_index(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDelegatedResourceAccountIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "GetDelegatedResourceAccountIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource_account_index_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDelegatedResourceAccountIndexV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "GetDelegatedResourceAccountIndexV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_can_delegated_max_size(
            &mut self,
            request: impl tonic::IntoRequest<super::CanDelegatedMaxSizeRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::CanDelegatedMaxSizeResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetCanDelegatedMaxSize",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetCanDelegatedMaxSize"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_available_unfreeze_count(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetAvailableUnfreezeCountRequestMessage,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetAvailableUnfreezeCountResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAvailableUnfreezeCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAvailableUnfreezeCount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_can_withdraw_unfreeze_amount(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CanWithdrawUnfreezeAmountRequestMessage,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CanWithdrawUnfreezeAmountResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetCanWithdrawUnfreezeAmount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "GetCanWithdrawUnfreezeAmount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_proposals(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ProposalList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListProposals",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ListProposals"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_paginated_proposal_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::ProposalList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPaginatedProposalList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetPaginatedProposalList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_proposal_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Proposal>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetProposalById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetProposalById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ExchangeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListExchanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ListExchanges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_paginated_exchange_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::ExchangeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPaginatedExchangeList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetPaginatedExchangeList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_exchange_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Exchange>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetExchangeById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetExchangeById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_chain_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ChainParameters>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetChainParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetChainParameters"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAssetIssueList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_paginated_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPaginatedAssetIssueList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "GetPaginatedAssetIssueList"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn total_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TotalTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "TotalTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_next_maintenance_time(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNextMaintenanceTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetNextMaintenanceTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_info_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionInfoById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetTransactionInfoById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn account_permission_update(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPermissionUpdateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/AccountPermissionUpdate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "AccountPermissionUpdate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_sign_weight(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionSignWeight>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionSignWeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetTransactionSignWeight"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_approved_list(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionApprovedList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionApprovedList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "GetTransactionApprovedList"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NodeInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNodeInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetNodeInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_reward_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetRewardInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetRewardInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_brokerage_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBrokerageInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBrokerageInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_brokerage(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBrokerageContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateBrokerage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "UpdateBrokerage"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shielded_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivateParameters>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateShieldedTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_merkle_tree_voucher_info(
            &mut self,
            request: impl tonic::IntoRequest<super::OutputPointInfo>,
        ) -> std::result::Result<
            tonic::Response<super::IncrementalMerkleVoucherInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMerkleTreeVoucherInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMerkleTreeVoucherInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanNoteByIvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ScanNoteByIvk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_and_mark_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptAndMarkParameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesMarked>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanAndMarkNoteByIvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ScanAndMarkNoteByIvk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_note_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanNoteByOvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "ScanNoteByOvk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_spending_key(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetSpendingKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetSpendingKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_expanded_spending_key(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ExpandedSpendingKeyMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetExpandedSpendingKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetExpandedSpendingKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ak_from_ask(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAkFromAsk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetAkFromAsk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_nk_from_nsk(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNkFromNsk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetNkFromNsk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_incoming_viewing_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewingKeyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::IncomingViewingKeyMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetIncomingViewingKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetIncomingViewingKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_diversifier(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DiversifierMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDiversifier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetDiversifier"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_new_shielded_address(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ShieldedAddressInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNewShieldedAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetNewShieldedAddress"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_zen_payment_address(
            &mut self,
            request: impl tonic::IntoRequest<super::IncomingViewingKeyDiversifierMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PaymentAddressMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetZenPaymentAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetZenPaymentAddress"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_rcm(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/protocol.Wallet/GetRcm");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("protocol.Wallet", "GetRcm"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_spend(
            &mut self,
            request: impl tonic::IntoRequest<super::NoteParameters>,
        ) -> std::result::Result<tonic::Response<super::SpendResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/protocol.Wallet/IsSpend");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("protocol.Wallet", "IsSpend"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shielded_transaction_without_spend_auth_sig(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivateParametersWithoutAsk>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedTransactionWithoutSpendAuthSig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "CreateShieldedTransactionWithoutSpendAuthSig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_shield_transaction_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetShieldTransactionHash",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetShieldTransactionHash"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_spend_auth_sig(
            &mut self,
            request: impl tonic::IntoRequest<super::SpendAuthSigParameters>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateSpendAuthSig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateSpendAuthSig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shield_nullifier(
            &mut self,
            request: impl tonic::IntoRequest<super::NfParameters>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldNullifier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateShieldNullifier"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shielded_contract_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivateShieldedTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::ShieldedTrc20Parameters>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedContractParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "CreateShieldedContractParameters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_shielded_contract_parameters_without_ask(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PrivateShieldedTrc20ParametersWithoutAsk,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ShieldedTrc20Parameters>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedContractParametersWithoutAsk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "CreateShieldedContractParametersWithoutAsk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanShieldedTRC20NotesByIvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "ScanShieldedTRC20NotesByIvk"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanShieldedTRC20NotesByOvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "ScanShieldedTRC20NotesByOvk"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_shielded_trc20_contract_note_spent(
            &mut self,
            request: impl tonic::IntoRequest<super::NfTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::NullifierResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/IsShieldedTRC20ContractNoteSpent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "IsShieldedTRC20ContractNoteSpent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_trigger_input_for_shielded_trc20_contract(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ShieldedTrc20TriggerContractParameters,
            >,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTriggerInputForShieldedTRC20Contract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.Wallet",
                        "GetTriggerInputForShieldedTRC20Contract",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_common_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateCommonTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "CreateCommonTransaction"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_info_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfoList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionInfoByBlockNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "GetTransactionInfoByBlockNum"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_burn_trx(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBurnTrx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBurnTrx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_from_pending(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionFromPending",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetTransactionFromPending"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_list_from_pending(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionIdList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionListFromPending",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.Wallet", "GetTransactionListFromPending"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_pending_size(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPendingSize",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetPendingSize"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockReq>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/protocol.Wallet/GetBlock");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("protocol.Wallet", "GetBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_bandwidth_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBandwidthPrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetBandwidthPrices"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_energy_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetEnergyPrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetEnergyPrices"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_memo_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMemoFee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Wallet", "GetMemoFee"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod wallet_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WalletServer.
    #[async_trait]
    pub trait Wallet: Send + Sync + 'static {
        async fn get_account(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status>;
        async fn get_account_by_id(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status>;
        async fn get_account_balance(
            &self,
            request: tonic::Request<super::AccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AccountBalanceResponse>,
            tonic::Status,
        >;
        async fn get_block_balance_trace(
            &self,
            request: tonic::Request<super::block_balance_trace::BlockIdentifier>,
        ) -> std::result::Result<
            tonic::Response<super::BlockBalanceTrace>,
            tonic::Status,
        >;
        async fn create_transaction(
            &self,
            request: tonic::Request<super::TransferContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn create_transaction2(
            &self,
            request: tonic::Request<super::TransferContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn broadcast_transaction(
            &self,
            request: tonic::Request<super::Transaction>,
        ) -> std::result::Result<tonic::Response<super::Return>, tonic::Status>;
        async fn update_account(
            &self,
            request: tonic::Request<super::AccountUpdateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn set_account_id(
            &self,
            request: tonic::Request<super::SetAccountIdContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn update_account2(
            &self,
            request: tonic::Request<super::AccountUpdateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn vote_witness_account(
            &self,
            request: tonic::Request<super::VoteWitnessContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn update_setting(
            &self,
            request: tonic::Request<super::UpdateSettingContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn update_energy_limit(
            &self,
            request: tonic::Request<super::UpdateEnergyLimitContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn vote_witness_account2(
            &self,
            request: tonic::Request<super::VoteWitnessContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn create_asset_issue(
            &self,
            request: tonic::Request<super::AssetIssueContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn create_asset_issue2(
            &self,
            request: tonic::Request<super::AssetIssueContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn update_witness(
            &self,
            request: tonic::Request<super::WitnessUpdateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn update_witness2(
            &self,
            request: tonic::Request<super::WitnessUpdateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn create_account(
            &self,
            request: tonic::Request<super::AccountCreateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn create_account2(
            &self,
            request: tonic::Request<super::AccountCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn create_witness(
            &self,
            request: tonic::Request<super::WitnessCreateContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn create_witness2(
            &self,
            request: tonic::Request<super::WitnessCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn transfer_asset(
            &self,
            request: tonic::Request<super::TransferAssetContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn transfer_asset2(
            &self,
            request: tonic::Request<super::TransferAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn participate_asset_issue(
            &self,
            request: tonic::Request<super::ParticipateAssetIssueContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn participate_asset_issue2(
            &self,
            request: tonic::Request<super::ParticipateAssetIssueContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn freeze_balance(
            &self,
            request: tonic::Request<super::FreezeBalanceContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn freeze_balance2(
            &self,
            request: tonic::Request<super::FreezeBalanceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn freeze_balance_v2(
            &self,
            request: tonic::Request<super::FreezeBalanceV2Contract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn unfreeze_balance(
            &self,
            request: tonic::Request<super::UnfreezeBalanceContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn unfreeze_balance2(
            &self,
            request: tonic::Request<super::UnfreezeBalanceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn unfreeze_balance_v2(
            &self,
            request: tonic::Request<super::UnfreezeBalanceV2Contract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn unfreeze_asset(
            &self,
            request: tonic::Request<super::UnfreezeAssetContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn unfreeze_asset2(
            &self,
            request: tonic::Request<super::UnfreezeAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn withdraw_balance(
            &self,
            request: tonic::Request<super::WithdrawBalanceContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn withdraw_balance2(
            &self,
            request: tonic::Request<super::WithdrawBalanceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn withdraw_expire_unfreeze(
            &self,
            request: tonic::Request<super::WithdrawExpireUnfreezeContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn delegate_resource(
            &self,
            request: tonic::Request<super::DelegateResourceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn un_delegate_resource(
            &self,
            request: tonic::Request<super::UnDelegateResourceContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn cancel_all_unfreeze_v2(
            &self,
            request: tonic::Request<super::CancelAllUnfreezeV2Contract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn update_asset(
            &self,
            request: tonic::Request<super::UpdateAssetContract>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn update_asset2(
            &self,
            request: tonic::Request<super::UpdateAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn proposal_create(
            &self,
            request: tonic::Request<super::ProposalCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn proposal_approve(
            &self,
            request: tonic::Request<super::ProposalApproveContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn proposal_delete(
            &self,
            request: tonic::Request<super::ProposalDeleteContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn buy_storage(
            &self,
            request: tonic::Request<super::BuyStorageContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn buy_storage_bytes(
            &self,
            request: tonic::Request<super::BuyStorageBytesContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn sell_storage(
            &self,
            request: tonic::Request<super::SellStorageContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn exchange_create(
            &self,
            request: tonic::Request<super::ExchangeCreateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn exchange_inject(
            &self,
            request: tonic::Request<super::ExchangeInjectContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn exchange_withdraw(
            &self,
            request: tonic::Request<super::ExchangeWithdrawContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn exchange_transaction(
            &self,
            request: tonic::Request<super::ExchangeTransactionContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn market_sell_asset(
            &self,
            request: tonic::Request<super::MarketSellAssetContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn market_cancel_order(
            &self,
            request: tonic::Request<super::MarketCancelOrderContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn get_market_order_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::MarketOrder>, tonic::Status>;
        async fn get_market_order_by_account(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::MarketOrderList>, tonic::Status>;
        async fn get_market_price_by_pair(
            &self,
            request: tonic::Request<super::MarketOrderPair>,
        ) -> std::result::Result<tonic::Response<super::MarketPriceList>, tonic::Status>;
        async fn get_market_order_list_by_pair(
            &self,
            request: tonic::Request<super::MarketOrderPair>,
        ) -> std::result::Result<tonic::Response<super::MarketOrderList>, tonic::Status>;
        async fn get_market_pair_list(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderPairList>,
            tonic::Status,
        >;
        async fn list_nodes(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NodeList>, tonic::Status>;
        async fn get_asset_issue_by_account(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn get_account_net(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<
            tonic::Response<super::AccountNetMessage>,
            tonic::Status,
        >;
        async fn get_account_resource(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<
            tonic::Response<super::AccountResourceMessage>,
            tonic::Status,
        >;
        async fn get_asset_issue_by_name(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        >;
        async fn get_asset_issue_list_by_name(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn get_asset_issue_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        >;
        async fn get_now_block(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_now_block2(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status>;
        async fn get_block_by_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_block_by_num2(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status>;
        async fn get_transaction_count_by_block_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_block_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_block_by_limit_next(
            &self,
            request: tonic::Request<super::BlockLimit>,
        ) -> std::result::Result<tonic::Response<super::BlockList>, tonic::Status>;
        async fn get_block_by_limit_next2(
            &self,
            request: tonic::Request<super::BlockLimit>,
        ) -> std::result::Result<
            tonic::Response<super::BlockListExtention>,
            tonic::Status,
        >;
        async fn get_block_by_latest_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockList>, tonic::Status>;
        async fn get_block_by_latest_num2(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<
            tonic::Response<super::BlockListExtention>,
            tonic::Status,
        >;
        async fn get_transaction_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn deploy_contract(
            &self,
            request: tonic::Request<super::CreateSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn get_contract(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::SmartContract>, tonic::Status>;
        async fn get_contract_info(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::SmartContractDataWrapper>,
            tonic::Status,
        >;
        async fn trigger_contract(
            &self,
            request: tonic::Request<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn trigger_constant_contract(
            &self,
            request: tonic::Request<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn estimate_energy(
            &self,
            request: tonic::Request<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateEnergyMessage>,
            tonic::Status,
        >;
        async fn clear_contract_abi(
            &self,
            request: tonic::Request<super::ClearAbiContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn list_witnesses(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::WitnessList>, tonic::Status>;
        async fn get_delegated_resource(
            &self,
            request: tonic::Request<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        >;
        async fn get_delegated_resource_v2(
            &self,
            request: tonic::Request<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        >;
        async fn get_delegated_resource_account_index(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        >;
        async fn get_delegated_resource_account_index_v2(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        >;
        async fn get_can_delegated_max_size(
            &self,
            request: tonic::Request<super::CanDelegatedMaxSizeRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::CanDelegatedMaxSizeResponseMessage>,
            tonic::Status,
        >;
        async fn get_available_unfreeze_count(
            &self,
            request: tonic::Request<super::GetAvailableUnfreezeCountRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::GetAvailableUnfreezeCountResponseMessage>,
            tonic::Status,
        >;
        async fn get_can_withdraw_unfreeze_amount(
            &self,
            request: tonic::Request<super::CanWithdrawUnfreezeAmountRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::CanWithdrawUnfreezeAmountResponseMessage>,
            tonic::Status,
        >;
        async fn list_proposals(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ProposalList>, tonic::Status>;
        async fn get_paginated_proposal_list(
            &self,
            request: tonic::Request<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::ProposalList>, tonic::Status>;
        async fn get_proposal_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Proposal>, tonic::Status>;
        async fn list_exchanges(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ExchangeList>, tonic::Status>;
        async fn get_paginated_exchange_list(
            &self,
            request: tonic::Request<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::ExchangeList>, tonic::Status>;
        async fn get_exchange_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Exchange>, tonic::Status>;
        async fn get_chain_parameters(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ChainParameters>, tonic::Status>;
        async fn get_asset_issue_list(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn get_paginated_asset_issue_list(
            &self,
            request: tonic::Request<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn total_transaction(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_next_maintenance_time(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_transaction_info_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::TransactionInfo>, tonic::Status>;
        async fn account_permission_update(
            &self,
            request: tonic::Request<super::AccountPermissionUpdateContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn get_transaction_sign_weight(
            &self,
            request: tonic::Request<super::Transaction>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionSignWeight>,
            tonic::Status,
        >;
        async fn get_transaction_approved_list(
            &self,
            request: tonic::Request<super::Transaction>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionApprovedList>,
            tonic::Status,
        >;
        async fn get_node_info(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NodeInfo>, tonic::Status>;
        async fn get_reward_info(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_brokerage_info(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn update_brokerage(
            &self,
            request: tonic::Request<super::UpdateBrokerageContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn create_shielded_transaction(
            &self,
            request: tonic::Request<super::PrivateParameters>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn get_merkle_tree_voucher_info(
            &self,
            request: tonic::Request<super::OutputPointInfo>,
        ) -> std::result::Result<
            tonic::Response<super::IncrementalMerkleVoucherInfo>,
            tonic::Status,
        >;
        async fn scan_note_by_ivk(
            &self,
            request: tonic::Request<super::IvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status>;
        async fn scan_and_mark_note_by_ivk(
            &self,
            request: tonic::Request<super::IvkDecryptAndMarkParameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesMarked>,
            tonic::Status,
        >;
        async fn scan_note_by_ovk(
            &self,
            request: tonic::Request<super::OvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status>;
        async fn get_spending_key(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn get_expanded_spending_key(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ExpandedSpendingKeyMessage>,
            tonic::Status,
        >;
        async fn get_ak_from_ask(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn get_nk_from_nsk(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn get_incoming_viewing_key(
            &self,
            request: tonic::Request<super::ViewingKeyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::IncomingViewingKeyMessage>,
            tonic::Status,
        >;
        async fn get_diversifier(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DiversifierMessage>,
            tonic::Status,
        >;
        async fn get_new_shielded_address(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::ShieldedAddressInfo>,
            tonic::Status,
        >;
        async fn get_zen_payment_address(
            &self,
            request: tonic::Request<super::IncomingViewingKeyDiversifierMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PaymentAddressMessage>,
            tonic::Status,
        >;
        async fn get_rcm(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn is_spend(
            &self,
            request: tonic::Request<super::NoteParameters>,
        ) -> std::result::Result<tonic::Response<super::SpendResult>, tonic::Status>;
        async fn create_shielded_transaction_without_spend_auth_sig(
            &self,
            request: tonic::Request<super::PrivateParametersWithoutAsk>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn get_shield_transaction_hash(
            &self,
            request: tonic::Request<super::Transaction>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn create_spend_auth_sig(
            &self,
            request: tonic::Request<super::SpendAuthSigParameters>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn create_shield_nullifier(
            &self,
            request: tonic::Request<super::NfParameters>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn create_shielded_contract_parameters(
            &self,
            request: tonic::Request<super::PrivateShieldedTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::ShieldedTrc20Parameters>,
            tonic::Status,
        >;
        async fn create_shielded_contract_parameters_without_ask(
            &self,
            request: tonic::Request<super::PrivateShieldedTrc20ParametersWithoutAsk>,
        ) -> std::result::Result<
            tonic::Response<super::ShieldedTrc20Parameters>,
            tonic::Status,
        >;
        async fn scan_shielded_trc20_notes_by_ivk(
            &self,
            request: tonic::Request<super::IvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        >;
        async fn scan_shielded_trc20_notes_by_ovk(
            &self,
            request: tonic::Request<super::OvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        >;
        async fn is_shielded_trc20_contract_note_spent(
            &self,
            request: tonic::Request<super::NfTrc20Parameters>,
        ) -> std::result::Result<tonic::Response<super::NullifierResult>, tonic::Status>;
        async fn get_trigger_input_for_shielded_trc20_contract(
            &self,
            request: tonic::Request<super::ShieldedTrc20TriggerContractParameters>,
        ) -> std::result::Result<tonic::Response<super::BytesMessage>, tonic::Status>;
        async fn create_common_transaction(
            &self,
            request: tonic::Request<super::Transaction>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn get_transaction_info_by_block_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfoList>,
            tonic::Status,
        >;
        async fn get_burn_trx(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_transaction_from_pending(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn get_transaction_list_from_pending(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionIdList>,
            tonic::Status,
        >;
        async fn get_pending_size(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_block(
            &self,
            request: tonic::Request<super::BlockReq>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status>;
        async fn get_bandwidth_prices(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        >;
        async fn get_energy_prices(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        >;
        async fn get_memo_fee(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WalletServer<T: Wallet> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Wallet> WalletServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletServer<T>
    where
        T: Wallet,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/protocol.Wallet/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Account>
                    for GetAccountSvc<T> {
                        type Response = super::Account;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAccountById" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Account>
                    for GetAccountByIdSvc<T> {
                        type Response = super::Account;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountBalanceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AccountBalanceRequest>
                    for GetAccountBalanceSvc<T> {
                        type Response = super::AccountBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_balance(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockBalanceTrace" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockBalanceTraceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::block_balance_trace::BlockIdentifier,
                    > for GetBlockBalanceTraceSvc<T> {
                        type Response = super::BlockBalanceTrace;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::block_balance_trace::BlockIdentifier,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_balance_trace(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockBalanceTraceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::TransferContract>
                    for CreateTransactionSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateTransaction2" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTransaction2Svc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::TransferContract>
                    for CreateTransaction2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_transaction2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTransaction2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/BroadcastTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Transaction>
                    for BroadcastTransactionSvc<T> {
                        type Response = super::Return;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Transaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).broadcast_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateAccount" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAccountSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AccountUpdateContract>
                    for UpdateAccountSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUpdateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/SetAccountId" => {
                    #[allow(non_camel_case_types)]
                    struct SetAccountIdSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::SetAccountIdContract>
                    for SetAccountIdSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAccountIdContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_account_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAccountIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateAccount2" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAccount2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AccountUpdateContract>
                    for UpdateAccount2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountUpdateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_account2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAccount2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/VoteWitnessAccount" => {
                    #[allow(non_camel_case_types)]
                    struct VoteWitnessAccountSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::VoteWitnessContract>
                    for VoteWitnessAccountSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VoteWitnessContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).vote_witness_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VoteWitnessAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateSetting" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSettingSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UpdateSettingContract>
                    for UpdateSettingSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSettingContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_setting(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSettingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateEnergyLimit" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateEnergyLimitSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UpdateEnergyLimitContract>
                    for UpdateEnergyLimitSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEnergyLimitContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_energy_limit(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateEnergyLimitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/VoteWitnessAccount2" => {
                    #[allow(non_camel_case_types)]
                    struct VoteWitnessAccount2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::VoteWitnessContract>
                    for VoteWitnessAccount2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VoteWitnessContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).vote_witness_account2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VoteWitnessAccount2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateAssetIssue" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAssetIssueSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AssetIssueContract>
                    for CreateAssetIssueSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetIssueContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_asset_issue(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAssetIssueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateAssetIssue2" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAssetIssue2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AssetIssueContract>
                    for CreateAssetIssue2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetIssueContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_asset_issue2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAssetIssue2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateWitness" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWitnessSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WitnessUpdateContract>
                    for UpdateWitnessSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WitnessUpdateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_witness(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateWitnessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateWitness2" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWitness2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WitnessUpdateContract>
                    for UpdateWitness2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WitnessUpdateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_witness2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateWitness2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAccountSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AccountCreateContract>
                    for CreateAccountSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountCreateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateAccount2" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAccount2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AccountCreateContract>
                    for CreateAccount2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountCreateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_account2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAccount2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateWitness" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWitnessSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WitnessCreateContract>
                    for CreateWitnessSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WitnessCreateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_witness(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWitnessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateWitness2" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWitness2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WitnessCreateContract>
                    for CreateWitness2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WitnessCreateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_witness2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWitness2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/TransferAsset" => {
                    #[allow(non_camel_case_types)]
                    struct TransferAssetSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::TransferAssetContract>
                    for TransferAssetSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).transfer_asset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferAssetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/TransferAsset2" => {
                    #[allow(non_camel_case_types)]
                    struct TransferAsset2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::TransferAssetContract>
                    for TransferAsset2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).transfer_asset2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferAsset2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ParticipateAssetIssue" => {
                    #[allow(non_camel_case_types)]
                    struct ParticipateAssetIssueSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ParticipateAssetIssueContract>
                    for ParticipateAssetIssueSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ParticipateAssetIssueContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).participate_asset_issue(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParticipateAssetIssueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ParticipateAssetIssue2" => {
                    #[allow(non_camel_case_types)]
                    struct ParticipateAssetIssue2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ParticipateAssetIssueContract>
                    for ParticipateAssetIssue2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ParticipateAssetIssueContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).participate_asset_issue2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParticipateAssetIssue2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/FreezeBalance" => {
                    #[allow(non_camel_case_types)]
                    struct FreezeBalanceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::FreezeBalanceContract>
                    for FreezeBalanceSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FreezeBalanceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).freeze_balance(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FreezeBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/FreezeBalance2" => {
                    #[allow(non_camel_case_types)]
                    struct FreezeBalance2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::FreezeBalanceContract>
                    for FreezeBalance2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FreezeBalanceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).freeze_balance2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FreezeBalance2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/FreezeBalanceV2" => {
                    #[allow(non_camel_case_types)]
                    struct FreezeBalanceV2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::FreezeBalanceV2Contract>
                    for FreezeBalanceV2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FreezeBalanceV2Contract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).freeze_balance_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FreezeBalanceV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UnfreezeBalance" => {
                    #[allow(non_camel_case_types)]
                    struct UnfreezeBalanceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UnfreezeBalanceContract>
                    for UnfreezeBalanceSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnfreezeBalanceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unfreeze_balance(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnfreezeBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UnfreezeBalance2" => {
                    #[allow(non_camel_case_types)]
                    struct UnfreezeBalance2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UnfreezeBalanceContract>
                    for UnfreezeBalance2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnfreezeBalanceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unfreeze_balance2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnfreezeBalance2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UnfreezeBalanceV2" => {
                    #[allow(non_camel_case_types)]
                    struct UnfreezeBalanceV2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UnfreezeBalanceV2Contract>
                    for UnfreezeBalanceV2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnfreezeBalanceV2Contract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unfreeze_balance_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnfreezeBalanceV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UnfreezeAsset" => {
                    #[allow(non_camel_case_types)]
                    struct UnfreezeAssetSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UnfreezeAssetContract>
                    for UnfreezeAssetSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnfreezeAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unfreeze_asset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnfreezeAssetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UnfreezeAsset2" => {
                    #[allow(non_camel_case_types)]
                    struct UnfreezeAsset2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UnfreezeAssetContract>
                    for UnfreezeAsset2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnfreezeAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unfreeze_asset2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnfreezeAsset2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/WithdrawBalance" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawBalanceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WithdrawBalanceContract>
                    for WithdrawBalanceSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WithdrawBalanceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).withdraw_balance(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/WithdrawBalance2" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawBalance2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WithdrawBalanceContract>
                    for WithdrawBalance2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WithdrawBalanceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).withdraw_balance2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawBalance2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/WithdrawExpireUnfreeze" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawExpireUnfreezeSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::WithdrawExpireUnfreezeContract>
                    for WithdrawExpireUnfreezeSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::WithdrawExpireUnfreezeContract,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).withdraw_expire_unfreeze(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawExpireUnfreezeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/DelegateResource" => {
                    #[allow(non_camel_case_types)]
                    struct DelegateResourceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::DelegateResourceContract>
                    for DelegateResourceSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelegateResourceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delegate_resource(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelegateResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UnDelegateResource" => {
                    #[allow(non_camel_case_types)]
                    struct UnDelegateResourceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UnDelegateResourceContract>
                    for UnDelegateResourceSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnDelegateResourceContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).un_delegate_resource(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnDelegateResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CancelAllUnfreezeV2" => {
                    #[allow(non_camel_case_types)]
                    struct CancelAllUnfreezeV2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::CancelAllUnfreezeV2Contract>
                    for CancelAllUnfreezeV2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelAllUnfreezeV2Contract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cancel_all_unfreeze_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelAllUnfreezeV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateAsset" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAssetSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UpdateAssetContract>
                    for UpdateAssetSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_asset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAssetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateAsset2" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAsset2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UpdateAssetContract>
                    for UpdateAsset2Svc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_asset2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAsset2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ProposalCreate" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalCreateSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ProposalCreateContract>
                    for ProposalCreateSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProposalCreateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).proposal_create(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposalCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ProposalApprove" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalApproveSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ProposalApproveContract>
                    for ProposalApproveSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProposalApproveContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).proposal_approve(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposalApproveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ProposalDelete" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalDeleteSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ProposalDeleteContract>
                    for ProposalDeleteSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProposalDeleteContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).proposal_delete(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposalDeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/BuyStorage" => {
                    #[allow(non_camel_case_types)]
                    struct BuyStorageSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::BuyStorageContract>
                    for BuyStorageSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BuyStorageContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).buy_storage(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BuyStorageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/BuyStorageBytes" => {
                    #[allow(non_camel_case_types)]
                    struct BuyStorageBytesSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::BuyStorageBytesContract>
                    for BuyStorageBytesSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BuyStorageBytesContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).buy_storage_bytes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BuyStorageBytesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/SellStorage" => {
                    #[allow(non_camel_case_types)]
                    struct SellStorageSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::SellStorageContract>
                    for SellStorageSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SellStorageContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).sell_storage(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SellStorageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ExchangeCreate" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeCreateSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ExchangeCreateContract>
                    for ExchangeCreateSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExchangeCreateContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exchange_create(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExchangeCreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ExchangeInject" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeInjectSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ExchangeInjectContract>
                    for ExchangeInjectSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExchangeInjectContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exchange_inject(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExchangeInjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ExchangeWithdraw" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeWithdrawSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ExchangeWithdrawContract>
                    for ExchangeWithdrawSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExchangeWithdrawContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exchange_withdraw(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExchangeWithdrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ExchangeTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct ExchangeTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ExchangeTransactionContract>
                    for ExchangeTransactionSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExchangeTransactionContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).exchange_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExchangeTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/MarketSellAsset" => {
                    #[allow(non_camel_case_types)]
                    struct MarketSellAssetSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::MarketSellAssetContract>
                    for MarketSellAssetSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketSellAssetContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).market_sell_asset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketSellAssetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/MarketCancelOrder" => {
                    #[allow(non_camel_case_types)]
                    struct MarketCancelOrderSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::MarketCancelOrderContract>
                    for MarketCancelOrderSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketCancelOrderContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).market_cancel_order(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketCancelOrderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMarketOrderById" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrderByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetMarketOrderByIdSvc<T> {
                        type Response = super::MarketOrder;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_order_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketOrderByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMarketOrderByAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrderByAccountSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetMarketOrderByAccountSvc<T> {
                        type Response = super::MarketOrderList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_order_by_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketOrderByAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMarketPriceByPair" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketPriceByPairSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::MarketOrderPair>
                    for GetMarketPriceByPairSvc<T> {
                        type Response = super::MarketPriceList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketOrderPair>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_price_by_pair(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketPriceByPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMarketOrderListByPair" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrderListByPairSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::MarketOrderPair>
                    for GetMarketOrderListByPairSvc<T> {
                        type Response = super::MarketOrderList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketOrderPair>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_order_list_by_pair(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketOrderListByPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMarketPairList" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketPairListSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetMarketPairListSvc<T> {
                        type Response = super::MarketOrderPairList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_pair_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketPairListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ListNodes" => {
                    #[allow(non_camel_case_types)]
                    struct ListNodesSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for ListNodesSvc<T> {
                        type Response = super::NodeList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_nodes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListNodesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAssetIssueByAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueByAccountSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Account>
                    for GetAssetIssueByAccountSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_by_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueByAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAccountNet" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountNetSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Account>
                    for GetAccountNetSvc<T> {
                        type Response = super::AccountNetMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_net(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountNetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAccountResource" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountResourceSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Account>
                    for GetAccountResourceSvc<T> {
                        type Response = super::AccountResourceMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_resource(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAssetIssueByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueByNameSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetAssetIssueByNameSvc<T> {
                        type Response = super::AssetIssueContract;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_by_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAssetIssueListByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueListByNameSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetAssetIssueListByNameSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_list_by_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueListByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAssetIssueById" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetAssetIssueByIdSvc<T> {
                        type Response = super::AssetIssueContract;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetNowBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetNowBlockSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetNowBlockSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_now_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNowBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetNowBlock2" => {
                    #[allow(non_camel_case_types)]
                    struct GetNowBlock2Svc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetNowBlock2Svc<T> {
                        type Response = super::BlockExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_now_block2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNowBlock2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockByNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNumSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByNumSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockByNum2" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNum2Svc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByNum2Svc<T> {
                        type Response = super::BlockExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_num2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByNum2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionCountByBlockNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionCountByBlockNumSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NumberMessage>
                    for GetTransactionCountByBlockNumSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_count_by_block_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionCountByBlockNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockById" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetBlockByIdSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockByLimitNext" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByLimitNextSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BlockLimit>
                    for GetBlockByLimitNextSvc<T> {
                        type Response = super::BlockList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockLimit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_limit_next(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByLimitNextSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockByLimitNext2" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByLimitNext2Svc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BlockLimit>
                    for GetBlockByLimitNext2Svc<T> {
                        type Response = super::BlockListExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockLimit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_limit_next2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByLimitNext2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockByLatestNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByLatestNumSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByLatestNumSvc<T> {
                        type Response = super::BlockList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_latest_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByLatestNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlockByLatestNum2" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByLatestNum2Svc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByLatestNum2Svc<T> {
                        type Response = super::BlockListExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_latest_num2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByLatestNum2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionById" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetTransactionByIdSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/DeployContract" => {
                    #[allow(non_camel_case_types)]
                    struct DeployContractSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::CreateSmartContract>
                    for DeployContractSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSmartContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deploy_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeployContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetContract" => {
                    #[allow(non_camel_case_types)]
                    struct GetContractSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetContractSvc<T> {
                        type Response = super::SmartContract;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetContractInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetContractInfoSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetContractInfoSvc<T> {
                        type Response = super::SmartContractDataWrapper;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_contract_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetContractInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/TriggerContract" => {
                    #[allow(non_camel_case_types)]
                    struct TriggerContractSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::TriggerSmartContract>
                    for TriggerContractSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerSmartContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trigger_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TriggerContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/TriggerConstantContract" => {
                    #[allow(non_camel_case_types)]
                    struct TriggerConstantContractSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::TriggerSmartContract>
                    for TriggerConstantContractSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerSmartContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trigger_constant_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TriggerConstantContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/EstimateEnergy" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateEnergySvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::TriggerSmartContract>
                    for EstimateEnergySvc<T> {
                        type Response = super::EstimateEnergyMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerSmartContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_energy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateEnergySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ClearContractABI" => {
                    #[allow(non_camel_case_types)]
                    struct ClearContractABISvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::ClearAbiContract>
                    for ClearContractABISvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearAbiContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).clear_contract_abi(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearContractABISvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ListWitnesses" => {
                    #[allow(non_camel_case_types)]
                    struct ListWitnessesSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for ListWitnessesSvc<T> {
                        type Response = super::WitnessList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_witnesses(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWitnessesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetDelegatedResource" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::DelegatedResourceMessage>
                    for GetDelegatedResourceSvc<T> {
                        type Response = super::DelegatedResourceList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelegatedResourceMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_delegated_resource(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetDelegatedResourceV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceV2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::DelegatedResourceMessage>
                    for GetDelegatedResourceV2Svc<T> {
                        type Response = super::DelegatedResourceList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelegatedResourceMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_delegated_resource_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetDelegatedResourceAccountIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceAccountIndexSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetDelegatedResourceAccountIndexSvc<T> {
                        type Response = super::DelegatedResourceAccountIndex;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_delegated_resource_account_index(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceAccountIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetDelegatedResourceAccountIndexV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceAccountIndexV2Svc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetDelegatedResourceAccountIndexV2Svc<T> {
                        type Response = super::DelegatedResourceAccountIndex;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_delegated_resource_account_index_v2(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceAccountIndexV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetCanDelegatedMaxSize" => {
                    #[allow(non_camel_case_types)]
                    struct GetCanDelegatedMaxSizeSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::CanDelegatedMaxSizeRequestMessage,
                    > for GetCanDelegatedMaxSizeSvc<T> {
                        type Response = super::CanDelegatedMaxSizeResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CanDelegatedMaxSizeRequestMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_can_delegated_max_size(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCanDelegatedMaxSizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAvailableUnfreezeCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableUnfreezeCountSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::GetAvailableUnfreezeCountRequestMessage,
                    > for GetAvailableUnfreezeCountSvc<T> {
                        type Response = super::GetAvailableUnfreezeCountResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAvailableUnfreezeCountRequestMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_unfreeze_count(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAvailableUnfreezeCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetCanWithdrawUnfreezeAmount" => {
                    #[allow(non_camel_case_types)]
                    struct GetCanWithdrawUnfreezeAmountSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::CanWithdrawUnfreezeAmountRequestMessage,
                    > for GetCanWithdrawUnfreezeAmountSvc<T> {
                        type Response = super::CanWithdrawUnfreezeAmountResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CanWithdrawUnfreezeAmountRequestMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_can_withdraw_unfreeze_amount(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCanWithdrawUnfreezeAmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ListProposals" => {
                    #[allow(non_camel_case_types)]
                    struct ListProposalsSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for ListProposalsSvc<T> {
                        type Response = super::ProposalList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_proposals(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListProposalsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetPaginatedProposalList" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaginatedProposalListSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::PaginatedMessage>
                    for GetPaginatedProposalListSvc<T> {
                        type Response = super::ProposalList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PaginatedMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_paginated_proposal_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPaginatedProposalListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetProposalById" => {
                    #[allow(non_camel_case_types)]
                    struct GetProposalByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetProposalByIdSvc<T> {
                        type Response = super::Proposal;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_proposal_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProposalByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ListExchanges" => {
                    #[allow(non_camel_case_types)]
                    struct ListExchangesSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for ListExchangesSvc<T> {
                        type Response = super::ExchangeList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_exchanges(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListExchangesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetPaginatedExchangeList" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaginatedExchangeListSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::PaginatedMessage>
                    for GetPaginatedExchangeListSvc<T> {
                        type Response = super::ExchangeList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PaginatedMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_paginated_exchange_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPaginatedExchangeListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetExchangeById" => {
                    #[allow(non_camel_case_types)]
                    struct GetExchangeByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetExchangeByIdSvc<T> {
                        type Response = super::Exchange;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_exchange_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExchangeByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetChainParameters" => {
                    #[allow(non_camel_case_types)]
                    struct GetChainParametersSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetChainParametersSvc<T> {
                        type Response = super::ChainParameters;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_chain_parameters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetChainParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAssetIssueList" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueListSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetAssetIssueListSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetPaginatedAssetIssueList" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaginatedAssetIssueListSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::PaginatedMessage>
                    for GetPaginatedAssetIssueListSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PaginatedMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_paginated_asset_issue_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPaginatedAssetIssueListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/TotalTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct TotalTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for TotalTransactionSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).total_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetNextMaintenanceTime" => {
                    #[allow(non_camel_case_types)]
                    struct GetNextMaintenanceTimeSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetNextMaintenanceTimeSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_next_maintenance_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNextMaintenanceTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionInfoById" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionInfoByIdSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetTransactionInfoByIdSvc<T> {
                        type Response = super::TransactionInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_info_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionInfoByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/AccountPermissionUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct AccountPermissionUpdateSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::AccountPermissionUpdateContract>
                    for AccountPermissionUpdateSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AccountPermissionUpdateContract,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).account_permission_update(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccountPermissionUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionSignWeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSignWeightSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Transaction>
                    for GetTransactionSignWeightSvc<T> {
                        type Response = super::TransactionSignWeight;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Transaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_sign_weight(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSignWeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionApprovedList" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionApprovedListSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Transaction>
                    for GetTransactionApprovedListSvc<T> {
                        type Response = super::TransactionApprovedList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Transaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_approved_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionApprovedListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetNodeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeInfoSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetNodeInfoSvc<T> {
                        type Response = super::NodeInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_node_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetRewardInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetRewardInfoSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetRewardInfoSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_reward_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRewardInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBrokerageInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetBrokerageInfoSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetBrokerageInfoSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_brokerage_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBrokerageInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/UpdateBrokerage" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBrokerageSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::UpdateBrokerageContract>
                    for UpdateBrokerageSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBrokerageContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_brokerage(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateBrokerageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateShieldedTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShieldedTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::PrivateParameters>
                    for CreateShieldedTransactionSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrivateParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_shielded_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateShieldedTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMerkleTreeVoucherInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetMerkleTreeVoucherInfoSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::OutputPointInfo>
                    for GetMerkleTreeVoucherInfoSvc<T> {
                        type Response = super::IncrementalMerkleVoucherInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OutputPointInfo>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_merkle_tree_voucher_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMerkleTreeVoucherInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ScanNoteByIvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanNoteByIvkSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::IvkDecryptParameters>
                    for ScanNoteByIvkSvc<T> {
                        type Response = super::DecryptNotes;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IvkDecryptParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_note_by_ivk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanNoteByIvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ScanAndMarkNoteByIvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanAndMarkNoteByIvkSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::IvkDecryptAndMarkParameters>
                    for ScanAndMarkNoteByIvkSvc<T> {
                        type Response = super::DecryptNotesMarked;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IvkDecryptAndMarkParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_and_mark_note_by_ivk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanAndMarkNoteByIvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ScanNoteByOvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanNoteByOvkSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::OvkDecryptParameters>
                    for ScanNoteByOvkSvc<T> {
                        type Response = super::DecryptNotes;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OvkDecryptParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_note_by_ovk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanNoteByOvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetSpendingKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetSpendingKeySvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetSpendingKeySvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_spending_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSpendingKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetExpandedSpendingKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetExpandedSpendingKeySvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetExpandedSpendingKeySvc<T> {
                        type Response = super::ExpandedSpendingKeyMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_expanded_spending_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExpandedSpendingKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetAkFromAsk" => {
                    #[allow(non_camel_case_types)]
                    struct GetAkFromAskSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetAkFromAskSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_ak_from_ask(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAkFromAskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetNkFromNsk" => {
                    #[allow(non_camel_case_types)]
                    struct GetNkFromNskSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetNkFromNskSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_nk_from_nsk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNkFromNskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetIncomingViewingKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetIncomingViewingKeySvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::ViewingKeyMessage>
                    for GetIncomingViewingKeySvc<T> {
                        type Response = super::IncomingViewingKeyMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ViewingKeyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_incoming_viewing_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetIncomingViewingKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetDiversifier" => {
                    #[allow(non_camel_case_types)]
                    struct GetDiversifierSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetDiversifierSvc<T> {
                        type Response = super::DiversifierMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_diversifier(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDiversifierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetNewShieldedAddress" => {
                    #[allow(non_camel_case_types)]
                    struct GetNewShieldedAddressSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetNewShieldedAddressSvc<T> {
                        type Response = super::ShieldedAddressInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_new_shielded_address(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNewShieldedAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetZenPaymentAddress" => {
                    #[allow(non_camel_case_types)]
                    struct GetZenPaymentAddressSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::IncomingViewingKeyDiversifierMessage,
                    > for GetZenPaymentAddressSvc<T> {
                        type Response = super::PaymentAddressMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::IncomingViewingKeyDiversifierMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_zen_payment_address(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetZenPaymentAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetRcm" => {
                    #[allow(non_camel_case_types)]
                    struct GetRcmSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetRcmSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_rcm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRcmSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/IsSpend" => {
                    #[allow(non_camel_case_types)]
                    struct IsSpendSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NoteParameters>
                    for IsSpendSvc<T> {
                        type Response = super::SpendResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NoteParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).is_spend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsSpendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateShieldedTransactionWithoutSpendAuthSig" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShieldedTransactionWithoutSpendAuthSigSvc<T: Wallet>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::PrivateParametersWithoutAsk>
                    for CreateShieldedTransactionWithoutSpendAuthSigSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrivateParametersWithoutAsk>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .create_shielded_transaction_without_spend_auth_sig(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateShieldedTransactionWithoutSpendAuthSigSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetShieldTransactionHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetShieldTransactionHashSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Transaction>
                    for GetShieldTransactionHashSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Transaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_shield_transaction_hash(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetShieldTransactionHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateSpendAuthSig" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSpendAuthSigSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::SpendAuthSigParameters>
                    for CreateSpendAuthSigSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SpendAuthSigParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_spend_auth_sig(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateSpendAuthSigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateShieldNullifier" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShieldNullifierSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NfParameters>
                    for CreateShieldNullifierSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NfParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_shield_nullifier(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateShieldNullifierSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateShieldedContractParameters" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShieldedContractParametersSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::PrivateShieldedTrc20Parameters>
                    for CreateShieldedContractParametersSvc<T> {
                        type Response = super::ShieldedTrc20Parameters;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PrivateShieldedTrc20Parameters,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_shielded_contract_parameters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateShieldedContractParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateShieldedContractParametersWithoutAsk" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShieldedContractParametersWithoutAskSvc<T: Wallet>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::PrivateShieldedTrc20ParametersWithoutAsk,
                    > for CreateShieldedContractParametersWithoutAskSvc<T> {
                        type Response = super::ShieldedTrc20Parameters;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PrivateShieldedTrc20ParametersWithoutAsk,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .create_shielded_contract_parameters_without_ask(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateShieldedContractParametersWithoutAskSvc(
                            inner,
                        );
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ScanShieldedTRC20NotesByIvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanShieldedTRC20NotesByIvkSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::IvkDecryptTrc20Parameters>
                    for ScanShieldedTRC20NotesByIvkSvc<T> {
                        type Response = super::DecryptNotesTrc20;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IvkDecryptTrc20Parameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_shielded_trc20_notes_by_ivk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanShieldedTRC20NotesByIvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/ScanShieldedTRC20NotesByOvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanShieldedTRC20NotesByOvkSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::OvkDecryptTrc20Parameters>
                    for ScanShieldedTRC20NotesByOvkSvc<T> {
                        type Response = super::DecryptNotesTrc20;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OvkDecryptTrc20Parameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_shielded_trc20_notes_by_ovk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanShieldedTRC20NotesByOvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/IsShieldedTRC20ContractNoteSpent" => {
                    #[allow(non_camel_case_types)]
                    struct IsShieldedTRC20ContractNoteSpentSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NfTrc20Parameters>
                    for IsShieldedTRC20ContractNoteSpentSvc<T> {
                        type Response = super::NullifierResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NfTrc20Parameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .is_shielded_trc20_contract_note_spent(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsShieldedTRC20ContractNoteSpentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTriggerInputForShieldedTRC20Contract" => {
                    #[allow(non_camel_case_types)]
                    struct GetTriggerInputForShieldedTRC20ContractSvc<T: Wallet>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<
                        super::ShieldedTrc20TriggerContractParameters,
                    > for GetTriggerInputForShieldedTRC20ContractSvc<T> {
                        type Response = super::BytesMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ShieldedTrc20TriggerContractParameters,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_trigger_input_for_shielded_trc20_contract(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTriggerInputForShieldedTRC20ContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/CreateCommonTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCommonTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::Transaction>
                    for CreateCommonTransactionSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Transaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_common_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateCommonTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionInfoByBlockNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionInfoByBlockNumSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::NumberMessage>
                    for GetTransactionInfoByBlockNumSvc<T> {
                        type Response = super::TransactionInfoList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_info_by_block_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionInfoByBlockNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBurnTrx" => {
                    #[allow(non_camel_case_types)]
                    struct GetBurnTrxSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetBurnTrxSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_burn_trx(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBurnTrxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionFromPending" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionFromPendingSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BytesMessage>
                    for GetTransactionFromPendingSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_from_pending(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionFromPendingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetTransactionListFromPending" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionListFromPendingSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetTransactionListFromPendingSvc<T> {
                        type Response = super::TransactionIdList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_list_from_pending(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionListFromPendingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetPendingSize" => {
                    #[allow(non_camel_case_types)]
                    struct GetPendingSizeSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetPendingSizeSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_pending_size(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPendingSizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::BlockReq>
                    for GetBlockSvc<T> {
                        type Response = super::BlockExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetBandwidthPrices" => {
                    #[allow(non_camel_case_types)]
                    struct GetBandwidthPricesSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetBandwidthPricesSvc<T> {
                        type Response = super::PricesResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_bandwidth_prices(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBandwidthPricesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetEnergyPrices" => {
                    #[allow(non_camel_case_types)]
                    struct GetEnergyPricesSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetEnergyPricesSvc<T> {
                        type Response = super::PricesResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_energy_prices(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEnergyPricesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Wallet/GetMemoFee" => {
                    #[allow(non_camel_case_types)]
                    struct GetMemoFeeSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::EmptyMessage>
                    for GetMemoFeeSvc<T> {
                        type Response = super::PricesResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_memo_fee(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMemoFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Wallet> Clone for WalletServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Wallet> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Wallet> tonic::server::NamedService for WalletServer<T> {
        const NAME: &'static str = "protocol.Wallet";
    }
}
/// Generated client implementations.
pub mod wallet_solidity_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WalletSolidityClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletSolidityClient<tonic::transport::Channel> {
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
    impl<T> WalletSolidityClient<T>
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
        ) -> WalletSolidityClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletSolidityClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAccountById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetAccountById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_witnesses(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::WitnessList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ListWitnesses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "ListWitnesses"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetAssetIssueList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_paginated_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetPaginatedAssetIssueList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetPaginatedAssetIssueList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetAssetIssueByName"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_list_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueListByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetAssetIssueListByName"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_issue_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetAssetIssueById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_now_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetNowBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetNowBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_now_block2(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetNowBlock2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetNowBlock2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBlockByNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetBlockByNum"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_num2(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBlockByNum2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetBlockByNum2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_count_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionCountByBlockNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetTransactionCountByBlockNum",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetDelegatedResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetDelegatedResource"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetDelegatedResourceV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetDelegatedResourceV2"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource_account_index(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetDelegatedResourceAccountIndex",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetDelegatedResourceAccountIndex",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_delegated_resource_account_index_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetDelegatedResourceAccountIndexV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetDelegatedResourceAccountIndexV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_can_delegated_max_size(
            &mut self,
            request: impl tonic::IntoRequest<super::CanDelegatedMaxSizeRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::CanDelegatedMaxSizeResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetCanDelegatedMaxSize",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetCanDelegatedMaxSize"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_available_unfreeze_count(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetAvailableUnfreezeCountRequestMessage,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetAvailableUnfreezeCountResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAvailableUnfreezeCount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetAvailableUnfreezeCount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_can_withdraw_unfreeze_amount(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CanWithdrawUnfreezeAmountRequestMessage,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CanWithdrawUnfreezeAmountResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetCanWithdrawUnfreezeAmount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetCanWithdrawUnfreezeAmount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_exchange_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Exchange>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetExchangeById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetExchangeById"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ExchangeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ListExchanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "ListExchanges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetTransactionById"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_info_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionInfoById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetTransactionInfoById"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_merkle_tree_voucher_info(
            &mut self,
            request: impl tonic::IntoRequest<super::OutputPointInfo>,
        ) -> std::result::Result<
            tonic::Response<super::IncrementalMerkleVoucherInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMerkleTreeVoucherInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetMerkleTreeVoucherInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanNoteByIvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "ScanNoteByIvk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_and_mark_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptAndMarkParameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesMarked>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanAndMarkNoteByIvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "ScanAndMarkNoteByIvk"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_note_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanNoteByOvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "ScanNoteByOvk"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_spend(
            &mut self,
            request: impl tonic::IntoRequest<super::NoteParameters>,
        ) -> std::result::Result<tonic::Response<super::SpendResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/IsSpend",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "IsSpend"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanShieldedTRC20NotesByIvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "ScanShieldedTRC20NotesByIvk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanShieldedTRC20NotesByOvk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "ScanShieldedTRC20NotesByOvk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn is_shielded_trc20_contract_note_spent(
            &mut self,
            request: impl tonic::IntoRequest<super::NfTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::NullifierResult>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/IsShieldedTRC20ContractNoteSpent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "IsShieldedTRC20ContractNoteSpent",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_reward_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetRewardInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetRewardInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_brokerage_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBrokerageInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetBrokerageInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn trigger_constant_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/TriggerConstantContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "TriggerConstantContract"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_energy(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateEnergyMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/EstimateEnergy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "EstimateEnergy"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transaction_info_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfoList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionInfoByBlockNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetTransactionInfoByBlockNum",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_order_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::MarketOrder>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketOrderById",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetMarketOrderById"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_order_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketOrderByAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetMarketOrderByAccount"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_price_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> std::result::Result<
            tonic::Response<super::MarketPriceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketPriceByPair",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetMarketPriceByPair"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_order_list_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketOrderListByPair",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletSolidity",
                        "GetMarketOrderListByPair",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_market_pair_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderPairList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketPairList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetMarketPairList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_burn_trx(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBurnTrx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetBurnTrx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockReq>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_bandwidth_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBandwidthPrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletSolidity", "GetBandwidthPrices"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_energy_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetEnergyPrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.WalletSolidity", "GetEnergyPrices"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod wallet_solidity_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WalletSolidityServer.
    #[async_trait]
    pub trait WalletSolidity: Send + Sync + 'static {
        async fn get_account(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status>;
        async fn get_account_by_id(
            &self,
            request: tonic::Request<super::Account>,
        ) -> std::result::Result<tonic::Response<super::Account>, tonic::Status>;
        async fn list_witnesses(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::WitnessList>, tonic::Status>;
        async fn get_asset_issue_list(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn get_paginated_asset_issue_list(
            &self,
            request: tonic::Request<super::PaginatedMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn get_asset_issue_by_name(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        >;
        async fn get_asset_issue_list_by_name(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::AssetIssueList>, tonic::Status>;
        async fn get_asset_issue_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::AssetIssueContract>,
            tonic::Status,
        >;
        async fn get_now_block(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_now_block2(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status>;
        async fn get_block_by_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_block_by_num2(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status>;
        async fn get_transaction_count_by_block_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_delegated_resource(
            &self,
            request: tonic::Request<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        >;
        async fn get_delegated_resource_v2(
            &self,
            request: tonic::Request<super::DelegatedResourceMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceList>,
            tonic::Status,
        >;
        async fn get_delegated_resource_account_index(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        >;
        async fn get_delegated_resource_account_index_v2(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        >;
        async fn get_can_delegated_max_size(
            &self,
            request: tonic::Request<super::CanDelegatedMaxSizeRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::CanDelegatedMaxSizeResponseMessage>,
            tonic::Status,
        >;
        async fn get_available_unfreeze_count(
            &self,
            request: tonic::Request<super::GetAvailableUnfreezeCountRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::GetAvailableUnfreezeCountResponseMessage>,
            tonic::Status,
        >;
        async fn get_can_withdraw_unfreeze_amount(
            &self,
            request: tonic::Request<super::CanWithdrawUnfreezeAmountRequestMessage>,
        ) -> std::result::Result<
            tonic::Response<super::CanWithdrawUnfreezeAmountResponseMessage>,
            tonic::Status,
        >;
        async fn get_exchange_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Exchange>, tonic::Status>;
        async fn list_exchanges(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::ExchangeList>, tonic::Status>;
        async fn get_transaction_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::Transaction>, tonic::Status>;
        async fn get_transaction_info_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::TransactionInfo>, tonic::Status>;
        async fn get_merkle_tree_voucher_info(
            &self,
            request: tonic::Request<super::OutputPointInfo>,
        ) -> std::result::Result<
            tonic::Response<super::IncrementalMerkleVoucherInfo>,
            tonic::Status,
        >;
        async fn scan_note_by_ivk(
            &self,
            request: tonic::Request<super::IvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status>;
        async fn scan_and_mark_note_by_ivk(
            &self,
            request: tonic::Request<super::IvkDecryptAndMarkParameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesMarked>,
            tonic::Status,
        >;
        async fn scan_note_by_ovk(
            &self,
            request: tonic::Request<super::OvkDecryptParameters>,
        ) -> std::result::Result<tonic::Response<super::DecryptNotes>, tonic::Status>;
        async fn is_spend(
            &self,
            request: tonic::Request<super::NoteParameters>,
        ) -> std::result::Result<tonic::Response<super::SpendResult>, tonic::Status>;
        async fn scan_shielded_trc20_notes_by_ivk(
            &self,
            request: tonic::Request<super::IvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        >;
        async fn scan_shielded_trc20_notes_by_ovk(
            &self,
            request: tonic::Request<super::OvkDecryptTrc20Parameters>,
        ) -> std::result::Result<
            tonic::Response<super::DecryptNotesTrc20>,
            tonic::Status,
        >;
        async fn is_shielded_trc20_contract_note_spent(
            &self,
            request: tonic::Request<super::NfTrc20Parameters>,
        ) -> std::result::Result<tonic::Response<super::NullifierResult>, tonic::Status>;
        async fn get_reward_info(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_brokerage_info(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn trigger_constant_contract(
            &self,
            request: tonic::Request<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionExtention>,
            tonic::Status,
        >;
        async fn estimate_energy(
            &self,
            request: tonic::Request<super::TriggerSmartContract>,
        ) -> std::result::Result<
            tonic::Response<super::EstimateEnergyMessage>,
            tonic::Status,
        >;
        async fn get_transaction_info_by_block_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionInfoList>,
            tonic::Status,
        >;
        async fn get_market_order_by_id(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::MarketOrder>, tonic::Status>;
        async fn get_market_order_by_account(
            &self,
            request: tonic::Request<super::BytesMessage>,
        ) -> std::result::Result<tonic::Response<super::MarketOrderList>, tonic::Status>;
        async fn get_market_price_by_pair(
            &self,
            request: tonic::Request<super::MarketOrderPair>,
        ) -> std::result::Result<tonic::Response<super::MarketPriceList>, tonic::Status>;
        async fn get_market_order_list_by_pair(
            &self,
            request: tonic::Request<super::MarketOrderPair>,
        ) -> std::result::Result<tonic::Response<super::MarketOrderList>, tonic::Status>;
        async fn get_market_pair_list(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MarketOrderPairList>,
            tonic::Status,
        >;
        async fn get_burn_trx(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::NumberMessage>, tonic::Status>;
        async fn get_block(
            &self,
            request: tonic::Request<super::BlockReq>,
        ) -> std::result::Result<tonic::Response<super::BlockExtention>, tonic::Status>;
        async fn get_bandwidth_prices(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        >;
        async fn get_energy_prices(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::PricesResponseMessage>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WalletSolidityServer<T: WalletSolidity> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WalletSolidity> WalletSolidityServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletSolidityServer<T>
    where
        T: WalletSolidity,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/protocol.WalletSolidity/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<T: WalletSolidity> tonic::server::UnaryService<super::Account>
                    for GetAccountSvc<T> {
                        type Response = super::Account;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetAccountById" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountByIdSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<T: WalletSolidity> tonic::server::UnaryService<super::Account>
                    for GetAccountByIdSvc<T> {
                        type Response = super::Account;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Account>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_account_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ListWitnesses" => {
                    #[allow(non_camel_case_types)]
                    struct ListWitnessesSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for ListWitnessesSvc<T> {
                        type Response = super::WitnessList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_witnesses(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWitnessesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetAssetIssueList" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueListSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetAssetIssueListSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetPaginatedAssetIssueList" => {
                    #[allow(non_camel_case_types)]
                    struct GetPaginatedAssetIssueListSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::PaginatedMessage>
                    for GetPaginatedAssetIssueListSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PaginatedMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_paginated_asset_issue_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPaginatedAssetIssueListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetAssetIssueByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueByNameSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetAssetIssueByNameSvc<T> {
                        type Response = super::AssetIssueContract;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_by_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetAssetIssueListByName" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueListByNameSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetAssetIssueListByNameSvc<T> {
                        type Response = super::AssetIssueList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_list_by_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueListByNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetAssetIssueById" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetIssueByIdSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetAssetIssueByIdSvc<T> {
                        type Response = super::AssetIssueContract;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_asset_issue_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAssetIssueByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetNowBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetNowBlockSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetNowBlockSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_now_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNowBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetNowBlock2" => {
                    #[allow(non_camel_case_types)]
                    struct GetNowBlock2Svc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetNowBlock2Svc<T> {
                        type Response = super::BlockExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_now_block2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNowBlock2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetBlockByNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNumSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByNumSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetBlockByNum2" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNum2Svc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByNum2Svc<T> {
                        type Response = super::BlockExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_num2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByNum2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetTransactionCountByBlockNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionCountByBlockNumSvc<T: WalletSolidity>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::NumberMessage>
                    for GetTransactionCountByBlockNumSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_count_by_block_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionCountByBlockNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetDelegatedResource" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::DelegatedResourceMessage>
                    for GetDelegatedResourceSvc<T> {
                        type Response = super::DelegatedResourceList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelegatedResourceMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_delegated_resource(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetDelegatedResourceV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceV2Svc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::DelegatedResourceMessage>
                    for GetDelegatedResourceV2Svc<T> {
                        type Response = super::DelegatedResourceList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelegatedResourceMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_delegated_resource_v2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetDelegatedResourceAccountIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceAccountIndexSvc<T: WalletSolidity>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetDelegatedResourceAccountIndexSvc<T> {
                        type Response = super::DelegatedResourceAccountIndex;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_delegated_resource_account_index(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceAccountIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetDelegatedResourceAccountIndexV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetDelegatedResourceAccountIndexV2Svc<T: WalletSolidity>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetDelegatedResourceAccountIndexV2Svc<T> {
                        type Response = super::DelegatedResourceAccountIndex;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_delegated_resource_account_index_v2(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDelegatedResourceAccountIndexV2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetCanDelegatedMaxSize" => {
                    #[allow(non_camel_case_types)]
                    struct GetCanDelegatedMaxSizeSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<
                        super::CanDelegatedMaxSizeRequestMessage,
                    > for GetCanDelegatedMaxSizeSvc<T> {
                        type Response = super::CanDelegatedMaxSizeResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CanDelegatedMaxSizeRequestMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_can_delegated_max_size(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCanDelegatedMaxSizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetAvailableUnfreezeCount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableUnfreezeCountSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<
                        super::GetAvailableUnfreezeCountRequestMessage,
                    > for GetAvailableUnfreezeCountSvc<T> {
                        type Response = super::GetAvailableUnfreezeCountResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAvailableUnfreezeCountRequestMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_unfreeze_count(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAvailableUnfreezeCountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetCanWithdrawUnfreezeAmount" => {
                    #[allow(non_camel_case_types)]
                    struct GetCanWithdrawUnfreezeAmountSvc<T: WalletSolidity>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<
                        super::CanWithdrawUnfreezeAmountRequestMessage,
                    > for GetCanWithdrawUnfreezeAmountSvc<T> {
                        type Response = super::CanWithdrawUnfreezeAmountResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CanWithdrawUnfreezeAmountRequestMessage,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_can_withdraw_unfreeze_amount(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCanWithdrawUnfreezeAmountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetExchangeById" => {
                    #[allow(non_camel_case_types)]
                    struct GetExchangeByIdSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetExchangeByIdSvc<T> {
                        type Response = super::Exchange;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_exchange_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetExchangeByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ListExchanges" => {
                    #[allow(non_camel_case_types)]
                    struct ListExchangesSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for ListExchangesSvc<T> {
                        type Response = super::ExchangeList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_exchanges(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListExchangesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetTransactionById" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionByIdSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetTransactionByIdSvc<T> {
                        type Response = super::Transaction;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetTransactionInfoById" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionInfoByIdSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetTransactionInfoByIdSvc<T> {
                        type Response = super::TransactionInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_info_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionInfoByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetMerkleTreeVoucherInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetMerkleTreeVoucherInfoSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::OutputPointInfo>
                    for GetMerkleTreeVoucherInfoSvc<T> {
                        type Response = super::IncrementalMerkleVoucherInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OutputPointInfo>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_merkle_tree_voucher_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMerkleTreeVoucherInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ScanNoteByIvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanNoteByIvkSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::IvkDecryptParameters>
                    for ScanNoteByIvkSvc<T> {
                        type Response = super::DecryptNotes;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IvkDecryptParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_note_by_ivk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanNoteByIvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ScanAndMarkNoteByIvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanAndMarkNoteByIvkSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::IvkDecryptAndMarkParameters>
                    for ScanAndMarkNoteByIvkSvc<T> {
                        type Response = super::DecryptNotesMarked;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IvkDecryptAndMarkParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_and_mark_note_by_ivk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanAndMarkNoteByIvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ScanNoteByOvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanNoteByOvkSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::OvkDecryptParameters>
                    for ScanNoteByOvkSvc<T> {
                        type Response = super::DecryptNotes;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OvkDecryptParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_note_by_ovk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanNoteByOvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/IsSpend" => {
                    #[allow(non_camel_case_types)]
                    struct IsSpendSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::NoteParameters>
                    for IsSpendSvc<T> {
                        type Response = super::SpendResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NoteParameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).is_spend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsSpendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ScanShieldedTRC20NotesByIvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanShieldedTRC20NotesByIvkSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::IvkDecryptTrc20Parameters>
                    for ScanShieldedTRC20NotesByIvkSvc<T> {
                        type Response = super::DecryptNotesTrc20;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IvkDecryptTrc20Parameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_shielded_trc20_notes_by_ivk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanShieldedTRC20NotesByIvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/ScanShieldedTRC20NotesByOvk" => {
                    #[allow(non_camel_case_types)]
                    struct ScanShieldedTRC20NotesByOvkSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::OvkDecryptTrc20Parameters>
                    for ScanShieldedTRC20NotesByOvkSvc<T> {
                        type Response = super::DecryptNotesTrc20;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OvkDecryptTrc20Parameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).scan_shielded_trc20_notes_by_ovk(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScanShieldedTRC20NotesByOvkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/IsShieldedTRC20ContractNoteSpent" => {
                    #[allow(non_camel_case_types)]
                    struct IsShieldedTRC20ContractNoteSpentSvc<T: WalletSolidity>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::NfTrc20Parameters>
                    for IsShieldedTRC20ContractNoteSpentSvc<T> {
                        type Response = super::NullifierResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NfTrc20Parameters>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .is_shielded_trc20_contract_note_spent(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsShieldedTRC20ContractNoteSpentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetRewardInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetRewardInfoSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetRewardInfoSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_reward_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRewardInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetBrokerageInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetBrokerageInfoSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetBrokerageInfoSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_brokerage_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBrokerageInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/TriggerConstantContract" => {
                    #[allow(non_camel_case_types)]
                    struct TriggerConstantContractSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::TriggerSmartContract>
                    for TriggerConstantContractSvc<T> {
                        type Response = super::TransactionExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerSmartContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).trigger_constant_contract(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TriggerConstantContractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/EstimateEnergy" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateEnergySvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::TriggerSmartContract>
                    for EstimateEnergySvc<T> {
                        type Response = super::EstimateEnergyMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerSmartContract>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_energy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateEnergySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetTransactionInfoByBlockNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionInfoByBlockNumSvc<T: WalletSolidity>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::NumberMessage>
                    for GetTransactionInfoByBlockNumSvc<T> {
                        type Response = super::TransactionInfoList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transaction_info_by_block_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionInfoByBlockNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetMarketOrderById" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrderByIdSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetMarketOrderByIdSvc<T> {
                        type Response = super::MarketOrder;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_order_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketOrderByIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetMarketOrderByAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrderByAccountSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::BytesMessage>
                    for GetMarketOrderByAccountSvc<T> {
                        type Response = super::MarketOrderList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BytesMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_order_by_account(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketOrderByAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetMarketPriceByPair" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketPriceByPairSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::MarketOrderPair>
                    for GetMarketPriceByPairSvc<T> {
                        type Response = super::MarketPriceList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketOrderPair>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_price_by_pair(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketPriceByPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetMarketOrderListByPair" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketOrderListByPairSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::MarketOrderPair>
                    for GetMarketOrderListByPairSvc<T> {
                        type Response = super::MarketOrderList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketOrderPair>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_order_list_by_pair(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketOrderListByPairSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetMarketPairList" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketPairListSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetMarketPairListSvc<T> {
                        type Response = super::MarketOrderPairList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_pair_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketPairListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetBurnTrx" => {
                    #[allow(non_camel_case_types)]
                    struct GetBurnTrxSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetBurnTrxSvc<T> {
                        type Response = super::NumberMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_burn_trx(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBurnTrxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<T: WalletSolidity> tonic::server::UnaryService<super::BlockReq>
                    for GetBlockSvc<T> {
                        type Response = super::BlockExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetBandwidthPrices" => {
                    #[allow(non_camel_case_types)]
                    struct GetBandwidthPricesSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetBandwidthPricesSvc<T> {
                        type Response = super::PricesResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_bandwidth_prices(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBandwidthPricesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletSolidity/GetEnergyPrices" => {
                    #[allow(non_camel_case_types)]
                    struct GetEnergyPricesSvc<T: WalletSolidity>(pub Arc<T>);
                    impl<
                        T: WalletSolidity,
                    > tonic::server::UnaryService<super::EmptyMessage>
                    for GetEnergyPricesSvc<T> {
                        type Response = super::PricesResponseMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_energy_prices(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEnergyPricesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: WalletSolidity> Clone for WalletSolidityServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: WalletSolidity> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WalletSolidity> tonic::server::NamedService for WalletSolidityServer<T> {
        const NAME: &'static str = "protocol.WalletSolidity";
    }
}
/// Generated client implementations.
pub mod wallet_extension_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WalletExtensionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletExtensionClient<tonic::transport::Channel> {
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
    impl<T> WalletExtensionClient<T>
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
        ) -> WalletExtensionClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletExtensionClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_transactions_from_this(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsFromThis",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletExtension",
                        "GetTransactionsFromThis",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transactions_from_this2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionListExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsFromThis2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protocol.WalletExtension",
                        "GetTransactionsFromThis2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transactions_to_this(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsToThis",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletExtension", "GetTransactionsToThis"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_transactions_to_this2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionListExtention>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsToThis2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protocol.WalletExtension", "GetTransactionsToThis2"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod wallet_extension_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WalletExtensionServer.
    #[async_trait]
    pub trait WalletExtension: Send + Sync + 'static {
        async fn get_transactions_from_this(
            &self,
            request: tonic::Request<super::AccountPaginated>,
        ) -> std::result::Result<tonic::Response<super::TransactionList>, tonic::Status>;
        async fn get_transactions_from_this2(
            &self,
            request: tonic::Request<super::AccountPaginated>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionListExtention>,
            tonic::Status,
        >;
        async fn get_transactions_to_this(
            &self,
            request: tonic::Request<super::AccountPaginated>,
        ) -> std::result::Result<tonic::Response<super::TransactionList>, tonic::Status>;
        async fn get_transactions_to_this2(
            &self,
            request: tonic::Request<super::AccountPaginated>,
        ) -> std::result::Result<
            tonic::Response<super::TransactionListExtention>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WalletExtensionServer<T: WalletExtension> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WalletExtension> WalletExtensionServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletExtensionServer<T>
    where
        T: WalletExtension,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/protocol.WalletExtension/GetTransactionsFromThis" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsFromThisSvc<T: WalletExtension>(pub Arc<T>);
                    impl<
                        T: WalletExtension,
                    > tonic::server::UnaryService<super::AccountPaginated>
                    for GetTransactionsFromThisSvc<T> {
                        type Response = super::TransactionList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountPaginated>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_from_this(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsFromThisSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletExtension/GetTransactionsFromThis2" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsFromThis2Svc<T: WalletExtension>(pub Arc<T>);
                    impl<
                        T: WalletExtension,
                    > tonic::server::UnaryService<super::AccountPaginated>
                    for GetTransactionsFromThis2Svc<T> {
                        type Response = super::TransactionListExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountPaginated>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_from_this2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsFromThis2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletExtension/GetTransactionsToThis" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsToThisSvc<T: WalletExtension>(pub Arc<T>);
                    impl<
                        T: WalletExtension,
                    > tonic::server::UnaryService<super::AccountPaginated>
                    for GetTransactionsToThisSvc<T> {
                        type Response = super::TransactionList;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountPaginated>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_to_this(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsToThisSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.WalletExtension/GetTransactionsToThis2" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsToThis2Svc<T: WalletExtension>(pub Arc<T>);
                    impl<
                        T: WalletExtension,
                    > tonic::server::UnaryService<super::AccountPaginated>
                    for GetTransactionsToThis2Svc<T> {
                        type Response = super::TransactionListExtention;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccountPaginated>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_to_this2(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsToThis2Svc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: WalletExtension> Clone for WalletExtensionServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: WalletExtension> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WalletExtension> tonic::server::NamedService for WalletExtensionServer<T> {
        const NAME: &'static str = "protocol.WalletExtension";
    }
}
/// Generated client implementations.
pub mod database_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DatabaseClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatabaseClient<tonic::transport::Channel> {
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
    impl<T> DatabaseClient<T>
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
        ) -> DatabaseClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DatabaseClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_block_reference(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockReference>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/getBlockReference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Database", "getBlockReference"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_dynamic_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DynamicProperties>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/GetDynamicProperties",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Database", "GetDynamicProperties"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_now_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/GetNowBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Database", "GetNowBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_block_by_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/GetBlockByNum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Database", "GetBlockByNum"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod database_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DatabaseServer.
    #[async_trait]
    pub trait Database: Send + Sync + 'static {
        async fn get_block_reference(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::BlockReference>, tonic::Status>;
        async fn get_dynamic_properties(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<
            tonic::Response<super::DynamicProperties>,
            tonic::Status,
        >;
        async fn get_now_block(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
        async fn get_block_by_num(
            &self,
            request: tonic::Request<super::NumberMessage>,
        ) -> std::result::Result<tonic::Response<super::Block>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DatabaseServer<T: Database> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Database> DatabaseServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DatabaseServer<T>
    where
        T: Database,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/protocol.Database/getBlockReference" => {
                    #[allow(non_camel_case_types)]
                    struct getBlockReferenceSvc<T: Database>(pub Arc<T>);
                    impl<T: Database> tonic::server::UnaryService<super::EmptyMessage>
                    for getBlockReferenceSvc<T> {
                        type Response = super::BlockReference;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_reference(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getBlockReferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Database/GetDynamicProperties" => {
                    #[allow(non_camel_case_types)]
                    struct GetDynamicPropertiesSvc<T: Database>(pub Arc<T>);
                    impl<T: Database> tonic::server::UnaryService<super::EmptyMessage>
                    for GetDynamicPropertiesSvc<T> {
                        type Response = super::DynamicProperties;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_dynamic_properties(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDynamicPropertiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Database/GetNowBlock" => {
                    #[allow(non_camel_case_types)]
                    struct GetNowBlockSvc<T: Database>(pub Arc<T>);
                    impl<T: Database> tonic::server::UnaryService<super::EmptyMessage>
                    for GetNowBlockSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_now_block(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNowBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protocol.Database/GetBlockByNum" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByNumSvc<T: Database>(pub Arc<T>);
                    impl<T: Database> tonic::server::UnaryService<super::NumberMessage>
                    for GetBlockByNumSvc<T> {
                        type Response = super::Block;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NumberMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_by_num(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByNumSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Database> Clone for DatabaseServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Database> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Database> tonic::server::NamedService for DatabaseServer<T> {
        const NAME: &'static str = "protocol.Database";
    }
}
/// Generated client implementations.
pub mod monitor_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MonitorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MonitorClient<tonic::transport::Channel> {
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
    impl<T> MonitorClient<T>
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
        ) -> MonitorClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MonitorClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_stats_info(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::MetricsInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Monitor/GetStatsInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.Monitor", "GetStatsInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod monitor_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MonitorServer.
    #[async_trait]
    pub trait Monitor: Send + Sync + 'static {
        async fn get_stats_info(
            &self,
            request: tonic::Request<super::EmptyMessage>,
        ) -> std::result::Result<tonic::Response<super::MetricsInfo>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MonitorServer<T: Monitor> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Monitor> MonitorServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MonitorServer<T>
    where
        T: Monitor,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/protocol.Monitor/GetStatsInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatsInfoSvc<T: Monitor>(pub Arc<T>);
                    impl<T: Monitor> tonic::server::UnaryService<super::EmptyMessage>
                    for GetStatsInfoSvc<T> {
                        type Response = super::MetricsInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_stats_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatsInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Monitor> Clone for MonitorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Monitor> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Monitor> tonic::server::NamedService for MonitorServer<T> {
        const NAME: &'static str = "protocol.Monitor";
    }
}
/// Generated client implementations.
pub mod network_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NetworkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkClient<tonic::transport::Channel> {
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
    impl<T> NetworkClient<T>
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
        ) -> NetworkClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NetworkClient::new(InterceptedService::new(inner, interceptor))
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
    }
}
/// Generated server implementations.
pub mod network_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NetworkServer.
    #[async_trait]
    pub trait Network: Send + Sync + 'static {}
    #[derive(Debug)]
    pub struct NetworkServer<T: Network> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Network> NetworkServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NetworkServer<T>
    where
        T: Network,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Network> Clone for NetworkServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Network> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Network> tonic::server::NamedService for NetworkServer<T> {
        const NAME: &'static str = "protocol.Network";
    }
}
/// Generated client implementations.
pub mod tron_zksnark_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TronZksnarkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TronZksnarkClient<tonic::transport::Channel> {
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
    impl<T> TronZksnarkClient<T>
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
        ) -> TronZksnarkClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TronZksnarkClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn check_zksnark_proof(
            &mut self,
            request: impl tonic::IntoRequest<super::ZksnarkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ZksnarkResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.TronZksnark/CheckZksnarkProof",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protocol.TronZksnark", "CheckZksnarkProof"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod tron_zksnark_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TronZksnarkServer.
    #[async_trait]
    pub trait TronZksnark: Send + Sync + 'static {
        async fn check_zksnark_proof(
            &self,
            request: tonic::Request<super::ZksnarkRequest>,
        ) -> std::result::Result<tonic::Response<super::ZksnarkResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TronZksnarkServer<T: TronZksnark> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TronZksnark> TronZksnarkServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TronZksnarkServer<T>
    where
        T: TronZksnark,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/protocol.TronZksnark/CheckZksnarkProof" => {
                    #[allow(non_camel_case_types)]
                    struct CheckZksnarkProofSvc<T: TronZksnark>(pub Arc<T>);
                    impl<
                        T: TronZksnark,
                    > tonic::server::UnaryService<super::ZksnarkRequest>
                    for CheckZksnarkProofSvc<T> {
                        type Response = super::ZksnarkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ZksnarkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).check_zksnark_proof(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckZksnarkProofSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TronZksnark> Clone for TronZksnarkServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: TronZksnark> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TronZksnark> tonic::server::NamedService for TronZksnarkServer<T> {
        const NAME: &'static str = "protocol.TronZksnark";
    }
}
