use scrypto::prelude::*;

#[blueprint]
mod proposal_result {
    struct ProposalResult {
        vote_count: u32,
        vote_power: Decimal,
        results: HashMap<String, Decimal>,
        additional_data: HashMap<String, String>,
    }

    impl ProposalResult {
        pub(crate) fn new(vote_count: u32, vote_power: Decimal, results: HashMap<String, Decimal>, additional_data: HashMap<String, String>) -> ProposalResult {
            let result = Self {
                vote_count,
                vote_power,
                results,
                additional_data,
            };

            result
        }
    }
}
