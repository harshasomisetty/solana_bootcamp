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

pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

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
                let account = next_account_info(accounts_iter)?;

                let mut data_arr = account.try_borrow_mut_data()?;

                let write_size = cmp::min(data_arr.len(), data.len());

                for n in 0..data_arr.len() {
                    msg!("nonzero rip");
                    return Err(EchoError::NonzeroData.into());
                }

                for n in 0..write_size {
                    data_arr[n] = data[n];
                }

                Ok(())
            }
            EchoInstruction::InitializeAuthorizedEcho {
                buffer_seed,
                buffer_size,
            } => {
                msg!("Instruction: InitauthEcho");
                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                let (authorized_buffer_key, bump) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes(),
                    ],
                    _program_id,
                );

                assert_with_msg(
                    authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Authority must sign",
                )?;
                invoke_signed(
                    &system_instruction::create_account(
                        authority.key,
                        authorized_buffer.key,
                        Rent::get()?.minimum_balance(buffer_size),
                        buffer_size.try_into().unwrap(),
                        _program_id,
                    ),
                    &[
                        authority.clone(),
                        authorized_buffer.clone(),
                        system_program.clone(),
                    ],
                    &[&[
                        b"authority",
                        authority.key.as_ref(),
                        &buffer_seed.to_le_bytes(),
                        &[bump],
                    ]],
                )?;
                let mut data_arr = authorized_buffer.try_borrow_mut_data()?;

                data_arr[0] = bump;
                data_arr[1..9].clone_from_slice(&buffer_seed.to_le_bytes());

                Ok(())
            }
            EchoInstruction::AuthorizedEcho { data } => {
                msg!("Instruction: authEcho");

                let authorized_buffer = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;

                let mut data_arr = authorized_buffer.try_borrow_mut_data()?;

                let (authorized_buffer_key, bump) = Pubkey::find_program_address(
                    &[
                        b"authority",
                        authority.key.as_ref(),
                        data_arr[1..9].as_ref(),
                    ],
                    _program_id,
                );

                assert_with_msg(
                    &authorized_buffer_key != authorized_buffer.key,
                    EchoError::DifferentAuthority.into(),
                    "Authorities are different",
                );

                assert_with_msg(
                    authority.is_signer,
                    EchoError::NotSigner.into(),
                    "Authority is not the signer",
                );

                msg!("authorized buff key computed {:?}", authorized_buffer_key);
                msg!("authority key {:?}", authorized_buffer.key);

                let write_size = cmp::min(data_arr.len() - 9, data.len());

                // msg!("data {:?}", data);
                msg!("data arr {:?}", data_arr);
                // msg!("write size {:?}", write_size);
                for n in 9..(data_arr.len()) {
                    data_arr[n] = 0
                }
                msg!("cleaned data arr {:?}", data_arr);
                for n in 0..write_size {
                    data_arr[n + 9] = data[n];
                }
                msg!("after data arr {:?}", data_arr);

                Ok(())
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
