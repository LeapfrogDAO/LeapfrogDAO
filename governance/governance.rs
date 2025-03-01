//! LeapfrogDAO Governance Program
//!
//! A Solana program for the LeapfrogDAO governance system as described in the whitepaper.
//! This program manages proposals, voting, quadratic voting, and treasury management.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    program::invoke,
    program_pack::Pack,
    system_instruction,
    clock::Clock,
};
use spl_token::state::{Account as TokenAccount, Mint};
use std::collections::HashMap;

// Program ID would be set during deployment
solana_program::declare_id!("LeapFD1stribuTedgoverNanceTokenXXXXXXXXX");

/// LeapfrogDAO governance instruction types
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum LeapfrogInstruction {
    /// Initialize a new governance realm
    /// 
    /// Accounts expected:
    /// 0. `[signer, writable]` Funding account (must be rent exempt)
    /// 1. `[writable]` The realm account to create
    /// 2. `[]` The governance token mint
    /// 3. `[]` The council token mint (optional, can be same as governance token)
    /// 4. `[]` The system program
    InitializeRealm {
        name: String,
        min_community_tokens_to_create_proposal: u64,
        community_mint_max_vote_weight_source: MintMaxVoteWeightSource,
        use_quadratic_voting: bool,
    },

    /// Create a new proposal
    /// 
    /// Accounts expected:
    /// 0. `[signer, writable]` Proposal owner account
    /// 1. `[writable]` Proposal account to create
    /// 2. `[]` Governance account the proposal belongs to
    /// 3. `[]` Token owner record of the proposal owner
    /// 4. `[]` Governance authority
    /// 5. `[]` The system program
    CreateProposal {
        name: String,
        description_link: String,
        vote_type: VoteType,
        options: Vec<String>,
        use_denial_quorum: bool,
        voting_period_days: u8,
    },

    /// Cast a vote on a proposal
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Governance authority account
    /// 1. `[writable]` Proposal account
    /// 2. `[writable]` Token owner record of the voter
    /// 3. `[]` Governance token account of the voter
    /// 4. `[writable]` Vote record account
    CastVote {
        vote: Vote,
        staked_amount: u64,
    },

    /// Execute an approved proposal
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Governance authority
    /// 1. `[writable]` Proposal account
    /// 2. `[]` Token owner record of the proposal owner
    /// Remaining accounts are the accounts required for the instructions being executed
    ExecuteProposal,

    /// Stake tokens for voting
    /// 
    /// Accounts expected:
    /// 0. `[signer, writable]` Token owner account  
    /// 1. `[writable]` Token account to stake from
    /// 2. `[writable]` Staking vault account
    /// 3. `[writable]` Token owner record
    /// 4. `[]` The SPL Token program
    StakeTokens {
        amount: u64,
    },

    /// Unstake tokens after a cooldown period
    /// 
    /// Accounts expected:
    /// 0. `[signer, writable]` Token owner account
    /// 1. `[writable]` Staking vault account
    /// 2. `[writable]` Token account to receive unstaked tokens
    /// 3. `[writable]` Token owner record
    /// 4. `[]` The SPL Token program
    /// 5. `[]` The clock sysvar
    UnstakeTokens {
        amount: u64,
    },
}

/// Vote types supported by the governance program
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum VoteType {
    /// Single choice vote
    SingleChoice,
    
    /// Multiple choice vote (select K from N)
    MultiChoice { max_voter_options: u8 },
    
    /// Weighted vote (allocate weights to choices)
    Weighted,
}

/// Vote 
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub enum Vote {
    /// Single choice vote
    SingleChoice { option_index: u8 },
    
    /// Multiple choice vote
    MultiChoice { option_indices: Vec<u8> },
    
    /// Weighted vote
    Weighted { weights: Vec<(u8, u8)> }, // (option_index, weight)
}

/// Mint max vote weight source
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone, Copy)]
pub enum MintMaxVoteWeightSource {
    /// Use the full supply as a source of max vote weight
    SupplyFraction { fraction: u64 },
    
    /// Use a fixed absolute value as a source of max vote weight
    Absolute { value: u64 },
}

/// Proposal state
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone, Copy)]
pub enum ProposalState {
    /// Draft - the proposal is being created and edited
    Draft,
    
    /// Active - the proposal is active for voting
    Active,
    
    /// Approved - the proposal has been approved
    Approved,
    
    /// Rejected - the proposal has been rejected
    Rejected,
    
    /// Executed - the proposal has been executed
    Executed,
    
