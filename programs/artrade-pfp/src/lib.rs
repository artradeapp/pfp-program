pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("dMtyD9gkB41frDGEp4YuTmtv6fzdsjib3PGv7YhrVSM");

#[program]
pub mod artrade_pfp {
    use super::*;

    pub fn mint_pfp(ctx: Context<MintPfp>, pfp_type: String, name: String, uri: String) -> Result<()> {
        mint_pfp::handler(ctx, pfp_type, name, uri)
    }
}
