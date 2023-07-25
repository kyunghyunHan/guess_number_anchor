use anchor_lang::prelude::*;

declare_id!("C6y8ezsrZwT3h4SEAsPHc6A1V2Gep5kv1WmEjDN7PGCW");

#[program]
pub mod guess_number {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>,target_number:u8) -> ProgramResult{
        let game= &mut ctx.accounts.game;
        game.target_number= target_number;
        Ok(())
    }
    pub fn guess(ctx: Context<Guess>,guessed_number:u8)->ProgramResult {
        let game= &mut ctx.accounts.game;
        if guessed_number==game.target_number{
            msg!("축하");
        }else if guessed_number < game.target_number {
            msg!("too low");
        }else {
            msg!("too low");

        }
        Ok(())

    }
}

#[derive(Accounts)]
pub struct Initialize<`info> {
    #[account(init,payer= user,space= 8+8)]
    pub game:Account<`info,Game>, 
    #[account(mut)]
    pub user:Signer<`info>,
    pub system_program:Program<`info,System>,
}

#[derive(Accounts)]
pub struct Guess<`info> {
    #[account(mut)]
    pub game:Account<`info,Game>,
} 


#[account]
pub struct Game {
    pub target_number:u8
}
