use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // this macro determine the initial space of this account
pub struct ProgramState {
    pub initialized: bool,
    pub campaign_count: u64,
    pub platform_fee: u64,
    pub platform_address: Pubkey,
}
