#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("BgBqnzoDwekLrMWCPJXcbx3V7naXVgqHZzXNjHi6Lgvb");

#[program]
pub mod fundus {
    use super::*;


}
