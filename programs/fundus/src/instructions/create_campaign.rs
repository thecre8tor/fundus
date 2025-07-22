use anchor_lang::prelude::*;

use crate::{
    constants::ANCHOR_DISCRIMINATOR_SIZE,
    states::{CampaignState, ProgramState},
};

#[derive(Accounts)]
pub struct CreateCampaignCtx<'info> {
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR_SIZE + CampaignState::INIT_SPACE,
        seeds = [b"campaign", &(program_state.campaign_count + 1).to_le_bytes()],
        bump
    )]
    pub campaign: Account<'info, CampaignState>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateCampaignCtx<'info> {
    pub fn create_campaign(
        context: Context<Self>,
        title: String,
        description: String,
        image_url: String,
        goal: u64,
    ) -> Result<()> {
        Ok(())
    }
}
