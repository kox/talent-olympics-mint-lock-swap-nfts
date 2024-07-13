use anchor_lang::prelude::*;

use crate::{
    constants::PROTOCOL_SEED, 
    states::Protocol
};

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Account to store the rent_fees
    pub protocol_treasure: SystemAccount<'info>,

    /// PDA with the rent_fee data
    #[account(
        init, 
        payer = payer,
        space = Protocol::INIT_SPACE,
        seeds = [PROTOCOL_SEED, payer.key().as_ref()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

    /// The system program.
    pub system_program: Program<'info, System>,
}

impl <'info> InitializeProtocol<'info> {
    pub fn initialize_protocol(&mut self, rent_fee: u64, bump: u8) -> Result<()> {
        self.protocol.set_inner(Protocol {
            rent_fee,
            bump,
        });

        Ok(())
    }
}