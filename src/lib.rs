use anchor_lang::prelude::*;

declare_id!("J4LA9iU1qVj1ZiXf8F4b55hGfQ6NnNXQfJjquuaFTBo9");

#[program]
pub mod intent_registry {
    use super::*;

    pub fn register_intent(
        ctx: Context<RegisterIntent>,
        description: String,
    ) -> Result<()> {
        let intent = &mut ctx.accounts.intent;
        intent.owner = ctx.accounts.owner.key();
        intent.description = description.clone();
        intent.created_at = Clock::get()?.unix_timestamp;
        intent.fulfilled = false;

        emit!(IntentRegistered {
            owner: intent.owner,
            description,
            created_at: intent.created_at,
        });

        Ok(())
    }

    pub fn fulfill_intent(ctx: Context<FulfillIntent>) -> Result<()> {
        let intent = &mut ctx.accounts.intent;
        require!(!intent.fulfilled, IntentError::AlreadyFulfilled);
        require!(
            ctx.accounts.owner.key() == intent.owner,
            IntentError::Unauthorized
        );

        intent.fulfilled = true;
        intent.fulfilled_at = Clock::get()?.unix_timestamp;

        emit!(IntentFulfilled {
            owner: intent.owner,
            fulfilled_at: intent.fulfilled_at,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(description: String)]
pub struct RegisterIntent<'info> {
    #[account(
        init,
        payer = owner,
        space = IntentRecord::LEN,
        seeds = [b"intent", owner.key().as_ref(), description.as_bytes()],
        bump
    )]
    pub intent: Account<'info, IntentRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FulfillIntent<'info> {
    #[account(mut, has_one = owner)]
    pub intent: Account<'info, IntentRecord>,
    pub owner: Signer<'info>,
}

#[account]
pub struct IntentRecord {
    pub owner: Pubkey,        // 32
    pub description: String,  // 4 + 128
    pub created_at: i64,      // 8
    pub fulfilled: bool,      // 1
    pub fulfilled_at: i64,    // 8
}

impl IntentRecord {
    pub const LEN: usize = 8 + 32 + (4 + 128) + 8 + 1 + 8;
}

#[event]
pub struct IntentRegistered {
    pub owner: Pubkey,
    pub description: String,
    pub created_at: i64,
}

#[event]
pub struct IntentFulfilled {
    pub owner: Pubkey,
    pub fulfilled_at: i64,
}

#[error_code]
pub enum IntentError {
    #[msg("Intent already fulfilled")]
    AlreadyFulfilled,
    #[msg("Unauthorized")]
    Unauthorized,
}
