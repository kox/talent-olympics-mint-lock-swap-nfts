# Mint, Lock, and Swap NFTs Protocol

This protocol allows users to mint, lock, and swap NFTs using the Anchor framework and the Metaplex mpl-core library. 

The protocol includes functionalities for creating collections, creating assets, locking assets in a vault, and swapping assets from the vault.


## Overview
This protocol facilitates the following operations:

- Initialize the protocol with a rent fee.
- Create a collection of NFTs.
- Create individual NFTs within the collection.
- Lock NFTs in a vault with a specified rent fee.
- Swap NFTs from the vault to a new owner, with payment to the previous owner.


## Instructions
### Initialize Protocol

Initialize the protocol with a specified rent fee.

Arguments: 
1. ctx: Context<InitializeProtocol>
2. rent_fee: u64 - The rent fee for locking assets.

```rust
pub fn initialize_protocol(ctx: Context<InitializeProtocol>, rent_fee: u64) -> Result<()> {
    ctx.accounts.initialize_protocol(rent_fee, ctx.bumps.protocol)
}
```

### Create Collection
Create a new NFT collection.

Arguments:
1. ctx: Context<CreateCollection>
2. args: CreateCollectionArgs

```rust
pub fn create_collection(ctx: Context<CreateCollection>, args: CreateCollectionArgs) -> Result<()> {
    ctx.accounts.create_collection(args)
}
```

### Create Asset
Create a new NFT within a collection.

Arguments:
1. ctx: Context<CreateAsset>
2. args: CreateAssetArgs

```rust
pub fn create_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
    ctx.accounts.create_asset(args)
}
```

### Lock Asset
Lock an NFT in a vault, paying the specified rent fee.

Arguments:
1. ctx: Context<LockAsset>

```rust
pub fn lock_asset(ctx: Context<LockAsset>) -> Result<()> {
    ctx.accounts.lock_asset(ctx.bumps.vault)
}
```

### Swap Asset
Swap an NFT from the vault to a new owner, with payment to the previous owner.

Arguments:
1. ctx: Context<SwapAsset>

```rust
pub fn swap_asset(ctx: Context<SwapAsset>) -> Result<()> {
    ctx.accounts.swap_asset()
}
```

## Accounts
### InitializeProtocol

```rust
#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub protocol_treasure: SystemAccount<'info>,
    #[account(
        init, 
        payer = payer,
        space = Protocol::INIT_SPACE,
        seeds = [PROTOCOL_SEED, payer.key().as_ref()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
```

### CreateCollection
```rust
#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    pub update_authority: Option<AccountInfo<'info>>,
    #[account(address = ID)]
    pub mpl_core: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
```

### CreateAsset
```rust
#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub asset: Signer<'info>,
    pub collection: Option<AccountInfo<'info>>,
    pub authority: Option<Signer<'info>>,
    pub owner: Option<AccountInfo<'info>>,
    pub update_authority: Option<AccountInfo<'info>>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Option<AccountInfo<'info>>,
    #[account(address = ID)]
    pub mpl_core: AccountInfo<'info>,
}
```

### LockAsset
```rust
#[derive(Accounts)]
pub struct LockAsset<'info> {
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    pub collection: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Option<Signer<'info>>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [VAULT_SEED, asset.key().as_ref()],
        bump,
        space = Vault::INIT_SPACE,
    )]
    pub vault: Account<'info, Vault>,
    pub owner: UncheckedAccount<'info>,
    #[account(
        seeds = [PROTOCOL_SEED, owner.key().as_ref()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(mut)]
    pub protocol_treasure: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Option<AccountInfo<'info>>,
    #[account(address = ID)]
    pub mpl_core: AccountInfo<'info>,
}
```

### SwapAsset
```rust
#[derive(Accounts)]
pub struct SwapAsset<'info> {
    #[account(mut)]
    pub asset: AccountInfo<'info>,
    pub collection: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Option<Signer<'info>>,
    #[account(
        seeds = [VAULT_SEED, asset.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut, address = vault.previous_owner)]
    pub previous_owner: AccountInfo<'info>,
    pub owner: UncheckedAccount<'info>,
    #[account(
        seeds = [PROTOCOL_SEED, owner.key().as_ref()],
        bump
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Option<AccountInfo<'info>>,
    #[account(address = ID)]
    pub mpl_core: AccountInfo<'info>,
}
```

## Tests
✔ Airdrops funds to the main users (278.493799ms)

  ✔ Can initialize the protocol (454.223282ms)

  ✔ Can create a Collection (410.166573ms)

  ✔ Can create the assets (411.892704ms)

  ✔ Can lock the asset in the protocol vault (408.256701ms)

  ✔ Can be swapped from the protocol vault paying static amount (388.091972ms)

## Youtube Link

`https://youtu.be/vd-s7FPh5wU`
