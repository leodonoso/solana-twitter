use anchor_lang::prelude::*;

declare_id!("Cmx8tcBfPpmv6vJ1sqonPTHESYJpnTSgS8K7DD1TfpfM");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
