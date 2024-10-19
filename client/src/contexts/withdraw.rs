use crate::*;

pub fn withdraw(client: &Client, user: Pubkey, amount: u64) -> ClientResult<()> {
    let vault = Pubkey::find_program_address(&[b"vault"], &chess::ID).0;
    let withdraw_ix = Instruction {
        program_id: chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: chess::instruction::Withdraw { amount }.data(),
    };

    send_and_confirm_tx(
        &client,
        [withdraw_ix].to_vec(),
        None,
        "withdraw".to_string(),
    )?;

    Ok(())
}
