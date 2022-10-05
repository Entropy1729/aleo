// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use aleo_account::{
    Address,
    Aleo,
    Identifier,
    PrivateKey,
    Process,
    ProgramID,
    RecordCiphertext,
    Transaction,
    Value,
    ViewKey,
};

use anyhow::{ensure, Result};
use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use std::{convert::TryFrom, str::FromStr};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
struct ExecutionRequest {
    transaction: Transaction,
    address: Address,
    program_id: ProgramID,
}

impl ExecutionRequest {
    pub fn new(transaction: Transaction, address: Address, program_id: ProgramID) -> Result<Self> {
        ensure!(
            matches!(transaction, Transaction::Execute(_, _, _)),
            "Cannot create an execution request with a deploy transaction"
        );
        Ok(Self {
            transaction,
            address,
            program_id,
        })
    }
}

impl Serialize for ExecutionRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut request = serializer.serialize_struct("ExecuteRequest", 3)?;
        request.serialize_field("transaction", &self.transaction)?;
        request.serialize_field("address", &self.address)?;
        request.serialize_field("program_id", &self.program_id)?;
        request.end()
    }
}

#[wasm_bindgen]
struct ExecutionResponse {
    transaction: Transaction,
}

impl ExecutionResponse {
    pub fn new(transaction: Transaction) -> Result<Self> {
        ensure!(
            matches!(transaction, Transaction::Execute(_, _, _)),
            "Cannot create an execution response with a deploy transaction"
        );
        Ok(Self { transaction })
    }
}

impl<'de> Deserialize<'de> for ExecutionResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let response = serde_json::Value::deserialize(deserializer)?;
        let execution_response =
            Self::new(serde_json::from_value(response["transaction"].clone()).map_err(de::Error::custom)?);
        match execution_response {
            Ok(execution_response) => Ok(execution_response),
            Err(error) => Err(de::Error::custom(error.to_string())),
        }
    }
}

/*
TODOs:
- Handle errors
- Modularize code
- Modularize structs
- Add tests
- Parametrize endpoints
*/
#[wasm_bindgen]
pub async fn transfer(from: &str, to: &str, amount: u64) -> Result<String, JsValue> {
    let from = PrivateKey::from_str(from).unwrap();
    let to = Address::from_str(to).unwrap();

    let view_key = ViewKey::try_from(from).unwrap();
    let client = reqwest_wasm::Client::new();
    let mut header = reqwest_wasm::header::HeaderMap::new();
    header.insert(
        reqwest_wasm::header::ACCEPT,
        reqwest_wasm::header::HeaderValue::from_static("application/json"),
    );
    let mut ciphertexts = client
        .post("http://localhost/testnet3/ciphertexts/unspent")
        .headers(header.clone())
        .json(&json!(view_key))
        .send()
        .await
        .unwrap()
        .json::<Vec<RecordCiphertext>>()
        .await
        .unwrap()
        .into_iter();

    let rng = &mut rand::thread_rng();
    let process = Process::load().unwrap();
    let program_id = ProgramID::from_str("credits.aleo").unwrap();
    let authorization = process
        .authorize::<Aleo, _>(
            &from,
            &program_id,
            Identifier::from_str("transfer").unwrap(),
            &[
                Value::Record(ciphertexts.next().unwrap().decrypt(&view_key).unwrap()),
                Value::from_str(&format!("{to}")).unwrap(),
                Value::from_str(&format!("{amount}u64")).unwrap(),
            ],
            rng,
        )
        .unwrap();
    let (_, execution) = process.execute::<Aleo, _>(authorization, rng).unwrap();
    let (_, additional_fee) = process
        .execute_additional_fee::<Aleo, _>(&from, ciphertexts.next().unwrap().decrypt(&view_key).unwrap(), 1, rng)
        .unwrap();
    let execution_transaction = Transaction::from_execution(execution, Some(additional_fee)).unwrap();
    let execution_transaction = client
        .post("http://localhost:4000/testnet3/execute")
        .headers(header)
        .json(&json!(
            ExecutionRequest::new(execution_transaction, to, program_id).unwrap()
        ))
        .send()
        .await
        .unwrap()
        .json::<ExecutionResponse>()
        .await
        .unwrap();

    Ok(execution_transaction.transaction.id().to_string())
}
