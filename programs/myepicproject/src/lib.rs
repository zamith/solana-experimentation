use anchor_lang::prelude::*;

declare_id!("GtSfwNTcVWYVAo6yCG8PhXxAZkZyMUfMpqNEatJ72h7q");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            upvotes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn update_item(ctx: Context<UpdateItem>, item_id: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        if let Some(item) = base_account.gif_list.get_mut(item_id as usize) {
            item.upvotes += 1;
        }

        Ok(())
    }

    pub fn tip_submitter(ctx: Context<TipSubmitter>) -> ProgramResult {
        let user = &mut ctx.accounts.user;
        let submitter = &mut ctx.accounts.submitter;

        let transfer = anchor_lang::solana_program::system_instruction::transfer(
            &user.key(),
            &submitter.key(),
            1000
        );

        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[
                user.to_account_info(),
                submitter.to_account_info(),
            ],
        )
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(mut)]
    base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct TipSubmitter<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    submitter: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    gif_link: String,
    user_address: Pubkey,
    upvotes: u32,
}

#[account]
pub struct BaseAccount {
    total_gifs: u64,
    gif_list: Vec<ItemStruct>,
}
