use crate::proposal_option::proposal_option::ProposalOption;
use crate::proposal_result::proposal_result::ProposalResult;
use crate::proposal_vote::proposal_vote::ProposalVote;
use crate::utils;
use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct Proposal {
    proposal_id: String,
    name: String,
    description: String,
    title: String,
    p_abstract: String,
    specification: String,
    voting_start: String,
    voting_end: String,
    created: String,
    #[mutable]
    info_url: Url,
    #[mutable]
    key_image_url: Url,
    #[mutable]
    additional_data: HashMap<String, String>,
    #[mutable]
    options: Vec<ProposalOption>,
    #[mutable]
    votes: Vec<ProposalVote>,
    #[mutable]
    result: Option<ProposalResult>,
}

pub(crate) fn create_proposal(
    id: String,
    title: String,
    p_abstract: String,
    specification: String,
    voting_start: String,
    voting_end: String,
    created: String,
    info_url: String,
    key_image_url: Url,
    additional_data: HashMap<String, String>,
) -> Proposal {
    let info_unchecked_url = Url::of(info_url);
    let proposal = Proposal {
        proposal_id: id,
        name: title.clone(),
        description: p_abstract.clone(),
        title,
        p_abstract,
        specification,
        voting_start,
        voting_end,
        created,
        info_url: info_unchecked_url,
        key_image_url,
        additional_data,
        options: Vec::new(),
        votes: Vec::new(),
        result: None,
    };

    proposal
}

pub(crate) fn create_proposal_resource_manager(dao_name: &str, icon_url: Url) -> ResourceManager {
    let name = format!("Proposals of {}", dao_name);
    let description = format!("These are the Proposals for {}", dao_name);
    let resource_manager = ResourceBuilder::new_string_non_fungible::<Proposal>(OwnerRole::None)
        .metadata(metadata!(
         init {
           "name" => name, updatable;
           "description" => description, updatable;
           "icon_url" => icon_url, updatable;
           "tags" => vec!["CrumbsUp", "DAO", "Proposal"], updatable;
         }
        ))
        .mint_roles(mint_roles! {
            minter => rule!(allow_all);
            minter_updater => rule!(deny_all);
        })
        .non_fungible_data_update_roles(non_fungible_data_update_roles!(
            non_fungible_data_updater => rule!(allow_all);
            non_fungible_data_updater_updater => rule!(deny_all);
        ))
        .create_with_no_initial_supply();

    resource_manager
}

pub(crate) fn add_option(
    proposal_resource_manager: ResourceManager,
    proposal_id: String,
    option_id: String,
    rank: u32,
    option: String,
    additional_data: HashMap<String, String>,
) {
    let proposal_fungible_id = utils::create_non_fungible_id_of_uuid(&proposal_id);
    let mut proposal: Proposal =
        proposal_resource_manager.get_non_fungible_data(&proposal_fungible_id);

    let option = ProposalOption::new(option_id, rank, option, additional_data);
    proposal.options.push(option);
    proposal_resource_manager.update_non_fungible_data(
        &proposal_fungible_id,
        "options",
        proposal.options,
    );
}

pub(crate) fn add_vote(
    proposal_resource_manager: ResourceManager,
    proposal_id: String,
    proposal_vote_id: String,
    proposal_option_id: String,
    entity: ComponentAddress,
    power: Decimal,
    created: String,
    additional_data: HashMap<String, String>,
) {
    let proposal_fungible_id = utils::create_non_fungible_id_of_uuid(&proposal_id);
    let mut proposal: Proposal =
        proposal_resource_manager.get_non_fungible_data(&proposal_fungible_id);

    let vote = ProposalVote::new(proposal_vote_id, proposal_option_id, entity, power, created, additional_data);
    proposal.votes.push(vote);
    proposal_resource_manager.update_non_fungible_data(
        &proposal_fungible_id,
        "votes",
        proposal.votes,
    );
}

pub(crate) fn add_result(
    proposal_resource_manager: ResourceManager,
    proposal_id: String,
    vote_count: u32,
    vote_power: Decimal,
    results: HashMap<String, Decimal>,
    additional_data: HashMap<String, String>,
) {
    let proposal_fungible_id = utils::create_non_fungible_id_of_uuid(&proposal_id);
    let mut proposal: Proposal =
        proposal_resource_manager.get_non_fungible_data(&proposal_fungible_id);

    let result = ProposalResult::new(vote_count, vote_power, results, additional_data);
    proposal.result = Some(result);
    proposal_resource_manager.update_non_fungible_data(
        &proposal_fungible_id,
        "result",
        proposal.result,
    );
}
