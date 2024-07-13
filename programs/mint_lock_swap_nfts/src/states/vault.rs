use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub previous_owner: Pubkey,

    pub bump: u8,
    
}

impl Space for Vault {
    const INIT_SPACE: usize = 8 + 32 + 1;
}
