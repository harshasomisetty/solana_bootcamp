use std::cmp;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar::{rent::Rent, Sysvar},
};

use crate::error::EchoError;
use crate::instruction::EchoInstruction;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let accounts_iter = &mut _accounts.iter();
        match instruction {
            EchoInstruction::Echo { data } => {
                msg!("Instruction: Echo");

                if _accounts[0].data.borrow().iter().all(|&x| x != 0) {
                    msg!("nonzero rip");
                    return Err(EchoError::NonzeroData.into());
                }

                // msg!("empty {:?}", _accounts[0].data_is_empty());
                let mut data_arr = _accounts[0].try_borrow_mut_data()?;

                let write_size = cmp::min(data_arr.len(), data.len());
                // msg!("data arr {:?}", data_arr);
                for n in 0..write_size {
                    data_arr[n] = data[n];
                }

                Ok(())
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed,
                buffer_size,
            } => {
                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                let (authorized_buffer_key, bump_seed) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes(),
                    ],
                    _program_id,
                );

                msg!("buffer_seed: {:?}", buffer_seed);
                msg!("buffer_size: {:?}", buffer_size);
                msg!("authorized_buf_key {:?}", authorized_buffer_key);
                msg!("Instruction: InitializeAuthorizedEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::AuthorizedEcho { data: _ } => {
                msg!("Instruction: AuthorizedEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::InitializeVendingMachineEcho {
                price: _,
                buffer_size: _,
            } => {
                msg!("Instruction: InitializeVendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
            EchoInstruction::VendingMachineEcho { data: _ } => {
                msg!("Instruction: VendingMachineEcho");
                Err(EchoError::NotImplemented.into())
            }
        }
    }
}
