// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Support for encoding transactions for common situations.

use crate::{account::create_signed_txn_with_association_account, account::Account};
use stdlib::transaction_scripts::{CREATE_ACCOUNT_TXN, MINT_TXN, PEER_TO_PEER_TRANSFER_TXN};
use types::account_address::AccountAddress;
use types::transaction::{SignedUserTransaction, TransactionArgument};

pub const TXN_RESERVED: u64 = 500_000;

/// Returns a transaction to create a new account with the given arguments.
pub fn create_account_txn(
    sender: &Account,
    new_account: &Account,
    seq_num: u64,
    initial_amount: u64,
) -> SignedUserTransaction {
    let mut args: Vec<TransactionArgument> = Vec::new();
    args.push(TransactionArgument::Address(*new_account.address()));
    args.push(TransactionArgument::U8Vector(new_account.auth_key_prefix()));
    args.push(TransactionArgument::U64(initial_amount));

    sender.create_signed_txn_with_args(CREATE_ACCOUNT_TXN.clone(), args, seq_num, TXN_RESERVED, 1)
}

/// Returns a transaction to transfer coin from one account to another (possibly new) one, with the
/// given arguments.
pub fn peer_to_peer_txn(
    sender: &Account,
    receiver: &Account,
    seq_num: u64,
    transfer_amount: u64,
) -> SignedUserTransaction {
    let mut args: Vec<TransactionArgument> = Vec::new();
    args.push(TransactionArgument::Address(*receiver.address()));
    args.push(TransactionArgument::U8Vector(receiver.auth_key_prefix()));
    args.push(TransactionArgument::U64(transfer_amount));

    // get a SignedTransaction
    sender.create_signed_txn_with_args(
        PEER_TO_PEER_TRANSFER_TXN.clone(),
        args,
        seq_num,
        TXN_RESERVED, // this is a default for gas
        1,            // this is a default for gas
    )
}

/// Returns a transaction to mint new funds with the given arguments.
pub fn mint_txn(
    sender: &Account,
    receiver: &Account,
    seq_num: u64,
    transfer_amount: u64,
) -> SignedUserTransaction {
    let mut args: Vec<TransactionArgument> = Vec::new();
    args.push(TransactionArgument::Address(*receiver.address()));
    args.push(TransactionArgument::U8Vector(receiver.auth_key_prefix()));
    args.push(TransactionArgument::U64(transfer_amount));

    // get a SignedTransaction
    sender.create_signed_txn_with_args(
        MINT_TXN.clone(),
        args,
        seq_num,
        TXN_RESERVED, // this is a default for gas
        1,            // this is a default for gas
    )
}

/// Returns a transaction to create a new account with the given arguments.
pub fn create_account_txn_send_with_association_account(
    new_account: &Account,
    seq_num: u64,
    initial_amount: u64,
) -> SignedUserTransaction {
    let mut args: Vec<TransactionArgument> = Vec::new();
    args.push(TransactionArgument::Address(*new_account.address()));
    args.push(TransactionArgument::U8Vector(new_account.auth_key_prefix()));
    args.push(TransactionArgument::U64(initial_amount));

    create_signed_txn_with_association_account(
        CREATE_ACCOUNT_TXN.clone(),
        args,
        seq_num,
        TXN_RESERVED,
        1,
    )
}