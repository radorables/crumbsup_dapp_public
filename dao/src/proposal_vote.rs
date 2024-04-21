use scrypto::prelude::*;

#[blueprint]
mod proposal_vote {
    struct ProposalVote {
        id: String,
        option_id: String,
        entity: ComponentAddress,
        power: Decimal,
        nfts: Vec<NonFungibleLocalId>,
        created: String,
        created_epoch: Epoch,
        additional_data: HashMap<String, String>,
        additional_data_vec: HashMap<String, Vec<String>>,
    }

    impl ProposalVote {
        pub(crate) fn new(
            id: String,
            option_id: String,
            entity: ComponentAddress,
            power: Decimal,
            nfts: Vec<NonFungibleLocalId>,
            created: String,
            additional_data: HashMap<String, String>,
        ) -> ProposalVote {
            let vote = Self {
                id,
                option_id,
                entity,
                power,
                nfts,
                created,
                created_epoch: Runtime::current_epoch(),
                additional_data,
                additional_data_vec: HashMap::new(),
            };

            vote
        }

        pub(crate) fn id(&self) -> String {
            self.id.clone()
        }

        pub(crate) fn option_id(&self) -> String {
            self.option_id.clone()
        }

        pub(crate) fn power(&self) -> Decimal {
            self.power.clone()
        }

        pub(crate) fn nfts(&self) -> Vec<NonFungibleLocalId> {
            self.nfts.clone()
        }
    }
}
