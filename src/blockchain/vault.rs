use std::collections::{HashMap, HashSet};
use std::fmt;

#[allow(unused_imports)]
use log::{debug, error, info, trace};

use dogecoin::{BlockHeader, Script, Transaction, Txid};
use serde::Deserialize;

use super::*;
use crate::blockchain::utils::{ElectrumLikeSync, ElsGetHistoryRes};
use crate::database::BatchDatabase;
use crate::error::Error;
use crate::FeeRate;

// const DEFAULT_CONCURRENT_REQUESTS: u8 = 4;

#[derive(Debug)]
struct UrlClient {}

#[derive(Debug)]
struct Client {}

#[derive(Debug)]
pub struct TodoBlockchain {}

impl TodoBlockchain {
    /// Create a new instance of the client from a base URL
    pub fn new(_base_url: &str) -> Self {
        todo!()
    }
}

#[maybe_async]
impl Blockchain for TodoBlockchain {
    fn get_capabilities(&self) -> HashSet<Capability> {
        vec![
            Capability::FullHistory,
            Capability::GetAnyTx,
            Capability::AccurateFees,
        ]
        .into_iter()
        .collect()
    }

    fn setup<D: BatchDatabase, P: Progress>(
        &self,
        _database: &mut D,
        _progress_update: P,
    ) -> Result<(), Error> {
        todo!()
    }

    fn get_tx(&self, _txid: &Txid) -> Result<Option<Transaction>, Error> {
        todo!()
    }

    fn broadcast(&self, _tx: &Transaction) -> Result<(), Error> {
        todo!()
    }

    fn get_height(&self) -> Result<u32, Error> {
        todo!()
    }

    fn estimate_fee(&self, _target: usize) -> Result<FeeRate, Error> {
        todo!()
    }
}

#[allow(dead_code)]
impl UrlClient {
    fn script_to_scripthash(_script: &Script) -> String {
        todo!()
    }

    async fn _get_tx(&self, _txid: &Txid) -> Result<Option<Transaction>, EsploraError> {
        todo!()
    }

    async fn _get_tx_no_opt(&self, _txid: &Txid) -> Result<Transaction, EsploraError> {
        todo!()
    }

    async fn _get_header(&self, _block_height: u32) -> Result<BlockHeader, EsploraError> {
        todo!()
    }

    async fn _broadcast(&self, _transaction: &Transaction) -> Result<(), EsploraError> {
        todo!()
    }

    async fn _get_height(&self) -> Result<u32, EsploraError> {
        todo!()
    }

    async fn _script_get_history(
        &self,
        _script: &Script,
    ) -> Result<Vec<ElsGetHistoryRes>, EsploraError> {
        todo!()
    }

    async fn _get_fee_estimates(&self) -> Result<HashMap<String, f64>, EsploraError> {
        todo!()
    }
}

#[derive(Deserialize)]
struct EsploraGetHistoryStatus {
    _block_height: Option<usize>,
}

#[derive(Deserialize)]
struct EsploraGetHistory {
    _txid: Txid,
    _status: EsploraGetHistoryStatus,
}

impl ElectrumLikeSync for Client {
    fn els_batch_script_get_history<'s, I: IntoIterator<Item = &'s Script> + Clone>(
        &self,
        _scripts: I,
    ) -> Result<Vec<Vec<ElsGetHistoryRes>>, Error> {
        todo!()
    }

    fn els_batch_transaction_get<'s, I: IntoIterator<Item = &'s Txid> + Clone>(
        &self,
        _txids: I,
    ) -> Result<Vec<Transaction>, Error> {
        todo!()
    }

    fn els_batch_block_header<I: IntoIterator<Item = u32> + Clone>(
        &self,
        _heights: I,
    ) -> Result<Vec<BlockHeader>, Error> {
        todo!()
    }
}

/// Configuration for an [`TodoBlockchain`]
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct TodoBlockchainConfig {
    /// Base URL of the esplora service
    ///
    /// eg. `https://blockstream.info/api/`
    pub base_url: String,
    /// Number of parallel requests sent to the esplora service (default: 4)
    pub concurrency: Option<u8>,
    /// Stop searching addresses for transactions after finding an unused gap of
    /// this length
    pub stop_gap: usize,
}

impl ConfigurableBlockchain for TodoBlockchain {
    type Config = TodoBlockchainConfig;

    fn from_config(_config: &Self::Config) -> Result<Self, Error> {
        todo!()
    }
}

/// Errors that can happen during a sync with [`TodoBlockchain`]
#[derive(Debug)]
pub enum EsploraError {
    /// Invalid number returned
    Parsing(std::num::ParseIntError),
    /// Invalid Hex data returned
    Hex(dogecoin::hashes::hex::Error),

    /// Transaction not found
    TransactionNotFound(Txid),
    /// Header height not found
    HeaderHeightNotFound(u32),
}

impl fmt::Display for EsploraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EsploraError {}

impl_error!(std::num::ParseIntError, Parsing, EsploraError);
impl_error!(dogecoin::hashes::hex::Error, Hex, EsploraError);

#[cfg(test)]
#[cfg(feature = "test-esplora")]
crate::bdk_blockchain_tests! {
    fn test_instance(test_client: &TestClient) -> TodoBlockchain {
        TodoBlockchain::new(&format!("http://{}",test_client.electrsd.esplora_url.as_ref().unwrap()), None, 20)
    }
}
