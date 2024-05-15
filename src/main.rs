// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A minimal omni-node capable of running any substrate-based runtime so long as it adheres
//! [`standards`].
//!
//! See this module for more information about the assumptions of this node.

#![warn(missing_docs)]

mod cli;
mod command;
mod fake_runtime_api;
mod rpc;
mod service;
mod standards;

fn main() -> sc_cli::Result<()> {
    command::run()
}
