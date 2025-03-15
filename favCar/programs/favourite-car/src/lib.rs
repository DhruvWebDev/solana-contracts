use anchor_lang::prelude::*;

declare_id!("DXWFenFFGXHgve9HeDNFbDArMrdqNHa4UBKCsqV7DybS");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourite_car {
    use super::*;

    pub fn set_fav_car(ctx: Context<SetFavCar>, car: String) -> Result<()> {
        require!(car.len() <= 10, CustomError::CarNameTooLong);

        msg!("The user_pub_key: {}", ctx.accounts.user.key());
        msg!("The car is {}", car);

        ctx.accounts
            .favourite_car_pda
            .set_inner(FavouriteCarStruct { car });

        Ok(())
    }
}

#[error_code]
pub enum CustomError {
    #[msg("Car name exceeds the allowed length of 10 characters.")]
    CarNameTooLong,
}

#[derive(Accounts)]
#[instruction()]
pub struct SetFavCar<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + FavouriteCarStruct::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favourite_car_pda: Account<'info, FavouriteCarStruct>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct FavouriteCarStruct {
    #[max_len(10)]
    pub car: String,
}
