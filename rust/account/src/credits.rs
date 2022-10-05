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

use snarkvm_wasm::{
    network::Testnet3,
    program::{Identifier as AleoIdentifier, ProgramID as AleoProgramID, Value as AleoValue},
    AleoV0,
    Process as AleoProcess,
    Transaction as AleoTransaction,
};

pub type CurrentNetwork = Testnet3;
pub type Process = AleoProcess<CurrentNetwork>;
pub type ProgramID = AleoProgramID<CurrentNetwork>;
pub type Identifier = AleoIdentifier<CurrentNetwork>;
pub type Value = AleoValue<CurrentNetwork>;
pub type Transaction = AleoTransaction<CurrentNetwork>;
pub type Aleo = AleoV0;
