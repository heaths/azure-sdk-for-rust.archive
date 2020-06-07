// Copyright (c) 2019 Heath Stewart.
// Licensed under the MIT License.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::option::Option;

/// Represents an Azure service bearer access token with expiry information.
#[allow(dead_code)]
pub struct AccessToken {
    /// The access token value.
    token: String,

    /// The time when the provided token expires.
    expires_on: DateTime<Utc>,
}

/// Represents a credential capable of providing an OAuth token.
#[async_trait]
pub trait TokenCredential {
    /// Gets an `AccessToken` for the specified set of scopes.
    async fn get_token(&self, scopes: &[String], parent_request_id: Option<&str>) -> AccessToken;
}
