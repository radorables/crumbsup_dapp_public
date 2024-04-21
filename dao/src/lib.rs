use scrypto::prelude::*;

mod dao;
mod dao_admin_badge;
mod proposal;
mod proposal_option;
mod proposal_result;
mod proposal_result_option;
mod proposal_vote;
mod utils;

#[blueprint]
mod dao_hoard {
    // the Owner Badge has to exist before compiling
    // RESIM Martin const OWNER_BADGE: ResourceManager = resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");
    // RESIM Stephan const OWNER_BADGE: ResourceManager = resource_manager!("resource_sim1nfkwg8fa7ldhwh8exe5w4acjhp9v982svmxp3yqa8ncruad4t8fptu");
    // STOKENET
    const OWNER_BADGE: ResourceManager = resource_manager!("resource_tdx_2_1n2r7x3k0e6ed4a3cztq92gv69s4cd4fwh77me5gdw0p6h4mlh5cf82");

    enable_function_auth! {
        // only the OWNER is able to instantiate our dao horde
        dao_hoard_instantiate => rule!(require(OWNER_BADGE.address()));
    }
    enable_method_auth! {
        roles {
            super_admin => updatable_by: [OWNER];
            dao_creator => updatable_by: [OWNER];
            dao_admin => updatable_by: [super_admin, OWNER];
        },
        methods {
            dao_hoard_set_proposal_creation_price => restrict_to: [super_admin, OWNER];
            dao_hoard_withdraw_crumb_fees => restrict_to: [super_admin, OWNER];
            dao_mint_admin_badge_by_owner => restrict_to: [super_admin, OWNER, SELF];
            dao_mint_admin_badge_by_dao_admin => restrict_to: [dao_admin];
            dao_create => restrict_to: [dao_creator, super_admin, OWNER];
            dao_update => restrict_to: [dao_admin];
            dao_add_proposal => restrict_to: [dao_admin];
            proposal_add_option => restrict_to: [dao_admin];
            proposal_mint_nft_vote => PUBLIC;
        }
    }
    struct DaoHoard {
        component_address: ComponentAddress,
        owner_badge_access_rule: AccessRule,
        dao_resource_manager: ResourceManager,
        dao_admin_badges_manager: ResourceManager,
        crumb_fees: Vault,
        proposal_creation_price: Decimal,
    }

