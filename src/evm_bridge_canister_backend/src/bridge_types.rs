use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;
use std::{borrow::Cow, cell::RefCell};
const MAX_VALUE_SIZE: u32 = 1000000;

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum ChainSelection {
    #[serde(rename = "mumbai")]
    Mumbai,
    #[serde(rename = "binance")]
    Binance,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Nonce {
    pub nonce: u64,
    pub chain_id: u64,
}
#[derive(Clone, Debug, CandidType, Deserialize, Default)]
pub struct CanisterWallet {
    pub public_key: Vec<u8>,
    pub public_key_str: String,
    pub nonces: Vec<Nonce>,
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
        max_size: 100, // Replace with the actual max size
        is_fixed_size: false,
    };
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct NetworkDetails {
    pub network_name: String,
    pub rpc_url: String,
    pub contract_address: String,
    pub chain_id: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub key_name: String,
    pub sign_cycles: u64,
    pub owner: Principal,
    pub rpc_canister: String,
    pub network_details_list: Vec<NetworkDetails>,
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
                owner: Principal::anonymous(),
                rpc_canister: "".to_string(),
                network_details_list: vec![
                    NetworkDetails {
                        network_name: "mumbai".to_string(),
                        rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                        contract_address: "0x3270934BF219CBD66697DE72fB218A2cC44bBfe9".to_string(),
                        chain_id: 80001,
                    },
                    NetworkDetails {
                        network_name: "binance".to_string(),
                        rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_owned(),
                        contract_address: "0x17E9C819Ea0fd3793a41248fA0724a35CD7Ff8a7".to_string(),
                        chain_id: 97,
                    },
                ],
            }
        } else if env == Environment::Production {
            Self {
                env: Environment::Production,
                key_name: "key_1".to_string(),
                sign_cycles: 26_153_846_153,
                owner: Principal::anonymous(),
                rpc_canister: "".to_string(),
                network_details_list: vec![
                    NetworkDetails {
                        network_name: "mumbai".to_string(),
                        rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                        contract_address: "0x3270934BF219CBD66697DE72fB218A2cC44bBfe9".to_string(),
                        chain_id: 80001,
                    },
                    NetworkDetails {
                        network_name: "binance".to_string(),
                        rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_owned(),
                        contract_address: "0x17E9C819Ea0fd3793a41248fA0724a35CD7Ff8a7".to_string(),
                        chain_id: 97,
                    },
                ],
            }
        } else {
            Self {
                env: Environment::Development,
                key_name: "dfx_test_key".to_string(),
                sign_cycles: 0,
                owner: Principal::anonymous(),
                rpc_canister: "".to_string(),
                network_details_list: vec![
                    NetworkDetails {
                        network_name: "mumbai".to_string(),
                        rpc_url: "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_owned(),
                        contract_address: "0x3270934BF219CBD66697DE72fB218A2cC44bBfe9".to_string(),
                        chain_id: 80001,
                    },
                    NetworkDetails {
                        network_name: "binance".to_string(),
                        rpc_url: "https://data-seed-prebsc-1-s1.binance.org:8545".to_owned(),
                        contract_address: "0x17E9C819Ea0fd3793a41248fA0724a35CD7Ff8a7".to_string(),
                        chain_id: 97,
                    },
                ],
            }
        }
    }
}

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct State {
    pub canister_wallet: CanisterWallet,
    pub config: Config,
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
