use anchor_lang::prelude::*;

#[account]
pub struct Protocol {
    pub rent_fee: u64,

    pub bump: u8,
}

impl Space for Protocol {
    const INIT_SPACE: usize = 8 + 8 + 1;
}
