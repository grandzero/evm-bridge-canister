use ethers_core::abi::{Contract, FunctionExt, Token};
use hex;
use hex::FromHexError;
use std::rc::Rc;

pub fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

pub fn from_hex(data: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(&data[2..])
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

pub async fn get_data_with_arguments(
    function_name: &str,
    args: &[Token],
) -> Result<Vec<u8>, String> {
    let abi = Rc::new(include_abi!("icp_contract.json"));
    // let owner_address =
    //     ethers_core::types::Address::from_str(&receiver_address).expect("Invalid owner address");
    let result = prepare_data(&abi, function_name, args).await;
    return result;
}
