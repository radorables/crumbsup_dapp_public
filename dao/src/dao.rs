use scrypto::prelude::*;

use crate::{proposal, utils};

#[derive(ScryptoSbor, NonFungibleData)]
pub(crate) struct Dao {
    dao_id: String,
    component_type: String,
    version: u32,
    #[mutable]
    name: String,
    #[mutable]
    description: String,
    #[mutable]
    info_url: Url,
    #[mutable]
    key_image_url: Url,
    #[mutable]
    dao_type: String,
    #[mutable]
    governance_resource: ResourceAddress,
    #[mutable]
    about: String,
    #[mutable]
    general: String,
    created: String,
    created_epoch: Epoch,
    #[mutable]
    rules: Vec<String>,
    #[mutable]
    additional_data: HashMap<String, String>,
    #[mutable]
    additional_data_vec: HashMap<String, Vec<String>>,
    proposals: ResourceManager,
}

impl Dao {
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    pub(crate) fn proposals(&self) -> ResourceManager {
        self.proposals
    }

    pub(crate) fn dao_type(&self) -> String {
        self.dao_type.clone()
    }

    pub(crate) fn governance_resource(&self) -> ResourceAddress {
        self.governance_resource.clone()
    }

    pub(crate) fn key_image_url(&self) -> Url {
        self.key_image_url.clone()
    }
}

const VERSION: u32 = 1;


pub(crate) fn create_resource_manager(
    owner_badge_address: ResourceAddress,
    owner_badge_access_rule: &AccessRule,
    component_address: ComponentAddress,
) -> ResourceManager {
    let dao_resource_manager =
        ResourceBuilder::new_string_non_fungible::<Dao>(OwnerRole::Fixed(owner_badge_access_rule.clone()))
            .metadata(metadata!(
                        roles {
                            metadata_setter => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                            metadata_setter_updater => OWNER;
                            metadata_locker => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                            metadata_locker_updater => OWNER;
                        },
                        init {
                            "name" => "CrumbsUp Daos", updatable;
                            "description" => "These are the Daos on CrumbsUp platform", updatable;
                            "info_url" => Url::of("https://crumbsup.io"), updatable;
                            "icon_url" => Url::of("https://arweave.net/-xdfyErdaRWX_WaD9xiFUgMJ73TbDqwdy-Q9Megr4-s"), updatable;
                            "tags" => vec!["CrumbsUp", "DAO"], updatable;
                            "component_type" => "CrumbsUpDaoHoard".to_string(), locked;
                            "version" => VERSION, locked;
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

    dao_resource_manager
}

pub(crate) fn create(
    owner_badge_address: ResourceAddress,
    owner_badge_access_rule: &AccessRule,
    component_address: ComponentAddress,
    dao_resource_manager: ResourceManager,
    dao_id: String,
    name: String,
    info_url: String,
    key_image_url: String,
    dao_type: String,
    governance_resource: ResourceAddress,
    about: String,
    general: String,
    created: String,
    rules: Vec<String>,
    additional_data: HashMap<String, String>,
) -> Bucket {
    let info_unchecked_url = Url::of(info_url);
    let key_image_unchecked_url = Url::of(key_image_url);
    let proposal_resource_manager =
        proposal::create_resource_manager(
            owner_badge_address,
            owner_badge_access_rule,
            component_address,
            &name,
            key_image_unchecked_url.clone(),
        );

    let dao = Dao {
        dao_id: dao_id.clone(),
        component_type: "CrumbsUpDao".to_string(),
        version: VERSION,
        name,
        description: about.clone(),
        info_url: info_unchecked_url,
        key_image_url: key_image_unchecked_url,
        dao_type,
        governance_resource,
        about,
        general,
        created,
        created_epoch: Runtime::current_epoch(),
        rules,
        additional_data,
        additional_data_vec: HashMap::new(),
        proposals: proposal_resource_manager,
    };

    let dao_non_fungible_id = utils::create_non_fungible_id_of_uuid(&dao_id);
    let dao_bucket = dao_resource_manager.mint_non_fungible(&dao_non_fungible_id, dao);
    dao_bucket
}

pub(crate) fn update(
    dao_resource_manager: ResourceManager,
    dao_id: String,
    name: String,
    info_url: String,
    key_image_url: String,
    dao_type: String,
    governance_resource: ResourceAddress,
    about: String,
    general: String,
    rules: Vec<String>,
    additional_data: HashMap<String, String>,
) {
    let dao_non_fungible_id = utils::create_non_fungible_id_of_uuid(&dao_id);

    let info_unchecked_url = Url::of(info_url);
    let key_image_unchecked_url = Url::of(key_image_url);
    dao_resource_manager
        .update_non_fungible_data(&dao_non_fungible_id, "name", name);
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "info_url",
        info_unchecked_url,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "key_image_url",
        key_image_unchecked_url,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "dao_type",
        dao_type,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "governance_resource",
        governance_resource,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "about",
        about,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "general",
        general,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "rules",
        rules,
    );
    dao_resource_manager.update_non_fungible_data(
        &dao_non_fungible_id,
        "additional_data",
        additional_data,
    );
}

pub(crate) fn get(dao_resource_manager: ResourceManager, dao_id: &str) -> Dao {
    let dao_non_fungible_id = utils::create_non_fungible_id_of_uuid(dao_id);
    let dao: Dao = dao_resource_manager.get_non_fungible_data(&dao_non_fungible_id);
    dao
}
