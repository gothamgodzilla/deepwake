//! `initialize_mint` — create the $KRKN mint and its metadata account.

use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata as MetadataProgram,
    },
    token::{Mint, Token},
};

use crate::constants::{
    KRKN_DECIMALS, MINT_AUTHORITY_SEED, MINT_SEED, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_URI,
};

/// Accounts for [`crate::initialize_mint`].
///
/// `mint` and `mint_authority` are program PDAs. `metadata` is the Metaplex
/// metadata PDA (owned by the token-metadata program) and is created by CPI
/// inside the handler.
#[derive(Accounts)]
pub struct InitializeMint<'info> {
    /// Pays rent for the mint and the metadata account.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Signing PDA used as mint authority, freeze authority, and metadata update authority.
    ///
    /// CHECK: Address is the `MINT_AUTHORITY_SEED` PDA. The account holds no
    /// data; `invoke_signed` uses it only as a signer.
    #[account(seeds = [MINT_AUTHORITY_SEED], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    /// The $KRKN mint. One canonical account, derived from `MINT_SEED`.
    #[account(
        init,
        payer = payer,
        seeds = [MINT_SEED],
        bump,
        mint::decimals = KRKN_DECIMALS,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority
    )]
    pub mint: Account<'info, Mint>,

    /// Metaplex metadata account for `mint`. Created by the token-metadata program.
    ///
    /// CHECK: Seeds are the Metaplex metadata PDA. The account is created by
    /// `create_metadata_accounts_v3` in the handler, not by this program.
    #[account(
        mut,
        seeds = [
            b"metadata",
            anchor_spl::metadata::ID.as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = anchor_spl::metadata::ID,
        bump
    )]
    pub metadata: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, MetadataProgram>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
    let bump = [ctx.bumps.mint_authority];
    let signer_seeds: &[&[u8]] = &[MINT_AUTHORITY_SEED, &bump];

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.key(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.mint_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                update_authority: ctx.accounts.mint_authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &[signer_seeds],
        ),
        DataV2 {
            name: TOKEN_NAME.to_string(),
            symbol: TOKEN_SYMBOL.to_string(),
            uri: TOKEN_URI.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    msg!(
        "Initialized {} ({}) mint {}",
        TOKEN_NAME,
        TOKEN_SYMBOL,
        ctx.accounts.mint.key()
    );
    Ok(())
}
