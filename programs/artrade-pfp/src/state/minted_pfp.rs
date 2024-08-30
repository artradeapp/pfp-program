use anchor_lang::prelude::*;

#[account]
pub struct MintedPfp {
    pub pfp_mint: Pubkey,
    pub minter_address: Pubkey,
}