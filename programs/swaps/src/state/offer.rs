use anchor_lang::prelude::*;

#[account]
//Trait for deriving the space used by this struct, that will be needed in the pda 

/*
> the offer should have:
  > id
  > maker's pubkey,
  > token_mint_a(mint address of the token a to ensure that it is a valid token)
  >token_mint_b(the mint address of the token b, you want in exchange of the token_mint_a)
  >token_b_amount(the amount fo thr token b you want in exchange of the token a)
  >Bump -> The bump is a nonce value used in combination with a Program Derived Address (PDA) to ensure that the address is valid and doesn't collide with any private key.
*/

#[derive(InitSpace)]
pub struct Offer {
    pub id:u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b:Pubkey,
    pub bump:u8,
}