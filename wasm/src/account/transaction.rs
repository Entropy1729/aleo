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

use std::str::FromStr;

use aleo_account::{Transaction as TransactionNative, Transactions as TransactionsNative};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction(TransactionNative);

#[wasm_bindgen]
impl Transaction {
    pub fn from_string(transaction: &str) -> Self {
        Self::from_str(transaction).unwrap()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Transaction {
    type Err = anyhow::Error;

    fn from_str(transaction: &str) -> Result<Self, Self::Err> {
        Ok(Self(TransactionNative::from_str(transaction)?))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transactions(TransactionsNative);

#[wasm_bindgen]
impl Transactions {
    pub fn from_string(transactions: &str) -> Self {
        Self::from_str(transactions).unwrap()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Transactions {
    type Err = anyhow::Error;

    fn from_str(transactions: &str) -> Result<Self, Self::Err> {
        Ok(Self(TransactionsNative::from_str(transactions)?))
    }
}