    /// Expired - the proposal has expired
    Expired,
}

/// Realm account
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct Realm {
    /// Governance program account type
    pub account_type: AccountType,
    
    /// Name of the realm
    pub name: String,
    
    /// Community mint
    pub community_mint: Pubkey,
    
    /// Optional council mint
    pub council_mint: Option<Pubkey>,
    
    /// Min community tokens required to create a proposal
    pub min_community_tokens_to_create_proposal: u64,
    
    /// Community mint max vote weight source
    pub community_mint_max_vote_weight_source: MintMaxVoteWeightSource,
    
    /// Whether to use quadratic voting
    pub use_quadratic_voting: bool,
    
    /// Reserved space for future versions
    pub reserved: [u8; 64],
}

/// Proposal account
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct Proposal {
    /// Governance program account type
    pub account_type: AccountType,
    
    /// Governance account the proposal belongs to
    pub governance: Pubkey,
    
    /// Proposal owner who created the proposal
    pub proposal_owner: Pubkey,
    
    /// Name of the proposal
    pub name: String,
    
    /// Link to the description of the proposal
    pub description_link: String,
    
    /// The time the proposal was created
    pub created_at: u64,
    
    /// Current state of the proposal
    pub state: ProposalState,
    
    /// Vote type for the proposal
    pub vote_type: VoteType,
    
    /// Options for the proposal
    pub options: Vec<String>,
    
    /// Whether the proposal uses a denial quorum
    pub use_denial_quorum: bool,
    
    /// Timestamp when voting on the proposal starts
    pub voting_starts_at: u64,
    
    /// Timestamp when voting on the proposal ends
    pub voting_ends_at: u64,
    
    /// Vote results
    pub vote_results: HashMap<u8, u64>,
    
    /// Total vote weight cast
    pub total_vote_weight: u64,
    
    /// Reserved space for future versions
    pub reserved: [u8; 64],
}

/// Token owner record
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct TokenOwnerRecord {
    /// Governance program account type
    pub account_type: AccountType,
    
    /// Realm account the token owner belongs to
    pub realm: Pubkey,
    
    /// Governing token mint the token owner deposits belong to
    pub governing_token_mint: Pubkey,
    
    /// Token owner
    pub governing_token_owner: Pubkey,
    
    /// Deposit amount of governing tokens
    pub governing_token_deposit_amount: u64,
    
    /// Unrelinquished vote count
    pub unrelinquished_votes_count: u32,
    
    /// The optimal time when tokens can be unstaked
    pub earliest_unstaking_time: u64,
    
    /// Reserved space for future versions
    pub reserved: [u8; 64],
}

/// Vote record
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct VoteRecord {
    /// Governance program account type
    pub account_type: AccountType,
    
    /// Proposal being voted on
    pub proposal: Pubkey,
    
    /// Token owner which cast this vote
    pub governing_token_owner: Pubkey,
    
    /// How the vote was cast
    pub vote: Vote,
    
    /// The amount of tokens used for the vote (before quadratic calculation)
    pub stake_amount: u64,
    
    /// The weight of the vote after applying quadratic voting if enabled
    pub vote_weight: u64,
    
    /// Whether the vote has been relinquished by the voter
    pub is_relinquished: bool,
    
    /// Reserved space for future versions
    pub reserved: [u8; 64],
}

/// Governance program account types
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone, Copy)]
pub enum AccountType {
    /// Uninitialized account
    Uninitialized,
    
    /// Realm account
    Realm,
    
    /// Proposal account
    Proposal,
    
    /// Token owner record account
    TokenOwnerRecord,
    
    /// Vote record account
    VoteRecord,
}

// Program entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = LeapfrogInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        LeapfrogInstruction::InitializeRealm { 
            name, 
            min_community_tokens_to_create_proposal, 
            community_mint_max_vote_weight_source,
            use_quadratic_voting,
        } => {
            msg!("Instruction: Initialize Realm");
            process_initialize_realm(
                program_id,
                accounts,
                name,
                min_community_tokens_to_create_proposal,
                community_mint_max_vote_weight_source,
                use_quadratic_voting,
            )
        }
        LeapfrogInstruction::CreateProposal { 
            name, 
            description_link, 
            vote_type, 
            options, 
            use_denial_quorum,
            voting_period_days,
        } => {
            msg!("Instruction: Create Proposal");
            process_create_proposal(
                program_id,
                accounts,
                name,
                description_link, 
                vote_type,
                options,
                use_denial_quorum,
                voting_period_days,
            )
        }
        LeapfrogInstruction::CastVote { vote, staked_amount } => {
            msg!("Instruction: Cast Vote");
            process_cast_vote(program_id, accounts, vote, staked_amount)
        }
        LeapfrogInstruction::ExecuteProposal => {
            msg!("Instruction: Execute Proposal");
            process_execute_proposal(program_id, accounts)
        }
        LeapfrogInstruction::StakeTokens { amount } => {
            msg!("Instruction: Stake Tokens");
            process_stake_tokens(program_id, accounts, amount)
        }
        LeapfrogInstruction::UnstakeTokens { amount } => {
            msg!("Instruction: Unstake Tokens");
            process_unstake_tokens(program_id, accounts, amount)
        }
    }
}

