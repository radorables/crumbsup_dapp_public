use scrypto::prelude::*;

#[blueprint]
mod proposal_option {
    struct ProposalOption {
        id: String,
        rank: u32,
        option: String,
        additional_data: HashMap<String, String>,
        additional_data_vec: HashMap<String, Vec<String>>,
    }

    impl ProposalOption {
        pub(crate) fn new(id: String, rank: u32, option: String, additional_data: HashMap<String, String>) -> ProposalOption {
            let option = Self {
                id,
                rank,
                option,
                additional_data,
                additional_data_vec: HashMap::new(),
            };

            option
        }

        pub(crate) fn id(&self) -> String {
            self.id.clone()
        }

        pub(crate) fn option(&self) -> String {
            self.option.clone()
        }

        pub(crate) fn rank(&self) -> u32 {
            self.rank.clone()
        }
    }
}
