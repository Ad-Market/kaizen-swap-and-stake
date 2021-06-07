use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
use crate::state::{
    Settings,
    SwapArgs,
    WithdrawArgs,
};

//**************************************************************************************************
//  Instruction
//--------------------------------------------------------------------------------------------------
#[derive(Debug, BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq)]
pub enum KaizenInstruction {
    Initialize(Settings),
    Swap(SwapArgs),
    Withdraw(WithdrawArgs),
}