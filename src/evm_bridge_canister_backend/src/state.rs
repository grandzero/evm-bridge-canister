use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};
const MAX_VALUE_SIZE: u32 = 1000000;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum ChainSelection {
    #[serde(rename = "mumbai")]
    Mumbai,
    #[serde(rename = "binance")]
    Binance,
}

#[derive(CandidType, Serialize, Debug, Clone, Deserialize)]
pub struct Transaction {
    pub data: Vec<u8>,
    pub timestamp: u64,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            data: vec![],
            timestamp: u64::from(0 as u64),
        }
    }
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct TransactionChainData {
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

impl Default for TransactionChainData {
    fn default() -> Self {
        TransactionChainData {
            nonce: 0 as u64,
            transactions: vec![],
        }
    }
}
#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct UserData {
    pub public_key: Vec<u8>,
    pub transactions: HashMap<u64, TransactionChainData>,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub enum Environment {
    #[serde(rename = "Development")]
    Development,
    #[serde(rename = "Staging")]
    Staging,
    #[serde(rename = "Production")]
    Production,
}

impl Storable for Environment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE, // Replace with the actual max size
        is_fixed_size: false,
    };
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub key_name: String,
    pub sign_cycles: u64,
    pub rpc_url: String,
    pub owner: Principal,
    pub public_key_str: String,
    pub rpc_canister: String,
    pub owner_public_key: Vec<u8>,
    pub rpc_endpoints: Vec<RPCEndpoints>,
    pub mumbai_contract: String,
    pub binance_contract: String, // TODO: Create new struct for all evm networks and it should store a list with all necessary data
}

impl Default for Config {
    fn default() -> Self {
        Self::from(Environment::Development)
    }
}

impl From<Environment> for Config {
    fn from(env: Environment) -> Self {
        if env == Environment::Staging {
            Self {
                env: Environment::Staging,
                key_name: "test_key_1".to_string(),
                sign_cycles: 10_000_000_000,
                rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                owner: Principal::anonymous(),
                public_key_str: "".to_string(),
                rpc_canister: "".to_string(),
                owner_public_key: vec![],
                rpc_endpoints: vec![
                    RPCEndpoints {
                        rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                        rpc_name: "mumbai".to_owned(),
                    },
                    RPCEndpoints {
                        rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_owned(),
                        rpc_name: "binance".to_owned(),
                    },
                ],
                mumbai_contract: "0x1d5989e6f450BBd9385Ce98922C0218af4c9aE97".to_string(),
                binance_contract: "0x4FF62fC53b4fCD08428aE79a59B36A5Ba9235817".to_string(),
            }
        } else if env == Environment::Production {
            Self {
                env: Environment::Production,
                key_name: "key_1".to_string(),
                sign_cycles: 26_153_846_153,
                rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                owner: Principal::anonymous(),
                public_key_str: "".to_string(),
                rpc_canister: "".to_string(),
                owner_public_key: vec![],
                rpc_endpoints: vec![
                    RPCEndpoints {
                        rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                        rpc_name: "mumbai".to_owned(),
                    },
                    RPCEndpoints {
                        rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_owned(),
                        rpc_name: "binance".to_owned(),
                    },
                ],
                mumbai_contract: "0x1d5989e6f450BBd9385Ce98922C0218af4c9aE97".to_string(),
                binance_contract: "0x4FF62fC53b4fCD08428aE79a59B36A5Ba9235817".to_string(),
            }
        } else {
            Self {
                env: Environment::Development,
                key_name: "dfx_test_key".to_string(),
                sign_cycles: 0,
                rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545/".to_owned(),
                owner: Principal::anonymous(),
                public_key_str: "".to_string(),
                rpc_canister: "".to_string(),
                owner_public_key: vec![],
                rpc_endpoints: vec![
                    RPCEndpoints {
                        rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                        rpc_name: "mumbai".to_owned(),
                    },
                    RPCEndpoints {
                        rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_owned(),
                        rpc_name: "binance".to_owned(),
                    },
                ],
                mumbai_contract: "0x1d5989e6f450BBd9385Ce98922C0218af4c9aE97".to_string(),
                binance_contract: "0x4FF62fC53b4fCD08428aE79a59B36A5Ba9235817".to_string(),
            }
        }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RPCEndpoints {
    pub rpc_url: String,
    pub rpc_name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WalletValues {
    pub binance_nonce: u64,
    pub mumbai_nonce: u64,
}

impl Default for WalletValues {
    fn default() -> Self {
        WalletValues {
            binance_nonce: 0,
            mumbai_nonce: 0,
        }
    }
}

pub type RPCEndpointList = Vec<RPCEndpoints>;
#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct State {
    pub users: HashMap<Principal, UserData>,
    pub config: Config,
    pub nonce: WalletValues,
}

impl Storable for State {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE, // Replace with the actual max size
        is_fixed_size: false,
    };
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}
