use {
    super::shared,
    pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult},
};

#[inline(always)]
pub fn process_transfer(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let amount = u64::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_error| ProgramError::InvalidInstructionData)?,
    );

    shared::transfer::process_transfer(accounts, amount, None)
}
