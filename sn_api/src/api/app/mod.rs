// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

mod auth;
mod consts;
mod helpers;
mod keys;
mod nrs;
mod realpath;
mod safe_client;
mod sequence;
#[cfg(test)]
mod test_helpers;
mod xorurl_media_types;

use super::common;
use super::constants;
use safe_client::SafeAppClient;
use xorurl::XorUrlBase;

// The following is what's meant to be the public API

pub mod fetch;
pub mod files;
pub mod nrs_map;
pub mod wallet;
pub mod xorurl;
use super::Result;
pub use consts::DEFAULT_XORURL_BASE;
pub use helpers::parse_coins_amount;
pub use nrs::ProcessedEntries;
use sn_data_types::Keypair;
use std::sync::Arc;
pub use xor_name::{XorName, XOR_NAME_LEN};

// TODO: should we be cloning this?
#[derive(Clone)]
pub struct Safe {
    safe_client: SafeAppClient,
    pub xorurl_base: XorUrlBase,
}

impl Default for Safe {
    fn default() -> Self {
        Self::new(Some(DEFAULT_XORURL_BASE))
    }
}

impl Safe {
    pub fn new(xorurl_base: Option<XorUrlBase>) -> Self {
        Self {
            safe_client: SafeAppClient::new(),
            xorurl_base: xorurl_base.unwrap_or_else(|| DEFAULT_XORURL_BASE),
        }
    }

    pub async fn keypair(&self) -> Result<Arc<Keypair>> {
        self.safe_client.keypair().await
    }
}
