// Mark this test as BPF-only due to current `ProgramTest` limitations when CPIing into the system program
#![cfg(feature = "test-bpf")]

use {
    solana_program::{
        pubkey::Pubkey,
        rent::Rent,
        instruction::AccountMeta,
        instruction::Instruction as SolanaProgramInstruction,
    },
    solana_program_test::*,
    solana_sdk::{
        signature::Signer,
        transaction::Transaction,
        account::Account,
    },
    kaizen::{
        *,
        instruction::KaizenInstruction,
        processor::Processor,
        state::{
            Settings,
            Token,
            SwapArgs,
            WithdrawArgs,
        },
    },
    borsh::BorshSerialize,
};
use std::str::FromStr;

//**************************************************************************************************
//  program_test
//--------------------------------------------------------------------------------------------------
pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "kaizen",
        id(),
        processor!(Processor::process_instruction),
    )
}

//**************************************************************************************************
//  test_initialize
//--------------------------------------------------------------------------------------------------
#[tokio::test]
async fn test_initialize() {
    let (account_settings_pubkey, _bump_seed0) = Pubkey::find_program_address(&[b"You pass butter0"], &id());
    let (account_from_pubkey, _bump_seed2) = Pubkey::find_program_address(&[b"You pass butter2"], &id());

    let mut program_test = ProgramTest::new(
        "kaizen",
        id(),
        processor!(Processor::process_instruction),
    );

    program_test.add_account(
        account_settings_pubkey,
        Account {
            owner: id(),
            lamports:   Rent::default().minimum_balance(1000),
            data:       vec![0; 165],
            ..Account::default()
        },
    );
    program_test.add_account(
        account_from_pubkey,
        Account {
            owner: id(),
            lamports:   Rent::default().minimum_balance(1000),
            data:       vec![0; 17],
            ..Account::default()
        },
    );

    let (mut banks_client, payer, last_blockhash) = program_test.start().await;


    // init
    let settings = Settings {
        is_initialized:         true,
        revenue_owner:          Pubkey::from_str("FzTgrM9hhmyB9w7E1iNdStYQ3ikNNhpCYjbPjGSr8t78").unwrap(),
        interest_basis_points:  10,
        locked_token:           Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap(),
        locked_token_owner:     Pubkey::from_str("FzTgrM9hhmyB9w7E1iNdStYQ3ikNNhpCYjbPjGSr8t78").unwrap(),
        unlock_timestamp:       1629999999,
        supply_total:           1_000_000,
        supply_locked:          0,
        token0:                 Token { address: Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap(), price: 2 },
    };

    let     instraction_t           = KaizenInstruction::Initialize(settings);
    let     instraction_serialized  = instraction_t.try_to_vec().unwrap();
    let mut transaction             = Transaction::new_with_payer(
        &[SolanaProgramInstruction {
            program_id: id(),
            accounts:   vec![
                AccountMeta::new(account_settings_pubkey, false),
            ],
            data:       instraction_serialized,
        }],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], last_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();


    // swap
    let     instraction_swap_t      = KaizenInstruction::Swap(SwapArgs { amount: 100 });
    let     instraction_swap        = instraction_swap_t.try_to_vec().unwrap();
    let mut transaction_swap        = Transaction::new_with_payer(
        &[SolanaProgramInstruction {
            program_id: id(),
            accounts:   vec![
                AccountMeta::new(account_settings_pubkey, false),
                AccountMeta::new(account_from_pubkey, false),
            ],
            data:       instraction_swap,
        }],
        Some(&payer.pubkey()),
    );

    transaction_swap.sign(&[&payer], last_blockhash);
    banks_client.process_transaction(transaction_swap).await.unwrap();


    // withdraw
    let     instraction_withdraw_t      = KaizenInstruction::Withdraw(WithdrawArgs { amount: 55 });
    let     instraction_withdraw        = instraction_withdraw_t.try_to_vec().unwrap();
    let mut transaction_withdraw        = Transaction::new_with_payer(
        &[SolanaProgramInstruction {
            program_id: id(),
            accounts:   vec![
                AccountMeta::new(account_settings_pubkey, false),
                AccountMeta::new(account_from_pubkey, false),
            ],
            data:       instraction_withdraw,
        }],
        Some(&payer.pubkey()),
    );

    transaction_withdraw.sign(&[&payer], last_blockhash);
    banks_client.process_transaction(transaction_withdraw).await.unwrap();
}