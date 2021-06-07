use crate::{
    instruction::KaizenInstruction,
    state::{
        Settings,
        SwapArgs,
        WithdrawArgs,
        Savings,
    },
};
use ::borsh::BorshDeserialize;
use solana_program::{
    pubkey::Pubkey,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_pack::{
        IsInitialized,
        Pack,
    },
};
use anchor_lang::prelude::*;

//**************************************************************************************************
//  Processor
//--------------------------------------------------------------------------------------------------
pub struct Processor {}

impl Processor {

    //==================================================================================================
    //  initialize
    //--------------------------------------------------------------------------------------------------
    fn initialize(_accounts: &[AccountInfo], _settings: &Settings) -> ProgramResult {
        let accounts_info_it    = &mut _accounts.iter();
        let account_settings    = next_account_info(accounts_info_it)?;
        let settings            = Settings::unpack_unchecked(&account_settings.data.borrow())?;

        if settings.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if _settings.unlock_timestamp <= (Clock::get()?.unix_timestamp as u64) {
            return Err(ProgramError::InvalidInstructionData); 
        }

        Settings::pack(Settings {
            is_initialized: true,
            supply_locked:  0,
            ..*_settings
        },
        &mut account_settings.data.borrow_mut())?;

        Ok(())
    }
    //==================================================================================================
    //  swap
    //--------------------------------------------------------------------------------------------------
    fn swap(_accounts: &[AccountInfo], _swap_args: &SwapArgs) -> ProgramResult {
        let accounts_info_it    = &mut _accounts.iter();
        let account_settings    = next_account_info(accounts_info_it)?;
        let account_from        = next_account_info(accounts_info_it)?;
        let mut settings        = Settings::unpack(&account_settings.data.borrow())?;

        if settings.unlock_timestamp <= (Clock::get()?.unix_timestamp as u64) {
            return Err(ProgramError::InvalidInstructionData); 
        }

        if !account_from.try_borrow_data()?.is_empty() && account_from.try_borrow_data()?.len() != Savings::LEN {
            return Err(ProgramError::InvalidAccountData); 
        }

        let amount_with_interest        = _swap_args.amount + (_swap_args.amount * (settings.interest_basis_points as u64) / 100);
        let custom_amount_with_interest = amount_with_interest / settings.token0.price;
        let custom_allowed              = settings.supply_total - settings.supply_locked;

        if custom_allowed < custom_amount_with_interest {
            return Err(ProgramError::InvalidInstructionData);
        }

        **account_from.try_borrow_mut_lamports()?       -= _swap_args.amount;
        **account_settings.try_borrow_mut_lamports()?   += _swap_args.amount;

        settings.supply_locked                          += custom_amount_with_interest;
        Settings::pack(settings, &mut account_settings.data.borrow_mut())?;

        let user_savings                                = Savings::unpack_unchecked(&account_from.data.borrow())?;
        Savings::pack(Savings {
            is_initialized: true,
            total_technical: if user_savings.is_initialized() {
                    user_savings.total_technical + custom_amount_with_interest
                } else {
                    custom_amount_with_interest
                },
            ..user_savings
        },
        &mut account_from.data.borrow_mut())?;

        Ok(())
    }
    //==================================================================================================
    //  withdraw
    //--------------------------------------------------------------------------------------------------
    fn withdraw(_accounts: &[AccountInfo], _withdraw_args: &WithdrawArgs) -> ProgramResult {
        let accounts_info_it    = &mut _accounts.iter();
        let account_settings    = next_account_info(accounts_info_it)?;
        let account_from        = next_account_info(accounts_info_it)?;
        let mut settings        = Settings::unpack(&account_settings.data.borrow())?;

        if (Clock::get()?.unix_timestamp as u64) < settings.unlock_timestamp {
            return Err(ProgramError::InvalidInstructionData); 
        }

        if account_from.try_borrow_data()?.is_empty() || account_from.try_borrow_data()?.len() != Savings::LEN {
            return Err(ProgramError::InvalidAccountData); 
        }

        if settings.supply_locked < _withdraw_args.amount {
            return Err(ProgramError::InvalidInstructionData);
        }

        settings.supply_locked  -= _withdraw_args.amount;
        let data: &mut [u8]     = &mut account_settings.data.borrow_mut();
        Settings::pack(settings, data)?;

        let mut user_savings = Savings::unpack(&account_from.data.borrow())?;

        if user_savings.total_technical < _withdraw_args.amount {
            return Err(ProgramError::InvalidInstructionData);
        }

        user_savings.total_technical    -= _withdraw_args.amount;
        user_savings.total_original     += _withdraw_args.amount;

        let account_from_data: &mut [u8] = &mut account_from.data.borrow_mut();
        Savings::pack(user_savings, account_from_data)?;

        Ok(())
    }
    //==================================================================================================
    //  process_instruction
    //--------------------------------------------------------------------------------------------------
    pub fn process_instruction(
        _program_id:        &Pubkey,
        _accounts:          &[AccountInfo],
        _instruction_data:  &[u8]
    ) -> ProgramResult {
        let instruction = KaizenInstruction::try_from_slice(_instruction_data)?;
        match instruction {
            KaizenInstruction::Initialize(settings)     => Self::initialize(_accounts, &settings),
            KaizenInstruction::Swap(swap_args)          => Self::swap(_accounts, &swap_args),
            KaizenInstruction::Withdraw(withdraw_args)  => Self::withdraw(_accounts, &withdraw_args),
        }
    }

}//impl Processor