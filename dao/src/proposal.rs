use scrypto::prelude::*;

use crate::proposal_option::proposal_option::ProposalOption;
use crate::proposal_result::calc_result;
use crate::proposal_result::proposal_result::ProposalResult;
use crate::proposal_vote::proposal_vote::ProposalVote;
use crate::utils;

pub(crate) fn create_resource_manager(
    owner_badge_address: ResourceAddress,
    owner_badge_access_rule: &AccessRule,
    component_address: ComponentAddress,
    dao_name: &str,
    icon_url: Url,
) -> ResourceManager {
    let name = format!("{} Proposals", dao_name);
    let description = format!("These are the Proposals for {}", dao_name);
    let resource_manager =
        ResourceBuilder::new_string_non_fungible::<Proposal>(OwnerRole::Fixed(owner_badge_access_rule.clone()))
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                    metadata_setter_updater => OWNER;
                    metadata_locker => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                    metadata_locker_updater => OWNER;
                },
                 init {
                   "name" => name, updatable;
                   "description" => description, updatable;
                   "icon_url" => icon_url, updatable;
                   "tags" => vec!["CrumbsUp", "DAO", "Proposal"], updatable;
                 }
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                        non_fungible_data_updater => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                        non_fungible_data_updater_updater => OWNER;
                    ))
            .mint_roles(mint_roles! {
                    minter => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                    minter_updater => OWNER;
                })
            .recall_roles(recall_roles! {
                    recaller => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                    recaller_updater => OWNER;
                })
            .burn_roles(burn_roles! {
                    burner => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                    burner_updater => OWNER;
                })
            .withdraw_roles(withdraw_roles! {
                    withdrawer => rule!(allow_all);
                    withdrawer_updater => OWNER;
                })
            .deposit_roles(deposit_roles! {
                    depositor => rule!(allow_all);
                    depositor_updater => OWNER;
                })
            .freeze_roles(freeze_roles! {
                    freezer => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                    freezer_updater => OWNER;
                })
            .create_with_no_initial_supply();

    resource_manager
}

#[derive(ScryptoSbor, NonFungibleData)]
pub(crate) struct Proposal {
    proposal_id: String,
    name: String,
    description: String,
    title: String,
    p_abstract: String,
    specification: String,
    dao_type: String,
    governance_resource: ResourceAddress,
    voting_start: String,
    voting_start_epoch: Epoch,
    voting_end: String,
    voting_end_epoch: Epoch,
    created: String,
    created_epoch: Epoch,
    #[mutable]
    info_url: Url,
    #[mutable]
    key_image_url: Url,
    #[mutable]
    additional_data: HashMap<String, String>,
    #[mutable]
    additional_data_vec: HashMap<String, Vec<String>>,
    #[mutable]
    options: Vec<ProposalOption>,
    #[mutable]
    votes: Vec<ProposalVote>,
    #[mutable]
    nfts_voted: HashSet<NonFungibleLocalId>,
    #[mutable]
    result: Option<ProposalResult>,
}

pub(crate) fn create(
    id: String,
    title: String,
    p_abstract: String,
    specification: String,
    dao_type: String,
    governance_resource: ResourceAddress,
    voting_start: String,
    voting_start_epoch: u64,
    voting_end: String,
    voting_end_epoch: u64,
    created: String,
    info_url: String,
    key_image_url: Url,
    additional_data: HashMap<String, String>,
) -> Proposal {
    let current_epoch = Runtime::current_epoch();

    assert!(current_epoch.number() < voting_start_epoch, "Voting start epoch {} is in the past. Current epoch {}", voting_start_epoch, current_epoch.number());
    assert!(voting_start_epoch <= voting_end_epoch, "Voting start epoch {} is after voting end epoch {}", voting_start_epoch, voting_end_epoch);

    let info_unchecked_url = Url::of(info_url);
    let proposal = Proposal {
        proposal_id: id,
        name: title.clone(),
        description: p_abstract.clone(),
        title,
        p_abstract,
        specification,
        dao_type,
        governance_resource,
        voting_start,
        voting_start_epoch: Epoch::of(voting_start_epoch),
        voting_end,
        voting_end_epoch: Epoch::of(voting_end_epoch),
        created,
        created_epoch: current_epoch,
        info_url: info_unchecked_url,
        key_image_url,
        additional_data,
        additional_data_vec: HashMap::new(),
        options: Vec::new(),
        votes: Vec::new(),
        nfts_voted: HashSet::new(),
        result: None,
    };

    proposal
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

    let current_epoch = Runtime::current_epoch().number();
    assert!(current_epoch < proposal.voting_start_epoch.number(), "Options to proposal can only be added epoch before voting start epoch {}. Current epoch {}", proposal.voting_start_epoch.number(), current_epoch);

    for existing_option in proposal.options.iter() {
        assert_ne!(existing_option.id(), option_id, "Option with id {} already exists", option_id);
        assert_ne!(existing_option.option(), option, "Option with name {} already exists", option);
        assert_ne!(existing_option.rank(), rank, "Option with rank {} already exists", rank);
    }

    let option = ProposalOption::new(option_id, rank, option, additional_data);
    proposal.options.push(option);
    proposal_resource_manager.update_non_fungible_data(
        &proposal_fungible_id,
        "options",
        proposal.options,
    );
}

