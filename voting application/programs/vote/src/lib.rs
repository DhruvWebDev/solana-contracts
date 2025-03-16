use anchor_lang::prelude::*;

// This is your program's public key and it will update automatically when you build the project.
declare_id!("81nWf4vowqgHuhQyPprSSu56bwmwKDoWL54tcxmruTtc");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod vote {
    use super::*;

    pub fn cast_vote(
        ctx: Context<SetVote>,
        candidate: String,
        vote_count: i16,
        voter_name: String,
    ) -> Result<()> {
        // Get the candidate list from the Candidate PDA
        let candidate_list = &ctx.accounts.candidate.candidates;
        msg!("candidates list is : {:?}",candidate_list);
        // Check if the candidate exists in the list
        require!(
            candidate_list.contains(&candidate),
            CustomError::CandidateNotFound
        );

        msg!("Candidate you are voting for: {}", candidate);
        msg!("Vote count: {}", vote_count);
        msg!("Voter name: {}", voter_name);

        ctx.accounts.vote.set_inner(VoteStruct {
            candidate,
            vote_count,
            voter_name,
        });

        Ok(())
    }

    pub fn edit_candidate_list(
        ctx: Context<SetCandidate>,
        candidate_list: Vec<String>,
    ) -> Result<()> {
        msg!("Updated candidate list: {:?}", candidate_list);

        ctx.accounts.candidate.set_inner(CandidateStruct {
            candidates: candidate_list,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VoteStruct {
    #[max_len(50)]
    candidate: String, // Store candidate name as a String instead of Enum
    vote_count: i16,
    #[max_len(50)]
    voter_name: String,
}

#[derive(Accounts)]
pub struct SetVote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + VoteStruct::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub vote: Account<'info, VoteStruct>,

    #[account(
        seeds = [b"candidate", user.key().as_ref()],
        bump
    )]
    pub candidate: Account<'info, CandidateStruct>, // Fetch candidate list

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetCandidate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + CandidateStruct::INIT_SPACE,
        seeds =  [b"candidate", user.key().as_ref()],
        bump,
    )]
    pub candidate: Account<'info, CandidateStruct>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateStruct {
    #[max_len(10, 50)]
    candidates: Vec<String>, // Store candidates as a dynamic list
}

#[error_code]
pub enum CustomError {
    #[msg("The selected candidate is not in the list.")]
    CandidateNotFound,
}
