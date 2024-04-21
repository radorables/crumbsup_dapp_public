use scrypto::prelude::*;

#[blueprint]
mod proposal_option {

    struct ProposalOption {
        id: String,
        rank: u32,
        option: String,
        additional_data: HashMap<String, String>,
    }

    impl ProposalOption {
        pub(crate) fn new(id: String, rank: u32, option: String, additional_data: HashMap<String, String>,) -> ProposalOption {
            let option = Self {
                id,
                rank,
                option,
                additional_data,
            };

            option
        }
    }
}