    impl DaoHoard {
        pub fn dao_hoard_instantiate(crumbs_token_address: ResourceAddress, proposal_creation_price: Decimal) -> Global<DaoHoard> {
            let (address_reservation, component_address) = Runtime::allocate_component_address(DaoHoard::blueprint_id());
            let owner_badge_access_rule: AccessRule = rule!(require(OWNER_BADGE.address()));
            let public_rule: AccessRule = rule!(allow_all);

            let dao_resource_manager = dao::create_resource_manager(OWNER_BADGE.address(), &owner_badge_access_rule, component_address);
            let dao_admin_badges_manager = dao_admin_badge::create_dao_admin_badge_resource_manager(OWNER_BADGE.address(), &owner_badge_access_rule, component_address);

            let cloned_owner_badge_access_rule = owner_badge_access_rule.clone();
            let dao_hoard = Self {
                component_address,
                owner_badge_access_rule: cloned_owner_badge_access_rule,
                dao_resource_manager,
                dao_admin_badges_manager,
                crumb_fees: Vault::new(crumbs_token_address),
                proposal_creation_price,
            };

            let dao_hoard_global = dao_hoard.instantiate()
                .prepare_to_globalize(OwnerRole::Fixed(owner_badge_access_rule))
                .with_address(address_reservation)
                .roles(roles!(
                    super_admin => OWNER;
                    dao_creator => public_rule;
                    dao_admin => rule!(require(dao_admin_badges_manager.address()));
                ))
                .metadata(metadata!(
                    roles {
                        metadata_setter => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(OWNER_BADGE.address())]));
                        metadata_setter_updater => OWNER;
                        metadata_locker => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(OWNER_BADGE.address())]));
                        metadata_locker_updater => OWNER;
                    },
                    init {
                        "name" => "CrumbsUp", updatable;
                        "description" => "Dao Component of CrumbsUp platform", updatable;
                        "info_url" => Url::of("https://crumbsup.io"), updatable;
                        "icon_url" => Url::of("https://arweave.net/-xdfyErdaRWX_WaD9xiFUgMJ73TbDqwdy-Q9Megr4-s"), updatable;
                        "claimed_websites" => [Origin::of("https://www.crumbsup.io"), Origin::of("https://www.stokenet.crumbsup.de")], updatable;
                        "tags" => vec!["CrumbsUp", "DAO"], updatable;
                    }
                ))
                .enable_component_royalties(component_royalties! {
                    roles {
                        royalty_setter => OWNER;
                        royalty_setter_updater => OWNER;
                        royalty_locker => OWNER;
                        royalty_locker_updater => OWNER;
                        royalty_claimer => rule!(require_any_of(vec![global_caller(component_address), ResourceOrNonFungible::Resource(OWNER_BADGE.address())]));
                        royalty_claimer_updater => OWNER;
                    },
                    init {
                        dao_hoard_set_proposal_creation_price => Free, updatable;
                        dao_hoard_withdraw_crumb_fees => Free, updatable;
                        dao_mint_admin_badge_by_owner => Free, updatable;
                        dao_mint_admin_badge_by_dao_admin => Usd(dec!("0.10")), updatable;
                        dao_create => Free, updatable;
                        dao_update => Free, updatable;
                        dao_add_proposal => Free, updatable;
                        proposal_add_option => Free, updatable;
                        proposal_mint_nft_vote => Usd(dec!("0.05")), updatable;
                    }
                })
                .globalize();

            dao_hoard_global
        }

        pub fn dao_hoard_set_proposal_creation_price(&mut self, proposal_creation_price: Decimal) {
            self.proposal_creation_price = proposal_creation_price;
        }

        pub fn dao_hoard_withdraw_crumb_fees(&mut self) -> Bucket {
            self.crumb_fees.take_all()
        }

        pub fn dao_mint_admin_badge_by_owner(&mut self, dao_id: String) -> Bucket {
            let dao = dao::get(self.dao_resource_manager, &dao_id);
            let dao_admin_badge_bucket = dao_admin_badge::mint(self.dao_admin_badges_manager, dao_id, dao.name());
            dao_admin_badge_bucket
        }

        pub fn dao_mint_admin_badge_by_dao_admin(&mut self, admin_badge: Proof) -> Bucket {
            let admin_badge_nft: NonFungible<dao_admin_badge::DaoAdminBadge> =
                admin_badge.check(self.dao_admin_badges_manager.address()).as_non_fungible().non_fungible();

            let dao = dao::get(self.dao_resource_manager, &admin_badge_nft.data().dao_id());
            let dao_admin_badge_bucket = dao_admin_badge::mint(self.dao_admin_badges_manager, admin_badge_nft.data().dao_id(), dao.name());
            dao_admin_badge_bucket
        }

        pub fn dao_create(
            &mut self,
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
        ) -> (Bucket, Bucket) {

            let dao_bucket = dao::create(
                OWNER_BADGE.address(),
                &self.owner_badge_access_rule,
                self.component_address,
                self.dao_resource_manager,
                dao_id.clone(),
                name,
                info_url,
                key_image_url,
                dao_type,
                governance_resource,
                about,
                general,
                created,
                rules,
                additional_data,
            );

            let admin_badge_bucket = self.dao_mint_admin_badge_by_owner(dao_id);
            (dao_bucket, admin_badge_bucket)
        }

        pub fn dao_update(
            &self,
            admin_badges: Proof,
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
            dao_admin_badge::check_is_dao_admin(self.dao_admin_badges_manager, admin_badges, &dao_id);

            dao::update(
                self.dao_resource_manager,
                dao_id,
                name,
                info_url,
                key_image_url,
                dao_type,
                governance_resource,
                about,
                general,
                rules,
                additional_data,
            );
        }

        pub fn dao_add_proposal(
            &mut self,
            admin_badges: Proof,
            mut payment: Bucket,
            dao_id: String,
            proposal_id: String,
            title: String,
            proposal_abstract: String,
            specification: String,
            voting_start: String,
            voting_start_epoch: u64,
            voting_end: String,
            voting_end_epoch: u64,
            created: String,
            info_url: String,
            additional_data: HashMap<String, String>,
        ) -> (Bucket, Bucket) {
            dao_admin_badge::check_is_dao_admin(self.dao_admin_badges_manager, admin_badges, &dao_id);

            let crumb_fee = payment.take(self.proposal_creation_price);
            self.crumb_fees.put(crumb_fee);

            let proposal_non_fungible_id = utils::create_non_fungible_id_of_uuid(&proposal_id);

            let dao = dao::get(self.dao_resource_manager, &dao_id);
            let proposal = proposal::create(
                proposal_id,
                title,
                proposal_abstract,
                specification,
                dao.dao_type(),
                dao.governance_resource(),
                voting_start,
                voting_start_epoch,
                voting_end,
                voting_end_epoch,
                created,
                info_url,
                dao.key_image_url(),
                additional_data,
            );

            let proposal_bucket = dao
                .proposals()
                .mint_non_fungible(&proposal_non_fungible_id, proposal);

            (proposal_bucket, payment)
        }

        pub fn proposal_add_option(
            &self,
            admin_badges: Proof,
            dao_id: String,
            proposal_id: String,
            proposal_option_id: String,
            rank: u32,
            option: String,
            additional_data: HashMap<String, String>,
        ) {
            dao_admin_badge::check_is_dao_admin(self.dao_admin_badges_manager, admin_badges, &dao_id);

            let dao = dao::get(self.dao_resource_manager, &dao_id);
            proposal::add_option(dao.proposals(), proposal_id, proposal_option_id, rank, option, additional_data);
        }

        pub fn proposal_mint_nft_vote(
            &self,
            dao_id: String,
            proposal_id: String,
            proposal_vote_id: String,
            proposal_option_id: String,
            entity: ComponentAddress,
            voting_nfts: Proof,
            created: String,
            additional_data: HashMap<String, String>,
        ) {
            let dao = dao::get(self.dao_resource_manager, &dao_id);
            proposal::mint_nft_vote(
                dao.proposals(),
                proposal_id,
                proposal_vote_id,
                proposal_option_id,
                entity,
                voting_nfts,
                created,
                additional_data,
            );
        }
    }
}
