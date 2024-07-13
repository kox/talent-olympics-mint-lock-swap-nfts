#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod contexts;
pub mod states;
pub mod errors;
pub mod constants;
use contexts::*;

declare_id!("6FGwV4j1SekDP8Ww2RN5QYb5Rbt1KZap1mrkzGqpJAxk");

#[program]
pub mod mint_lock_swap_nfts {
    use super::*;

    pub fn initialize_protocol(ctx: Context<InitializeProtocol>, rent_fee: u64) -> Result<()> {
        ctx.accounts.initialize_protocol(rent_fee, ctx.bumps.protocol)
    }

    pub fn create_collection(ctx: Context<CreateCollection>, args: CreateCollectionArgs) -> Result<()> {
        ctx.accounts.create_collection(args)
    }

    pub fn create_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        ctx.accounts.create_asset(args)
    }

    pub fn lock_asset(ctx: Context<LockAsset>) -> Result<()> {
        ctx.accounts.lock_asset(ctx.bumps.vault)
    }

    pub fn swap_asset(ctx: Context<SwapAsset>) -> Result<()> {
        ctx.accounts.swap_asset()
    }
}

