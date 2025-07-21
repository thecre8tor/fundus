use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::AlreadyInitialized;
use crate::states::ProgramState;

#[derive(Accounts)]
pub struct AppContext<'info> {
    #[account(init, payer = deployer, space = ANCHOR_DISCRIMINATOR_SIZE + ProgramState::INIT_SPACE, seeds = [b"program_state"], bump)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub deployer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AppContext<'info> {
    pub fn initialize(context: Context<Self>) -> Result<()> {
        let state = &mut context.accounts.program_state;
        let deployer = &context.accounts.deployer;

        if state.initialized {
            return Err(AlreadyInitialized.into());
        }

        state.campaign_count = 0;
        state.platform_fee = 5; // percentage value
        state.platform_address = deployer.key();
        state.initialized = true;

        Ok(())
    }
}
