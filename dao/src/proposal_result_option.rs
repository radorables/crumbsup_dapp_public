use scrypto::prelude::*;

#[blueprint]
mod proposal_result_option {
    
    struct ProposalResultOption {
        option_id: String,
        option_name: String,
        share: Decimal,
        additional_data: HashMap<String, String>,
        additional_data_vec: HashMap<String, Vec<String>>,
    }

    impl ProposalResultOption {
        pub(crate) fn new(option_id: String, option_name: String, share: Decimal) -> ProposalResultOption {
            let result_option = Self {
                option_id,
                option_name,
                share,
                additional_data: HashMap::new(),
                additional_data_vec: HashMap::new(),
            };

            result_option
        }
    }
}
