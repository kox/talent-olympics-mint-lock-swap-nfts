import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { MintLockSwapNfts } from "../target/types/mint_lock_swap_nfts";
import { fetchAssetV1, fetchCollection } from "@metaplex-foundation/mpl-core";
import { describe, it } from "node:test";
import { keypairIdentity, publicKey } from '@metaplex-foundation/umi'
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata'
import { assert } from "chai";

describe("mint_lock_swap_nfts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.MintLockSwapNfts as Program<MintLockSwapNfts>;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    /* console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    ); */
    return signature;
  };

  const collectionMetadata = {
    name: "Solana Talent Olympics Collection",
    uri: "https://ipfs.io/ipfs/Qmbvsn5BH7Kot8YqN3kLdgGR3Z7xXrui7YEu5b2QfqEdzM",
    plugins: []
  }

  const assetMetadata = {
    name: "Solana Talent Olympics Collection - 0",
    uri: "https://ipfs.io/ipfs/Qmbvsn5BH7Kot8YqN3kLdgGR3Z7xXrui7YEu5b2QfqEdzM",
    plugins: []
  }

  // protocol administrator
  const admin = Keypair.generate();
  // NFT creator
  const creator = Keypair.generate();
  // NFT buyer
  const user = Keypair.generate();
  // Asset and collection keypairs
  const asset = Keypair.generate();
  const collection = Keypair.generate();
  // Protocol treasure to earn rent_fees
  const protocolTreasure = Keypair.generate();
  // PDAs
  const protocolPda = PublicKey.findProgramAddressSync([
    Buffer.from("protocol"),
    admin.publicKey.toBuffer(),
  ], program.programId)[0];
  const assetVaultPda = PublicKey.findProgramAddressSync([
    Buffer.from("vault"),
    asset.publicKey.toBuffer(),
  ], program.programId)[0];
  // Rent fee for using the protocol
  const rentFee = new BN(0.5 * LAMPORTS_PER_SOL);
  // For better integration with mpl_core, I'm using umi 
  const umi = createUmi('http://127.0.0.1:8899', 'confirmed')
    .use(mplTokenMetadata())
    .use(keypairIdentity(provider.wallet.payer));

  it('Airdrops funds to the main users', async () => {
    let tx = new Transaction();
    
    tx.instructions = [
      ...[admin, creator, user].map((k) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: k.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      )
    ];

    await provider.sendAndConfirm(tx, [provider.wallet.payer]).then(log);
  });

  it('Can initialize the protocol', async () => {
    await program.methods
      .initializeProtocol(rentFee)
      .accounts({
        payer: admin.publicKey,
        protocol: protocolPda,
        protocolTreasure: protocolTreasure.publicKey,
      })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);

    const protocolAccount = await program.account.protocol.fetch(protocolPda);
    assert.equal(protocolAccount.rentFee.toNumber(), rentFee.toNumber());
  });

  it('Can create a Collection', async () => {
    await program.methods
      .createCollection(collectionMetadata)
      .accounts({
        collection: collection.publicKey,
        payer: creator.publicKey,
        updateAuthority: creator.publicKey,
      })
      .signers([creator, collection])
      .rpc()
      .then(confirm)
      .then(log);


    const collectionData = await fetchCollection(umi, collection.publicKey.toString());

    assert.equal(collectionData.name, collectionMetadata.name);
    assert.equal(collectionData.uri, collectionMetadata.uri);
    assert.equal(collectionData.numMinted, 0);
    assert.equal(collectionData.updateAuthority, creator.publicKey.toString());
  })

  it('Can create the assets', async () => {
    await program.methods
      .createAsset(assetMetadata)
      .accounts({
        asset: asset.publicKey,
        collection: collection.publicKey,
        payer: creator.publicKey,
        authority: creator.publicKey,
        owner: creator.publicKey,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([creator, asset])
      .rpc()
      .then(confirm)
      .then(log);

    const assetData = await fetchAssetV1(umi, publicKey(asset.publicKey.toString()));

    assert.equal(assetData.name, assetMetadata.name);
    assert.equal(assetData.uri, assetMetadata.uri);
    assert.equal(assetData.updateAuthority.type, 'Collection');
    assert.equal(assetData.updateAuthority.address, collection.publicKey.toString());
  
    const collectionData = await fetchCollection(umi, collection.publicKey.toString());
    assert.equal(collectionData.numMinted, 1);
  });

  it('Can lock the asset in the protocol vault', async () => {
    const protocolTreasureBalance = await provider.connection.getBalance(protocolTreasure.publicKey);

    await program.methods
      .lockAsset()
      .accounts({
        asset: asset.publicKey,
        collection: collection.publicKey,
        payer: creator.publicKey,
        authority: creator.publicKey,
        vault: assetVaultPda,
        owner: admin.publicKey,
        protocol: protocolPda,
        protocolTreasure: protocolTreasure.publicKey,
        logWrapper: null,
      })
      .signers([creator])
      .rpc()
      .then(confirm)
      .then(log);

    // The asset is own by the protocol
    const assetData = await fetchAssetV1(umi, publicKey(asset.publicKey.toString()));
    assert.equal(assetData.owner.toString(), assetVaultPda.toString());

    // Treasure has increased
    const protocolTreasureLatestBalance = await provider.connection.getBalance(protocolTreasure.publicKey);
    assert.equal(protocolTreasureBalance, 0);
    assert.equal(protocolTreasureLatestBalance, rentFee.toNumber());

    // The vault contains the previous owner to send the money directly
    const assetVaultAccount = await program.account.vault.fetch(assetVaultPda);
    assert.equal(assetVaultAccount.previousOwner.toString(), creator.publicKey.toString());
  });

  it("Can be swapped from the protocol vault paying static amount", async () => {
    const creatorBalance = await provider.connection.getBalance(creator.publicKey);

    await program.methods
      .swapAsset()
      .accounts({
        asset: asset.publicKey,
        collection: collection.publicKey,
        payer: user.publicKey,
        vault: assetVaultPda,
        owner: admin.publicKey,
        previousOwner: creator.publicKey,
        protocol: protocolPda,
        logWrapper: null,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);

    const assetData = await fetchAssetV1(umi, publicKey(asset.publicKey.toString()));
    assert.equal(assetData.owner.toString(), user.publicKey.toString());
  
    const creatorLatestBalance = await provider.connection.getBalance(creator.publicKey);
    assert.equal(creatorLatestBalance, creatorBalance + LAMPORTS_PER_SOL);
  });
});
