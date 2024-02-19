use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};
const MAX_VALUE_SIZE: u32 = 1000000;
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
                rpc_url: "https://polygon-mumbai-pokt.nodies.app".to_owned(),
            }
        } else if env == Environment::Production {
            Self {
                env: Environment::Production,
                key_name: "key_1".to_string(),
                sign_cycles: 26_153_846_153,
                rpc_url: "https://polygon-mumbai-pokt.nodies.app".to_owned(),
            }
        } else {
            Self {
                env: Environment::Development,
                key_name: "dfx_test_key".to_string(),
                sign_cycles: 0,
                rpc_url: "https://polygon-mumbai-pokt.nodies.app".to_owned(),
            }
        }
    }
}

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct State {
    pub users: HashMap<Principal, UserData>,
    pub config: Config,
    pub owner: String,
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
