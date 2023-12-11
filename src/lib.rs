use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");

    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;

    // Here, you can write logic to interact with the account

    Ok(())
}

fn process_instruction(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Implement logic here
    Ok(())
}

fn mint_nft() -> ProgramResult {
    Ok(())
}

fn bulk_mint_nfts(, nfts_to_mint: u64) -> ProgramResult {
    for _ in 0 ..nfts_to_mint {
        mint_nft()?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
