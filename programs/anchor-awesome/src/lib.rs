use anchor_lang::prelude::*;

declare_id!("2ihpd9SPDQk2EmT7WBdBoMMVcLwYfF5U4mbmf6TdBB4A");

#[program]
pub mod anchor_awesome {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
      
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            votes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn update_item(ctx: Context<UpdateItem>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        
        for item in base_account.gif_list.iter_mut() {
            if item.gif_link == gif_link {
                item.votes += 1;
            }
        }
        
        Ok(())
    }

    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        )

        anchor_lang.solana_program::program::invoke(
            &ix,
            &[
                ctx.acconts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ]
        )
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    to: AccountInfo<'info>
    system_program: Program<'info, System>,
}

// Create a custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u8,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct
    pub gif_list: Vec<ItemStruct>,    
}
