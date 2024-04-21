use scrypto::prelude::*;

use crate::proposal_result_option::proposal_result_option::ProposalResultOption;
use crate::proposal_vote::proposal_vote::ProposalVote;
use crate::proposal_option::proposal_option::ProposalOption;

#[blueprint]
mod proposal_result {

    struct ProposalResult {
        vote_count: u32,
        vote_power: Decimal,
        results: Vec<ProposalResultOption>,
        additional_data: HashMap<String, String>,
        additional_data_vec: HashMap<String, Vec<String>>,
    }

    impl ProposalResult {
        pub(crate) fn new(vote_count: u32, vote_power: Decimal, results: Vec<ProposalResultOption>) -> ProposalResult {
            let result = Self {
                vote_count,
                vote_power,
                results,
                additional_data: HashMap::new(),
                additional_data_vec: HashMap::new(),
            };

            result
        }
    }
}

pub(crate) fn calc_result(votes: &Vec<ProposalVote>, options: &Vec<ProposalOption>) -> proposal_result::ProposalResult {
    let mut option_power: HashMap<String, Decimal> = HashMap::new();
    for option in (*options).iter() {
        option_power.insert(option.id(), Decimal::zero());
    }

    let mut votes_count: u32 = 0;
    let mut all_votes_power: Decimal = Decimal::zero();
    for vote in (*votes).iter() {
        votes_count += 1;
        all_votes_power += Decimal::from(vote.nfts().len());
        option_power.insert(vote.option_id(), option_power[&vote.option_id()] + vote.power());
    }

    let mut result_options: Vec<ProposalResultOption> = Vec::new();
    for (option_id, option_power) in option_power.iter() {
        let option = (*options).iter().find(|option| option.id() == *option_id).unwrap();
        let option_share = *option_power / all_votes_power;
        let result_option = ProposalResultOption::new((*option_id).clone(), option.option(), option_share);
        result_options.push(result_option);
    }

    return proposal_result::ProposalResult::new(votes_count, all_votes_power, result_options);
}