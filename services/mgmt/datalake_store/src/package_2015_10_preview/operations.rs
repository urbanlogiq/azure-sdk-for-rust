#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use snafu::{ResultExt, Snafu};
pub mod account {
    use crate::models::*;
    use snafu::{ResultExt, Snafu};
    pub async fn get_firewall_rule(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        firewall_rule_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<FirewallRule, get_firewall_rule::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name,
            firewall_rule_name
        );
        let mut url = url::Url::parse(url_str).context(get_firewall_rule::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get_firewall_rule::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get_firewall_rule::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(get_firewall_rule::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: FirewallRule =
                    serde_json::from_slice(rsp_body).context(get_firewall_rule::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                get_firewall_rule::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod get_firewall_rule {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete_firewall_rule(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        firewall_rule_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete_firewall_rule::Response, delete_firewall_rule::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name,
            firewall_rule_name
        );
        let mut url = url::Url::parse(url_str).context(delete_firewall_rule::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(delete_firewall_rule::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(delete_firewall_rule::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(delete_firewall_rule::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete_firewall_rule::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete_firewall_rule::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                delete_firewall_rule::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod delete_firewall_rule {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_firewall_rules(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreFirewallRuleListResult, list_firewall_rules::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(list_firewall_rules::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_firewall_rules::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_firewall_rules::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_firewall_rules::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreFirewallRuleListResult =
                    serde_json::from_slice(rsp_body).context(list_firewall_rules::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list_firewall_rules::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list_firewall_rules {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update_firewall_rule(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        name: &str,
        parameters: &FirewallRule,
        subscription_id: &str,
    ) -> std::result::Result<FirewallRule, create_or_update_firewall_rule::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name,
            name
        );
        let mut url = url::Url::parse(url_str).context(create_or_update_firewall_rule::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(create_or_update_firewall_rule::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).context(create_or_update_firewall_rule::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .context(create_or_update_firewall_rule::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(create_or_update_firewall_rule::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: FirewallRule = serde_json::from_slice(rsp_body)
                    .context(create_or_update_firewall_rule::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                create_or_update_firewall_rule::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod create_or_update_firewall_rule {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &DataLakeStoreAccount,
        subscription_id: &str,
    ) -> std::result::Result<create::Response, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            name
        );
        let mut url = url::Url::parse(url_str).context(create::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(create::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).context(create::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(create::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccount =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(create::Response::Created201(rsp_value))
            }
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccount =
                    serde_json::from_slice(rsp_body).context(create::DeserializeError { body: rsp_body.clone() })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                create::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(DataLakeStoreAccount),
            Ok200(DataLakeStoreAccount),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &DataLakeStoreAccount,
        subscription_id: &str,
    ) -> std::result::Result<update::Response, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            name
        );
        let mut url = url::Url::parse(url_str).context(update::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(update::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(parameters).context(update::SerializeError)?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(update::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccount =
                    serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccount =
                    serde_json::from_slice(rsp_body).context(update::DeserializeError { body: rsp_body.clone() })?;
                Ok(update::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                update::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(DataLakeStoreAccount),
            Created201(DataLakeStoreAccount),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreAccount, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(get::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(get::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccount =
                    serde_json::from_slice(rsp_body).context(get::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                get::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(delete::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(delete::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            http::StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            http::StatusCode::NOT_FOUND => delete::NotFound404 {}.fail(),
            status_code => {
                let rsp_body = rsp.body();
                delete::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            NotFound404 {},
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn enable_key_vault(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<(), enable_key_vault::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/enableKeyVault",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            account_name
        );
        let mut url = url::Url::parse(url_str).context(enable_key_vault::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::POST);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(enable_key_vault::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(enable_key_vault::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(enable_key_vault::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => Ok(()),
            status_code => {
                let rsp_body = rsp.body();
                enable_key_vault::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod enable_key_vault {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        filter: Option<&str>,
        top: Option<i32>,
        skip: Option<i32>,
        expand: Option<&str>,
        select: Option<&str>,
        orderby: Option<&str>,
        count: Option<bool>,
        search: Option<&str>,
        format: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreAccountListResult, list_by_resource_group::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).context(list_by_resource_group::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        if let Some(skip) = skip {
            url.query_pairs_mut().append_pair("$skip", skip.to_string().as_str());
        }
        if let Some(expand) = expand {
            url.query_pairs_mut().append_pair("$expand", expand);
        }
        if let Some(select) = select {
            url.query_pairs_mut().append_pair("$select", select);
        }
        if let Some(orderby) = orderby {
            url.query_pairs_mut().append_pair("$orderby", orderby);
        }
        if let Some(count) = count {
            url.query_pairs_mut().append_pair("$count", count.to_string().as_str());
        }
        if let Some(search) = search {
            url.query_pairs_mut().append_pair("$search", search);
        }
        if let Some(format) = format {
            url.query_pairs_mut().append_pair("$format", format);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list_by_resource_group::BuildRequestError)?;
        let rsp = http_client
            .execute_request(req)
            .await
            .context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccountListResult =
                    serde_json::from_slice(rsp_body).context(list_by_resource_group::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list_by_resource_group::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        filter: Option<&str>,
        top: Option<i32>,
        skip: Option<i32>,
        expand: Option<&str>,
        select: Option<&str>,
        orderby: Option<&str>,
        count: Option<bool>,
        search: Option<&str>,
        format: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreAccountListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.DataLakeStore/accounts",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).context(list::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        if let Some(filter) = filter {
            url.query_pairs_mut().append_pair("$filter", filter);
        }
        if let Some(top) = top {
            url.query_pairs_mut().append_pair("$top", top.to_string().as_str());
        }
        if let Some(skip) = skip {
            url.query_pairs_mut().append_pair("$skip", skip.to_string().as_str());
        }
        if let Some(expand) = expand {
            url.query_pairs_mut().append_pair("$expand", expand);
        }
        if let Some(select) = select {
            url.query_pairs_mut().append_pair("$select", select);
        }
        if let Some(orderby) = orderby {
            url.query_pairs_mut().append_pair("$orderby", orderby);
        }
        if let Some(count) = count {
            url.query_pairs_mut().append_pair("$count", count.to_string().as_str());
        }
        if let Some(search) = search {
            url.query_pairs_mut().append_pair("$search", search);
        }
        if let Some(format) = format {
            url.query_pairs_mut().append_pair("$format", format);
        }
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).context(list::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: DataLakeStoreAccountListResult =
                    serde_json::from_slice(rsp_body).context(list::DeserializeError { body: rsp_body.clone() })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                list::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                }
                .fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            ParseUrlError { source: url::ParseError },
            BuildRequestError { source: http::Error },
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
