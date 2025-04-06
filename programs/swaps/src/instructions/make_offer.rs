/*
>The MakeOffer account included
 >token_a_mint
 >token_b_mint
 >maker_token_a
 >offer pda where we store the maker_offer
 >vault account (that stores the maker_token)
*/

use crate::{Offer, ANCHOR_DISCRIMINATOR_SIZE, SEED};


#[derive(Account)]
pub struct MakeOffer<'info>{
    //The #[account(mut)] is a checker that ensures the account passed is mutable
    #[account(mut)]
    pub maker: Signer,

    //The mint address of the token a
    #[account(mint::token_program = token_program)]
    pub token_a_mint_address:Pubkey

        //The mint address of the token b
    #[account(mint::token_program = token_program)]
    pub token_b_mint_address:Pubkey
    
    //Pda that will store the the offer on-chin
    //Storing it in the account itself ensures you can access or sign as that PDA later.
    #[account(
        init_if_needed,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR_SIZE + Offer::InitSpace,
        seeds:[bSEED, maker.key().as_ref(), id.to_len_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    //Vault account
    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

//Now a fn to send the offer 
//That’s precisely why you store the bump inside the Offer account struct — so that in a later instruction (like cancel_offer, accept_offer, close_offer, etc.), you can reconstruct the exact PDA
fn save_offer(context:context<MakeOffer, id: u64, token_b_wanted_amount: u64) -> Result<()> {
    context.accounts.offer.set_inner(Offer, {
        id,
        maker:context.accounts.maker.keY(,
        token_mint_a:token_a_mint_address,
        token_mint_b:token_b_mint_address,
        bump:context.bump.offer,
)  
    })
}