use crate::utils;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, EcdsaKeyId, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse,
    SignWithEcdsaResponse,
};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, SignWithEcdsaArgument};
use primitive_types::U256;
use serde::Serialize;

use crate::state::*;
use crate::transaction;
pub use utils::u64_to_u256;
use utils::{get_address_from_public_key, get_derivation_path};

use transaction::*;
#[derive(CandidType, Serialize, Debug)]
pub struct CreateAddressResponse {
    pub address: String,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct SignTransactionResponse {
    pub sign_tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct DeployContractResponse {
    pub tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct TransferERC20Response {
    pub tx: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct UserResponse {
    pub address: String,
    pub transactions: TransactionChainData,
}

pub fn init() {
    // if let Some(env) = env_opt {
    // let env = match env {
    //     1 => Environment::Development,
    //     2 => Environment::Staging,
    //     3 => Environment::Production,
    //     _ => Environment::Development,
    // };
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.config = Config::from(Environment::Development);
    })
    // }
}

pub async fn get_address(principal_id: Principal) -> String {
    let state = STATE.with(|s| s.borrow().clone());
    let user = state.users.get(&principal_id).unwrap();
    if let Ok(address) = get_address_from_public_key(user.public_key.clone()) {
        return address;
    }

    return "Error while getting address".to_owned();
}

pub async fn create_address(principal_id: Principal) -> Result<CreateAddressResponse, String> {
    let state = STATE.with(|s| s.borrow().clone());
    let user = state.users.get(&principal_id);

    if let Some(_) = user {
        return Err("this wallet already exist".to_string());
    }

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: state.config.key_name,
    };

    let caller = get_derivation_path(principal_id);

    let request = EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![caller],
        key_id: key_id.clone(),
    };

    let (res,): (EcdsaPublicKeyResponse,) = ecdsa_public_key(request)
        .await
        .map_err(|e| format!("Failed to call ecdsa_public_key {}", e.1))?;

    let address = get_address_from_public_key(res.public_key.clone()).unwrap();

    let mut user = UserData::default();
    user.public_key = res.public_key;

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.users.insert(principal_id, user);
    });

    Ok(CreateAddressResponse { address })
}

pub async fn sign_transaction(
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
    principal_id: Principal,
) -> Result<SignTransactionResponse, String> {
    let state = STATE.with(|s| s.borrow().clone());
    let user;

    if let Some(i) = state.users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let mut tx = transaction::get_transaction(&hex_raw_tx, chain_id.clone()).unwrap();

    let message = tx.get_message_to_sign().unwrap();

    assert!(message.len() == 32);

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: state.config.key_name,
    };

    let caller = get_derivation_path(principal_id);

    let request = SignWithEcdsaArgument {
        message_hash: message.clone(),
        derivation_path: vec![caller],
        key_id: key_id.clone(),
    };

    let (res,): (SignWithEcdsaResponse,) =
        ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(request)
            .await
            .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;

    let signed_tx = tx.sign(res.signature.clone(), user.public_key).unwrap();

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();

        let mut transaction = Transaction::default();
        transaction.data = signed_tx.clone();
        transaction.timestamp = ic_cdk::api::time();

        if let Some(user_tx) = user.transactions.get_mut(&chain_id) {
            user_tx.transactions.push(transaction);
            user_tx.nonce = tx.get_nonce().unwrap() + 1;
        } else {
            let mut chain_data = TransactionChainData::default();
            chain_data.nonce = tx.get_nonce().unwrap() + 1;
            chain_data.transactions.push(transaction);

            user.transactions.insert(chain_id, chain_data);
        }
    });

    Ok(SignTransactionResponse { sign_tx: signed_tx })
}

