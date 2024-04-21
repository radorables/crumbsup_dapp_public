use scrypto::prelude::*;

pub(crate) fn create_dao_admin_badge_resource_manager(
    owner_badge_address: ResourceAddress,
    owner_badge_access_rule: &AccessRule,
    component_address: ComponentAddress
) -> ResourceManager {
    ResourceBuilder::new_ruid_non_fungible::<DaoAdminBadge>(OwnerRole::Fixed(owner_badge_access_rule.clone()))
        .metadata(metadata!(
                    roles {
                        metadata_setter => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                        metadata_setter_updater => OWNER;
                        metadata_locker => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(owner_badge_address)]));
                        metadata_locker_updater => OWNER;
                    },
                    init {
                        "name" => "CrumbsUp DAO Admin Badge", updatable;
                        "description" => "These are the badges for administrating of CrumbsUp DAO", updatable;
                        "icon_url" => Url::of("https://arweave.net/RkZVcWWW0KzhzNggBXsz54T3tMDlerRwMDaobNSIPgk"), updatable;
                        "tags" => vec!["CrumbsUp", "DAO Admin"], updatable;
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
        .create_with_no_initial_supply()
}

#[derive(ScryptoSbor, NonFungibleData)]
pub(crate) struct DaoAdminBadge {
    dao_id: String,
    #[mutable]
    name: String,
    #[mutable]
    description: String,
    #[mutable]
    info_url: Url,
    #[mutable]
    key_image_url: Url,
    #[mutable]
    additional_data: HashMap<String, String>,
    #[mutable]
    additional_data_vec: HashMap<String, Vec<String>>,
}

impl DaoAdminBadge {
    pub(crate) fn dao_id(&self) -> String {
        self.dao_id.clone()
    }
}

pub(crate) fn mint(dao_admin_badges_manager: ResourceManager, dao_id: String, dao_name: String) -> Bucket {
    let dao_admin_badge = DaoAdminBadge {
        dao_id,
        name: format!("{} Admin Badge", dao_name),
        description: format!("This Admin Badge allows you to administrate the DAO {}.", dao_name),
        info_url: Url::of("https://crumbsup.io"),
        key_image_url: Url::of("https://arweave.net/RkZVcWWW0KzhzNggBXsz54T3tMDlerRwMDaobNSIPgk"),
        additional_data: HashMap::new(),
        additional_data_vec: HashMap::new(),
    };

    let dao_admin_badge_bucket = dao_admin_badges_manager.mint_ruid_non_fungible(dao_admin_badge);
    dao_admin_badge_bucket
}

pub(crate) fn check_is_dao_admin(dao_admin_badges_manager: ResourceManager, admin_badges: Proof, dao_id: &String) {
    let admin_badges_checked = admin_badges.check_with_message(dao_admin_badges_manager.address().clone(), "NFTs are no DAO Admin Badges").as_non_fungible();//.non_fungible_local_ids();
    let dao_admin_badge_found = admin_badges_checked.non_fungibles().iter().any(|nft: &NonFungible<DaoAdminBadge>| {
        *dao_id == nft.data().dao_id()
    });
    assert!(dao_admin_badge_found, "You are not an admin of this DAO");
}
