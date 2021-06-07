use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
use solana_program::{
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use std::convert::TryInto;
use std::fmt;

//**************************************************************************************************
//  Settings
//--------------------------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Settings {
    pub is_initialized:         bool,
    pub revenue_owner:          Pubkey,
    pub interest_basis_points:  u32,
    pub locked_token:           Pubkey,
    pub locked_token_owner:     Pubkey,
    pub unlock_timestamp:       u64,
    pub supply_total:           u64,
    pub supply_locked:          u64,
    pub token0:                 Token,
}

//**************************************************************************************************
//  Token
//--------------------------------------------------------------------------------------------------
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Token {
    pub address:    Pubkey,
    pub price:      u64,
}

//**************************************************************************************************
//  SwapArgs
//--------------------------------------------------------------------------------------------------
#[derive(Debug, BorshSerialize, BorshDeserialize, Copy, Clone, Default, PartialEq)]
pub struct SwapArgs {
    pub amount: u64,
}

//**************************************************************************************************
//  WithdrawArgs
//--------------------------------------------------------------------------------------------------
#[derive(Debug, BorshSerialize, BorshDeserialize, Copy, Clone, Default, PartialEq)]
pub struct WithdrawArgs {
    pub amount: u64,
}

//**************************************************************************************************
//  Savings
//--------------------------------------------------------------------------------------------------
#[derive(Debug, BorshSerialize, BorshDeserialize, Copy, Clone, Default, PartialEq)]
pub struct Savings {
    pub is_initialized:     bool,
    pub total_technical:    u64,
    pub total_original:     u64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
impl IsInitialized for Settings {

    //==================================================================================================
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl BorshDeserialize for Settings {

    //==================================================================================================
    fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
        let input = array_ref![_buf, 0, 165];
        let (
            is_initialized_b,
            revenue_owner_b,
            interest_basis_points_b,
            locked_token_b,
            locked_token_owner_b,
            unlock_timestamp_b,
            supply_total_b,
            supply_locked_b,
            tokens_b
        ) = array_refs![input, 1, 32, 4, 32, 32, 8, 8, 8, 40];

        let settings = Settings {
            is_initialized: match is_initialized_b {
                [0] => false,
                _ => true,
            },
            revenue_owner:          Pubkey::new_from_array(*revenue_owner_b),
            interest_basis_points:  u32::from_le_bytes(*interest_basis_points_b),
            locked_token:           Pubkey::new_from_array(*locked_token_b),
            locked_token_owner:     Pubkey::new_from_array(*locked_token_owner_b),
            unlock_timestamp:       u64::from_le_bytes(*unlock_timestamp_b),
            supply_total:           u64::from_le_bytes(*supply_total_b),
            supply_locked:          u64::from_le_bytes(*supply_locked_b),
            token0:                 Token::try_from_slice(tokens_b[0..40].try_into().expect("slice with incorrect length"))?
        };

        *_buf = &_buf[165..];

        Ok(settings)
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BorshSerialize for Settings {

    //==================================================================================================
    #[inline]
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        (self.is_initialized as u8).serialize(writer)?;
        self.revenue_owner.to_bytes().serialize(writer)?;
        self.interest_basis_points.serialize(writer)?;
        self.locked_token.to_bytes().serialize(writer)?;
        self.locked_token_owner.to_bytes().serialize(writer)?;
        self.unlock_timestamp.serialize(writer)?;
        self.supply_total.serialize(writer)?;
        self.supply_locked.serialize(writer)?;
        self.token0.serialize(writer)?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sealed for Settings {}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Pack for Settings {
    const LEN: usize = 165;

    //==================================================================================================
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 165];
        let (
            is_initialized_b,
            revenue_owner_b,
            interest_basis_points_b,
            locked_token_b,
            locked_token_owner_b,
            unlock_timestamp_b,
            supply_total_b,
            supply_locked_b,
            tokens_b0,
        ) = mut_array_refs![output, 1, 32, 4, 32, 32, 8, 8, 8, 40];

        is_initialized_b[0]         = self.is_initialized as u8;
        revenue_owner_b             .copy_from_slice(self.revenue_owner.as_ref());
        *interest_basis_points_b    = self.interest_basis_points.to_le_bytes();
        locked_token_b              .copy_from_slice(self.locked_token.as_ref());
        locked_token_owner_b        .copy_from_slice(self.locked_token_owner.as_ref());
        *unlock_timestamp_b         = self.unlock_timestamp.to_le_bytes();
        *supply_total_b             = self.supply_total.to_le_bytes();
        *supply_locked_b            = self.supply_locked.to_le_bytes();
        Token::pack(self.token0, &mut *tokens_b0).expect("slice with incorrect length");
    }
    //==================================================================================================
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, 165];
        let (
            is_initialized_b,
            revenue_owner_b,
            interest_basis_points_b,
            locked_token_b,
            locked_token_owner_b,
            unlock_timestamp_b,
            supply_total_b,
            supply_locked_b,
            tokens_b
        ) = array_refs![input, 1, 32, 4, 32, 32, 8, 8, 8, 40];

