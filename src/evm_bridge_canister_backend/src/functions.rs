use crate::utils;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, EcdsaKeyId, EcdsaPublicKeyArgument, EcdsaPublicKeyResponse,
    SignWithEcdsaResponse,
};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, SignWithEcdsaArgument};
use primitive_types::U256;
use serde::Serialize;

use crate::bridge_types::*;
use crate::transaction;
pub use utils::u64_to_u256;
use utils::{get_address_from_public_key, get_derivation_path};

use transaction::*;
#[derive(CandidType, Serialize, Debug)]
pub struct CreateAddressResponse {
    pub address: String,
    pub public_key: Vec<u8>,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct SignTransactionResponse {
    pub sign_tx: Vec<u8>,
}

pub async fn get_address() -> String {
    let state = STATE.with(|s| s.borrow().clone());
    let user = state.canister_wallet;
    if let Ok(address) = get_address_from_public_key(user.public_key.clone()) {
        return address;
    }

    return "Error while getting address".to_owned();
}

pub async fn create_address(principal_id: Principal) -> Result<CreateAddressResponse, String> {
    let state = STATE.with(|s| s.borrow().clone());

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
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.canister_wallet.public_key = res.public_key.clone();
        state.canister_wallet.public_key_str = address.clone();
    });

    Ok(CreateAddressResponse {
        address,
        public_key: res.public_key,
    })
}

pub async fn sign_transaction(
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
    principal_id: Principal,
) -> Result<SignTransactionResponse, String> {
    let state = STATE.with(|s| s.borrow().clone());
    let user = state.canister_wallet.clone();

    let mut tx = transaction::get_transaction(&hex_raw_tx, chain_id.clone()).unwrap();

    let message = tx.get_message_to_sign().unwrap();

    // assert!(message.len() == 32);

    let key_id = EcdsaKeyId {
        curve: EcdsaCurve::Secp256k1,
        name: state.config.key_name,
    };

    let caller = get_derivation_path(principal_id);

    // user.public_key = res.public_key.clone();
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

    return Ok(SignTransactionResponse { sign_tx: signed_tx });
    // Ok(SignTransactionResponse { sign_tx: signed_tx })
}

pub async fn sign_custom_tx(
    data: String,
    chain_id: u64,
) -> Result<SignTransactionResponse, String> {
    let state = STATE.with(|s| s.borrow().clone());
    // let nonce = state.nonce; // TODO: Store nonce for each network and use accordingly
    let gas_price = U256::from_dec_str("2000000000").unwrap(); // TODO: Use actual gas price from rpc

    let contract_address = state
        .config
        .network_details_list
        .iter()
        .find(|x| x.chain_id == chain_id)
        .unwrap()
        .contract_address
        .clone();
    let nonce = state
        .canister_wallet
        .nonces
        .iter()
        .find(|x| x.chain_id == chain_id)
        .unwrap()
        .nonce;
    ic_cdk::println!("contract address: {:?}", contract_address.clone());
    ic_cdk::println!("nonce: {:?}", nonce);
    ic_cdk::println!("chain_id: {}", chain_id);
    let legacy = transaction::TransactionLegacy {
        chain_id,
        nonce, // TODO: Use actual nonce
        gas_price: gas_price,
        gas_limit: 1000000,
        to: contract_address,
        value: U256::from(0),
        data,
        v: "0x00".to_string(),
        r: "0x00".to_string(),
        s: "0x00".to_string(),
    };

    let raw_tx = legacy.serialize().unwrap();

    // let raw_tx = tx.serialize().unwrap();
    let res = sign_transaction(raw_tx, chain_id, state.config.owner).await;
    return res;
}
