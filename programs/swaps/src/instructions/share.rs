//Create a tranfer_token fn that will be used to transfer token

/*
>from_token_account_pubkey
>to_token_account_pubkey
>mint_address_token(that is being transferred)
>Authority:Signer(the account that will sign the tx )
>token_amount
>token_programme
*/

fn transfer_token<'info>(from:&InterfaceAccount<'info, TokenAccount>,to:&InterfaceAccount<'info, TokenAccount>, token_amount:u64,token_mint:Pubkey, token_program: &Interface<'info, TokenInterface>,) -> Result<()>{
    let transfer_account_opt = TransferChecked {
        from: from.to_account_info(),
            mint: mint.to_account_info(),
            to: to.to_account_info(),
            authority: authority.to_account_info(),
    }
    //the context that will eb passed to the token_program(that is the smart contract responsible for the transfer of the token)
    let cpi_context = CpiContext::new(token_program.to_account_info,tranfer_accoutn_opt);
    
    //Passing the cpi_context to the token_program derived from the args of the fn()
    //IMPORTANT -> *amount dereference -> "give me the value pointed to by this reference".
    //Amount is passed in the base unit
    //mitnt.decimal is for safety check
    transfer_checked(cpi_context, *amount, mint.decimals)
    
    }