/// Process InitializeRealm instruction
pub fn process_initialize_realm(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    min_community_tokens_to_create_proposal: u64,
    community_mint_max_vote_weight_source: MintMaxVoteWeightSource,
    use_quadratic_voting: bool,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Extract accounts
    let funder_info = next_account_info(account_info_iter)?;
    let realm_info = next_account_info(account_info_iter)?;
    let community_mint_info = next_account_info(account_info_iter)?;
    let council_mint_info = next_account_info(account_info_iter).ok();
    let system_program_info = next_account_info(account_info_iter)?;

    // Check signer
    if !funder_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Validate realm account
    if realm_info.owner != program_id {
        // Create realm account if it doesn't exist
        let realm_size = Realm::get_max_size(&name)?;
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(realm_size);
        
        // Create account
        invoke(
            &system_instruction::create_account(
                funder_info.key,
                realm_info.key,
                rent_lamports,
                realm_size as u64,
                program_id,
            ),
            &[funder_info.clone(), realm_info.clone(), system_program_info.clone()],
        )?;
    }
    
    // Create and save realm data
    let realm = Realm {
        account_type: AccountType::Realm,
        name,
        community_mint: *community_mint_info.key,
        council_mint: council_mint_info.map(|info| *info.key),
        min_community_tokens_to_create_proposal,
        community_mint_max_vote_weight_source,
        use_quadratic_voting,
        reserved: [0; 64],
    };
    
    realm.serialize(&mut *realm_info.data.borrow_mut())?;
    
    Ok(())
}

/// Process CreateProposal instruction
pub fn process_create_proposal(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    description_link: String,
    vote_type: VoteType,
    options: Vec<String>,
    use_denial_quorum: bool,
    voting_period_days: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Extract accounts
    let proposal_owner_info = next_account_info(account_info_iter)?;
    let proposal_info = next_account_info(account_info_iter)?;
    let governance_info = next_account_info(account_info_iter)?;
    let token_owner_record_info = next_account_info(account_info_iter)?;
    let governance_authority_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Check signer
    if !proposal_owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Validate accounts and ensure the proposal creator has enough tokens
    let token_owner_record = TokenOwnerRecord::deserialize(&token_owner_record_info.data.borrow())?;
    
    // Create proposal account if it doesn't exist
    if proposal_info.owner != program_id {
        let proposal_size = Proposal::get_max_size(&name, &description_link, &options)?;
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(proposal_size);
        
        // Create account
        invoke(
            &system_instruction::create_account(
                proposal_owner_info.key,
                proposal_info.key,
                rent_lamports,
                proposal_size as u64,
                program_id,
            ),
            &[proposal_owner_info.clone(), proposal_info.clone(), system_program_info.clone()],
        )?;
    }
    
    // Get the current clock for timestamps
    let clock = Clock::get()?;
    
    // Create and save proposal data
    let mut vote_results = HashMap::new();
    for i in 0..options.len() {
        vote_results.insert(i as u8, 0);
    }
    
    let proposal = Proposal {
        account_type: AccountType::Proposal,
        governance: *governance_info.key,
        proposal_owner: *proposal_owner_info.key,
        name,
        description_link,
        created_at: clock.unix_timestamp as u64,
        state: ProposalState::Draft,
        vote_type,
        options,
        use_denial_quorum,
        voting_starts_at: clock.unix_timestamp as u64,
        voting_ends_at: (clock.unix_timestamp + (voting_period_days as i64 * 86400)) as u64,
        vote_results,
        total_vote_weight: 0,
        reserved: [0; 64],
    };
    
    proposal.serialize(&mut *proposal_info.data.borrow_mut())?;
    
    Ok(())
