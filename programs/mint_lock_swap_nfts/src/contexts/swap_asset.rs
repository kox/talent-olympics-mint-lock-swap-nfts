use anchor_lang::{
    prelude::*, 
    solana_program::native_token::LAMPORTS_PER_SOL
};
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
pub struct SwapAsset<'info> {
    /// The address of the asset.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The account buying the asset
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The owner or delegate of the asset.
    pub authority: Option<Signer<'info>>,

    /// The PDA owning the asset
    /// CHECK: Just a destination, no checks needed.
    #[account(
        seeds = [VAULT_SEED, asset.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,

    /// CHECK: This is safe because we are only using this to reference the previous owner
    #[account(mut, address = vault.previous_owner)]
    pub previous_owner: AccountInfo<'info>,

    // Protocol owner
    /// CHECK: Used for the seed
    pub owner: UncheckedAccount<'info>,

    /// PDA containing rent_fee information
    #[account(
        seeds = [PROTOCOL_SEED, owner.key().as_ref()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

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


impl <'info> SwapAsset<'info> {
    pub fn swap_asset(&mut self) -> Result<()> {
        // Paying for the NFT
        anchor_lang::system_program::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: self.payer.to_account_info(),
                    to: self.previous_owner.to_account_info(),
                }
            ),
            LAMPORTS_PER_SOL, // 1 SOL PER NFT. this is just a simplified way
        )?;

        // Transferring to the new owner
        TransferV1Cpi {
            asset: &self.asset.to_account_info(),
            collection: self.collection.as_ref(),
            payer: &self.payer.to_account_info(),
            authority: Some(&self.vault.to_account_info()),
            new_owner: &self.payer.to_account_info(),
            system_program: Some(&self.system_program.to_account_info()),
            log_wrapper: self.log_wrapper.as_ref(),
            __program: &self.mpl_core.to_account_info(),
            __args: TransferV1InstructionArgs {
                compression_proof: None,
            },
        }.invoke_signed(&[&[
            VAULT_SEED,
            self.asset.key().as_ref(),
            &[self.vault.bump],
        ]])?;

        Ok(())
    }
}
      