pub(crate) fn mint_nft_vote(
    proposal_resource_manager: ResourceManager,
    proposal_id: String,
    proposal_vote_id: String,
    proposal_option_id: String,
    entity: ComponentAddress,
    voting_nfts: Proof,
    created: String,
    additional_data: HashMap<String, String>,
) {
    let proposal_fungible_id = utils::create_non_fungible_id_of_uuid(&proposal_id);
    let mut proposal: Proposal =
        proposal_resource_manager.get_non_fungible_data(&proposal_fungible_id);

    check_vote_against_proposal(&proposal_vote_id, &proposal_option_id, &proposal);

    let nfts = voting_nfts.check_with_message(proposal.governance_resource.clone(), "NFTs are not from governance resource").as_non_fungible().non_fungible_local_ids();
    assert!(nfts.len() > 0, "No NFTs provided for voting");

    let mut nfts_to_vote: Vec<NonFungibleLocalId> = Vec::new();
    for nft in nfts.iter() {
        if proposal.nfts_voted.get(nft).is_some() {
            info!("NFT {} already voted for proposal", nft.to_string());
            continue;
        }
        proposal.nfts_voted.insert((*nft).clone());
        nfts_to_vote.push((*nft).clone());
    }
    assert!(nfts_to_vote.len() > 0, "All provided NFTs already voted for proposal");

    proposal_resource_manager.update_non_fungible_data(
        &proposal_fungible_id,
        "nfts_voted",
        proposal.nfts_voted,
    );

    let vote = ProposalVote::new(proposal_vote_id, proposal_option_id, entity, Decimal::from(nfts_to_vote.len()), nfts_to_vote, created, additional_data);
    proposal.votes.push(vote);
    proposal_resource_manager.update_non_fungible_data(
        &proposal_fungible_id,
        "votes",
        proposal.votes,
    );

    update_result(&proposal_resource_manager, &proposal_fungible_id);
}

fn check_vote_against_proposal(proposal_vote_id: &String, proposal_option_id: &String, proposal: &Proposal) {
    if proposal.options.iter().all(|option| option.id() != *proposal_option_id) {
        panic!("Proposal Option with id {} does not exist", *proposal_option_id);
    }

    if proposal.votes.iter().any(|vote| vote.id() == *proposal_vote_id) {
        panic!("Vote with id {} has already voted", *proposal_vote_id);
    }

    let current_epoch = Runtime::current_epoch().number();
    assert!(current_epoch >= proposal.voting_start_epoch.number(), "Voting has not started yet. Current epoch: {}, Voting start epoch: {}", current_epoch, proposal.voting_start_epoch.number());
    assert!(current_epoch <= proposal.voting_end_epoch.number(), "Voting has ended. Current epoch: {}, Voting end epoch: {}", current_epoch, proposal.voting_end_epoch.number());
}

fn update_result(
    proposal_resource_manager: &ResourceManager,
    proposal_id: &NonFungibleLocalId,
) {
    let mut proposal: Proposal =
        proposal_resource_manager.get_non_fungible_data(&proposal_id);
    let proposal_result = calc_result(&proposal.votes, &proposal.options);

    proposal.result = Some(proposal_result);
    proposal_resource_manager.update_non_fungible_data(
        &proposal_id,
        "result",
        proposal.result,
    );
}
