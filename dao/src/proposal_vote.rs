use scrypto::prelude::*;

#[blueprint]
mod proposal_vote {

    struct ProposalVote {
        id: String,
        option_id: String,
        entity: ComponentAddress,
        power: Decimal,
        created: String,
        additional_data: HashMap<String, String>,
    }

    impl ProposalVote {
        pub(crate) fn new(
            id: String,
            option_id: String,
            entity: ComponentAddress,
            power: Decimal,
            created: String,
            additional_data: HashMap<String, String>,
        ) -> ProposalVote {
            let vote = Self {
                id,
                option_id,
                entity,
                power,
                created,
                additional_data,
            };

            vote
        }
    }
}