        Ok(Self {
            is_initialized: match is_initialized_b {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            revenue_owner:          Pubkey::new_from_array(*revenue_owner_b),
            interest_basis_points:  u32::from_le_bytes(*interest_basis_points_b),
            locked_token:           Pubkey::new_from_array(*locked_token_b),
            locked_token_owner:     Pubkey::new_from_array(*locked_token_owner_b),
            unlock_timestamp:       u64::from_le_bytes(*unlock_timestamp_b),
            supply_total:           u64::from_le_bytes(*supply_total_b),
            supply_locked:          u64::from_le_bytes(*supply_locked_b),
            token0:                 Token::unpack_unchecked(tokens_b[0..40].try_into().expect("slice with incorrect length"))?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BorshDeserialize for Token {

    //==================================================================================================
    #[inline]
    fn deserialize(_buf: &mut &[u8]) -> std::io::Result<Self> {
        let input               = array_ref![_buf, 0, 40];
        let (pubkey_b, price_b) = array_refs![input, 32, 8];
        let token = Token {
            address:    Pubkey::new_from_array(*pubkey_b),
            price:      u64::from_le_bytes(*price_b),
        };
        *_buf = &_buf[40..];
        Ok(token)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl BorshSerialize for Token {

    //==================================================================================================
    #[inline]
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.address.to_bytes().serialize(writer)?;
        self.price.serialize(writer)?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Token {

    //==================================================================================================
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
         .field("address", &self.address)
         .field("price", &self.price)
         .finish()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sealed for Token {}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Pack for Token {
    const LEN: usize = 40;

    //==================================================================================================
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output                  = array_mut_ref![output, 0, 40];
        let (address_b, price_b)    = mut_array_refs![output, 32, 8];

        address_b.copy_from_slice(self.address.as_ref());
        *price_b = self.price.to_le_bytes();
    }
    //==================================================================================================
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input                   = array_ref![input, 0, 40];
        let (address_b, price_b)    = array_refs![input, 32, 8];
        Ok(Self {
            address:    Pubkey::new_from_array(*address_b),
            price:      u64::from_le_bytes(*price_b),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////
impl IsInitialized for Savings {
    
    //==================================================================================================
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sealed for Savings {}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Pack for Savings {
    const LEN: usize = 17;

    //==================================================================================================
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 17];
        let (
            is_initialized_b,
            total_technical_b,
            total_original_b,
        ) = mut_array_refs![output, 1, 8, 8];

        is_initialized_b[0] = self.is_initialized as u8;
        *total_technical_b  = self.total_technical.to_le_bytes();
        *total_original_b   = self.total_original.to_le_bytes();
    }
    //==================================================================================================
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, 17];
        let (
            is_initialized_b,
            total_technical_b,
            total_original_b
        ) = array_refs![input, 1, 8, 8];

        Ok(Self {
            is_initialized: match is_initialized_b {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            total_technical:    u64::from_le_bytes(*total_technical_b),
            total_original:     u64::from_le_bytes(*total_original_b)
        })
    }
}