// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

mod account;
mod account_resource;
mod auto_validate;
pub mod command;
mod governance;
pub mod json_rpc;
mod keys;
mod owner;
mod validate_transaction;
mod validator_config;
mod validator_set;

mod network_checker;
#[cfg(any(test, feature = "testing"))]
pub mod test_helper;

use libra_secure_json_rpc::VMStatusView;
use libra_types::account_address::AccountAddress;
use serde::Serialize;

/// Information for validating a transaction after it's been submitted, or
/// retrieving the execution result.
#[derive(Debug, PartialEq, Serialize)]
pub struct TransactionContext {
    pub address: AccountAddress,
    pub sequence_number: u64,

    // The execution result of the transaction if it has already been validated
    // successfully.
    pub execution_result: Option<VMStatusView>,
}

impl TransactionContext {
    pub fn new(address: AccountAddress, sequence_number: u64) -> TransactionContext {
        TransactionContext::new_with_validation(address, sequence_number, None)
    }

    pub fn new_with_validation(
        address: AccountAddress,
        sequence_number: u64,
        execution_result: Option<VMStatusView>,
    ) -> TransactionContext {
        TransactionContext {
            address,
            sequence_number,
            execution_result,
        }
    }
}
