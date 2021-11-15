use solana_program::program_error::ProgramError;

pub enum EscrowInstruction {
    /// create escrow acct
    /// populate escrow acct
    /// transfer ownership of temp token acct to PDA
    /// 
    /// Accts expected:
    /// 0. `[signer]` the acct of the person initializing the escrow
    /// 1. `[writable]` temp token acct that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` initializer's token acct for the token they will receive, should the trade go thru
    /// 3. `[writable]` the escrow account, holds all necessary info about the trade
    /// 4. `[]` rent sysvar
    /// 5. `[]` token program
    InitEscrow {
      /// the amt that party A expects to receive of token Y
      amount: u64
    }
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?; // we will import InvalidInstruction next

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        
        Ok(amount)
    }
}