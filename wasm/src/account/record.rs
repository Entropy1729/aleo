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

use aleo_account::Record as RecordNative;

use core::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record(RecordNative);

#[wasm_bindgen]
impl Record {
    /// Creates a record from a string.
    /// @example
    /// let record = Record.from_string("{ owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, gates: 99u64.public, _nonce: 0group.public }");
    pub fn from_string(record: &str) -> Self {
        Self::from_str(record).unwrap()
    }

    /// Cast a record to a string.
    /// @example
    /// let record = Record.from_string("{ owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, gates: 99u64.public, _nonce: 0group.public }");
    /// record.to_string(); // "{
    ///     owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private,
    ///     gates: 99u64.public,
    ///     _nonce: 0group.public }"
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Get the gates of the record
    /// @example
    /// let record = Record.from_string("{ owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, gates: 99u64.public, _nonce: 0group.public }");
    /// let gates = record.gates(); // "99u64.public"
    pub fn gates(&self) -> String {
        self.0.gates().to_string()
    }
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(plaintext: &str) -> Result<Self, Self::Err> {
        Ok(Self(RecordNative::from_str(plaintext)?))
    }
}
