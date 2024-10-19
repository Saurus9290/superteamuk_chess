use clockwork_client::thread::state::Thread;

use crate::*;

pub fn move_piece(
    client: &Client,
    user: Pubkey,
    game: Pubkey,
    from: chess::Square,
    to: chess::Square,
) -> ClientResult<()> {
    let game_thread = Thread::pubkey(game, "game_thread".to_string());

    let move_piece_ix = Instruction {
        program_id: chess::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(user, false),
            AccountMeta::new(user, false),
            AccountMeta::new(game_thread, false),
            AccountMeta::new_readonly(thread::ID, false),
            AccountMeta::new(game, false),
            AccountMeta::new_readonly(clock::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: chess::instruction::MovePiece { from, to }.data(),
    };

    send_and_confirm_tx(
        &client,
        [move_piece_ix].to_vec(),
        None,
        "move_piece".to_string(),
    )?;

    Ok(())
}
