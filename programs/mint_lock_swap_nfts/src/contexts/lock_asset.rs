use anchor_lang::prelude::*;
use mpl_core::{
    ID,
    instructions::{ 
        TransferV1Cpi,
        TransferV1InstructionArgs,
    },
};

use crate::{
    constants::{ 
        PROTOCOL_SEED, 
        VAULT_SEED, 
    },
    states::{ 
        Protocol,
        Vault,
    }
};

#[derive(Accounts)]
pub struct LockAsset<'info> {
    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The account owning the asset and paying for the rent
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,

    /// The new owner of the asset.
    /// CHECK: Just a destination, no checks needed.
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [VAULT_SEED, asset.key().as_ref()],
        bump,
        space = Vault::INIT_SPACE,
    )]
    pub vault: Account<'info, Vault>,

    // Protocol owner
    /// CHECK: Used for the seed
    pub owner: UncheckedAccount<'info>,    

    /// PDA containing rent_fee information
    #[account(
        seeds = [PROTOCOL_SEED, owner.key().as_ref()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

    /// Account to store the rent_fees
    #[account(mut)]
    pub protocol_treasure: SystemAccount<'info>,

    // The system program.
    pub system_program: Program<'info, System>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = ID)]
    pub mpl_core: AccountInfo<'info>,
}

impl <'info> LockAsset<'info> {
    pub fn lock_asset(&mut self, vault_bump: u8) -> Result<()> {
        let rent_fee = self.protocol.rent_fee;

        self.vault.set_inner(Vault {
            previous_owner: self.payer.key(),
            bump: vault_bump
        });

        // Paying rent_fee
        anchor_lang::system_program::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: self.payer.to_account_info(),
                    to: self.protocol_treasure.to_account_info(),
                }
            ),
            rent_fee
        )?;
       
        // Sending asset
        TransferV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            payer: &self.payer.to_account_info(),
            authority: self.authority.as_deref(),
            new_owner: &self.vault.to_account_info(),
            system_program: Some(&self.system_program.to_account_info()),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core,
            __args: TransferV1InstructionArgs {
                compression_proof: None,
            },
        }.invoke()?;

        Ok(())
    }
}