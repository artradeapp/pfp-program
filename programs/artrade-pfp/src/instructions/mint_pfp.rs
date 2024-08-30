use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_spl::associated_token;
use std::str::FromStr;
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3Cpi, 
    CreateMetadataAccountV3CpiAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    SetAndVerifyCollectionCpi,
    SetAndVerifyCollectionCpiAccounts
};
use mpl_token_metadata::types::{DataV2, Collection};

use crate::minted_pfp::MintedPfp;

const MPL_TOKEN_PROGRAM_ADDRESS: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const ARTRADE_ADDRESS: &str = "79q4wWfBQ7j5q152qH39iwR7gp7agC33o9UtBk4dWtHP";

#[derive(Accounts)]
#[instruction(pfp_type: String,name: String,uri: String,)]
pub struct MintPfp<'info> {
  
  /// CHECK
	#[account(mut, signer)]
  pub minter: AccountInfo<'info>,
  
  /// CHECK
	#[account(mut, signer, address = Pubkey::from_str(ARTRADE_ADDRESS).unwrap())]
  pub artrade: AccountInfo<'info>,
  
  #[account(
    init,
    payer = minter,
    mint::decimals = 0,
    mint::authority = artrade,
    mint::freeze_authority = artrade,
  )]
  pub mint: Box<Account<'info, Mint>>,
  
  /// CHECK
  #[account(
    init,
    payer = minter, 
    associated_token::mint = mint, 
    associated_token::authority = minter
  )]
	pub token_account: Box<Account<'info, TokenAccount>>, 
  
  /// CHECK
  #[account(
    init,
    payer = minter,
    seeds = [pfp_type.as_ref(), minter.key().as_ref()],
    space=8+32+32,
    bump,
  )]
	pub pda_minted_pfp: Box<Account<'info, MintedPfp>>,  
  
  /// CHECK
  #[account(mut)]
  pub metadata_account: AccountInfo<'info>,
  
  /// CHECK
  pub collection: AccountInfo<'info>,
  
  /// CHECK
  pub collection_metadata: AccountInfo<'info>,
  
  /// CHECK
  pub collection_master_edition: AccountInfo<'info>,
  
  /// CHECK 
  #[account(address = Pubkey::from_str(MPL_TOKEN_PROGRAM_ADDRESS).unwrap())]
	pub mpl_program: AccountInfo<'info>,
  
  /// CHECK
	#[account(address = anchor_spl::token::ID)]
  pub token_program: AccountInfo<'info>,
  
	/// CHECK
	#[account(address = associated_token::ID)]
  pub associated_token_program: AccountInfo<'info>,
  
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
      ctx: Context<MintPfp>, 
      _pfp_type: String,
      name: String,
      uri: String,
    ) -> Result<()> {
  
    // mint to 
		let cpi_ctx = CpiContext::new(
			ctx.accounts.token_program.to_account_info(),
			token::MintTo {
				mint: ctx.accounts.mint.to_account_info(),
				to: ctx.accounts.token_account.to_account_info(),
				authority: ctx.accounts.artrade.to_account_info(),
			}
		);
		token::mint_to(cpi_ctx, 1)?;
    
    // create metadata
    CreateMetadataAccountV3Cpi::new(
        &ctx.accounts.mpl_program,
        CreateMetadataAccountV3CpiAccounts {
            metadata: &ctx.accounts.metadata_account.to_account_info(),
            mint: &ctx.accounts.mint.to_account_info(),
            mint_authority: &ctx.accounts.artrade.to_account_info(),
            payer: &ctx.accounts.minter.to_account_info(),
            update_authority: (&ctx.accounts.artrade.to_account_info(), true),
            system_program: &ctx.accounts.system_program.to_account_info(),
            rent: Some(&ctx.accounts.rent.to_account_info()),
        },
        CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name,
                symbol: "".to_string(),
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: Some(Collection {key: ctx.accounts.collection.key(), verified: false}),
                uses: None,
            },
            is_mutable: true,
            collection_details: None,
        },
    )
    .invoke()?;
    
    // verify the collection
    SetAndVerifyCollectionCpi::new(
        &ctx.accounts.mpl_program,
        SetAndVerifyCollectionCpiAccounts {
            metadata: &ctx.accounts.metadata_account.to_account_info(),
            update_authority: &ctx.accounts.artrade.to_account_info(),
            collection_authority: &ctx.accounts.artrade.to_account_info(),
            payer: &ctx.accounts.minter.to_account_info(),
            collection_mint: &ctx.accounts.collection.to_account_info(),
            collection: &ctx.accounts.collection_metadata.to_account_info(),
            collection_master_edition_account: &ctx.accounts.collection_master_edition.to_account_info(),
            collection_authority_record: None,
        },
    )
    .invoke()?;

    // update PDA
    ctx.accounts.pda_minted_pfp.pfp_mint = ctx.accounts.mint.key();
    ctx.accounts.pda_minted_pfp.minter_address = ctx.accounts.minter.key();
    
    Ok(())
}
