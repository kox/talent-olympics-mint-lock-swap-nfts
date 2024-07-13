use anchor_lang::prelude::*;

use mpl_core::{
    ID,
    types::PluginAuthorityPair,
    instructions::{ 
        CreateCollectionV1Cpi,
        CreateCollectionV1InstructionArgs
    },
};

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    // Account paying the fees
    #[account(mut)]
    pub payer: Signer<'info>,

    // Address of the new asset
    #[account(mut)]
    pub collection: Signer<'info>,

    /// The authority on the new asset.
    /// CHECK: Checked in mpl-core.
    pub update_authority: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = ID)]
    pub mpl_core: AccountInfo<'info>,
    
    /// The system program.
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

impl <'info> CreateCollection<'info> {
    pub fn create_collection(&mut self, args: CreateCollectionArgs) -> Result<()> {
        
        CreateCollectionV1Cpi {
            collection: self.collection.as_ref(),
            payer: &self.payer.to_account_info(),
            update_authority: self.update_authority.as_ref(),
            system_program: &self.system_program.to_account_info(),
            __program: &self.mpl_core,
            __args: CreateCollectionV1InstructionArgs {
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            }
        }
        .invoke()?;

        Ok(())
    }
}