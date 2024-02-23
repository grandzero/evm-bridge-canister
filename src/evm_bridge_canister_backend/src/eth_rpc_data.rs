use ethers_core::abi::{Contract, FunctionExt, Token};
use hex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::{rc::Rc, str::FromStr};

const HTTP_CYCLES: u128 = 100_000_000;
const MAX_RESPONSE_BYTES: u64 = 2048;

pub fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    id: u64,
    jsonrpc: String,
    method: String,
    params: (EthCallParams, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcTransactionRequest {
    id: u64,
    jsonrpc: String,
    method: String,
    params: (EthSendTransactionParams, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EthCallParams {
    to: String,
    data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EthSendTransactionParams {
    from: String,
    to: String,
    data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcResult {
    result: Option<String>,
    error: Option<JsonRpcError>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: isize,
    message: String,
}

fn next_id() -> u64 {
    thread_local! {
        static NEXT_ID: RefCell<u64> = RefCell::default();
    }
    NEXT_ID.with(|next_id| {
        let mut next_id = next_id.borrow_mut();
        let id = *next_id;
        *next_id = next_id.wrapping_add(1);
        id
    })
}

pub async fn prepare_data(
    abi: &Contract,
    function_name: &str,
    args: &[Token],
) -> Result<Vec<u8>, String> {
    let f = match abi.functions_by_name(function_name).map(|v| &v[..]) {
        Ok([f]) => f,
        Ok(fs) => panic!(
            "Found {} function overloads. Please pass one of the following: {}",
            fs.len(),
            fs.iter()
                .map(|f| format!("{:?}", f.abi_signature()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Err(_) => abi
            .functions()
            .find(|f| function_name == f.abi_signature())
            .expect("Function not found"),
    };
    let data = f
        .encode_input(args)
        .expect("Error while encoding input args");
    return Ok(data);
}

#[macro_export]
macro_rules! include_abi {
    ($file:expr $(,)?) => {{
        match serde_json::from_str::<ethers_core::abi::Contract>(include_str!($file)) {
            Ok(contract) => contract,
            Err(err) => panic!("Error loading ABI contract {:?}: {}", $file, err),
        }
    }};
}

pub async fn get_data(_contract_address: String, index: u64) -> Result<Vec<u8>, String> {
    let abi = Rc::new(include_abi!("icp_contract.json"));
    // let owner_address =
    //     ethers_core::types::Address::from_str(&receiver_address).expect("Invalid owner address");
    let result = prepare_data(
        &abi,
        "getMessage",
        &[Token::Uint(ethers_core::types::U256::from(index))],
    )
    .await;
    return result;
}

pub async fn prepare_data_to_sign(message: String) -> Result<Vec<u8>, String> {
    let abi = Rc::new(include_abi!("icp_contract.json"));
    // let owner_address =
    //     ethers_core::types::Address::from_str(&receiver_address).expect("Invalid owner address");
    let result = prepare_data(&abi, "sendMessage", &[Token::String(message)]).await;
    return result;
}
