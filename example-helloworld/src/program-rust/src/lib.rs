use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::instruction::Instruction;
use solana_program::instruction::AccountMeta;
use solana_program::program::invoke_signed;
use solana_program::clock::Epoch;
use std::mem;
/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
    let nonce =_instruction_data[0];
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;
    let authority_info = next_account_info(accounts_iter)?;
    let create_account_programm_info = next_account_info(accounts_iter)?;
    let program_id1_info= next_account_info(accounts_iter)?;
    let mut buf = Vec::new();
    let mut vac_accounts = Vec::new();
    let instruction:u8 = 0;
    buf.push(instruction);

    let mut lamports = 0;
    let mut data = vec![0; mem::size_of::<u32>()];
    let token_swap=Pubkey::new(b"DTgQwyJ1qPSmgi4mFisZorqynErAzXTGYkmGW2iheT7N");
    let account_token_swap_new;
   
    let owner = Pubkey::new(b"5e2zZzHS8P1kkXQFVoBN6tVN15QBUHMDisy6mxVwVYSz");
    account_token_swap_new = AccountInfo::new(
                    &token_swap,
                    false,
                    true,
                    &mut lamports,
                    &mut data,
                    &owner,
                    false,
                    Epoch::default(),
                );
            
    vac_accounts.push(AccountMeta::new_readonly(*account_token_swap_new.key, false)); 
    //vac_accounts.push(AccountMeta::new(*create_account_programm_info.key, false));

    msg!("3");
    let ix = Instruction {
        accounts:vac_accounts,
        program_id: *program_id1_info.key,
        data: buf,
   };

   let result = invoke_signed(&ix, 
    &[account_token_swap_new.clone()],
    &[&[&create_account_programm_info.key.to_bytes()[..32], &[nonce]]]
    )? ;

    msg!("account1 {:?}",account.key);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