pub async fn sign_custom_tx(data: String) -> Result<SignTransactionResponse, String> {
    let state = STATE.with(|s| s.borrow().clone());
    // let tx = transaction::Transaction1559 {
    //     chain_id: 1,
    //     nonce: 0,
    //     max_priority_fee_per_gas: U256::from(13),
    //     gas_limit: 1000000,
    //     max_fee_per_gas: U256::from(5),
    //     to: "0xFD23c55fc75e1eaAdBB5493639C84b54B331A396".to_owned(),
    //     value: U256::from(1),
    //     access_list: vec![],
    //     data: data.clone(),
    //     v: "0x00".to_string(),
    //     r: "0x00".to_string(),
    //     s: "0x00".to_string(),
    // };

    let legacy = transaction::TransactionLegacy {
        chain_id: 1,
        nonce: 0,
        gas_price: U256::from(5),
        gas_limit: 1000000,
        to: "0xFD23c55fc75e1eaAdBB5493639C84b54B331A396".to_owned(),
        value: U256::from(1),
        data,
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = legacy.serialize().unwrap();

    // let raw_tx = tx.serialize().unwrap();
    let res = sign_transaction(raw_tx, 1, state.config.owner).await;
    return res;
}

pub async fn deploy_contract(
    principal_id: Principal,
    bytecode: Vec<u8>,
    chain_id: u64,
    max_priority_fee_per_gas: U256,
    gas_limit: u64,
    max_fee_per_gas: U256,
) -> Result<DeployContractResponse, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let nonce: u64;
    if let Some(user_transactions) = user.transactions.get(&chain_id) {
        nonce = user_transactions.nonce;
    } else {
        nonce = 0;
    }
    let data = "0x".to_owned() + &utils::vec_u8_to_string(&bytecode);
    let tx = transaction::Transaction1559 {
        nonce,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        to: "0x".to_string(),
        value: U256::zero(),
        data,
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = tx.serialize().unwrap();
    let res = sign_transaction(raw_tx, chain_id, principal_id)
        .await
        .unwrap();

    Ok(DeployContractResponse { tx: res.sign_tx })
}

pub async fn transfer_erc_20(
    principal_id: Principal,
    chain_id: u64,
    max_priority_fee_per_gas: U256,
    gas_limit: u64,
    max_fee_per_gas: U256,
    address: String,
    value: U256,
    contract_address: String,
) -> Result<TransferERC20Response, String> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;

    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return Err("this user does not exist".to_string());
    }

    let nonce: u64;
    if let Some(user_transactions) = user.transactions.get(&chain_id) {
        nonce = user_transactions.nonce;
    } else {
        nonce = 0;
    }

    let data = "0x".to_owned() + &utils::get_transfer_data(&address, value).unwrap();

    let tx = transaction::Transaction1559 {
        nonce,
        chain_id,
        max_priority_fee_per_gas,
        gas_limit,
        max_fee_per_gas,
        to: contract_address,
        value: U256::zero(),
        data,
        access_list: vec![],
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = tx.serialize().unwrap();

    let res = sign_transaction(raw_tx, chain_id, principal_id)
        .await
        .unwrap();

    Ok(TransferERC20Response { tx: res.sign_tx })
}

pub fn get_caller_data(principal_id: Principal, chain_id: u64) -> Option<UserResponse> {
    let users = STATE.with(|s| s.borrow().users.clone());
    let user;
    if let Some(i) = users.get(&principal_id) {
        user = i.clone();
    } else {
        return None;
    }

    let address = get_address_from_public_key(user.public_key.clone()).unwrap();

    let transaction_data = user
        .transactions
        .get(&chain_id)
        .cloned()
        .unwrap_or_else(|| TransactionChainData::default());

    Some(UserResponse {
        address,
        transactions: transaction_data,
    })
}

pub fn clear_caller_history(principal_id: Principal, chain_id: u64) -> Result<(), String> {
    let users = STATE.with(|s| s.borrow().users.clone());

    if let None = users.get(&principal_id) {
        return Err("this user does not exist".to_string());
    }

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let user = state.users.get_mut(&principal_id).unwrap();
        let user_tx = user.transactions.get_mut(&chain_id);
        if let Some(user_transactions) = user_tx {
            user_transactions.transactions.clear();
        }
    });

    Ok(())
}
