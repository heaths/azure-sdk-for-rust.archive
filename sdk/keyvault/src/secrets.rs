// Copyright (c) 2019 Heath Stewart.
// Licensed under the MIT License.

extern crate azure_core;
use azure_core::TokenCredential;
use hyper::Uri;

/// The SecretClient provides synchronous and asynchronous methods to manage secrets in the Azure Key Vault.
pub struct SecretClient<T: TokenCredential> {
    vault_uri: Uri,
    credential: T,
}

impl<T: TokenCredential> SecretClient<T> {
    /// Creates a new SecretClient.
    fn new(vault_uri: &Uri, credential: &T) -> SecretClient<T> {
        SecretClient {
            vault_uri: vault_uri.clone(),
            credential: credential.clone(),
        }
    }

    /// The Uri of the vault used to create this instance of the SecretClient.
    pub fn vault_uri(&self) -> &Uri {
        &self.vault_uri
    }
}
