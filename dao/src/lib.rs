mod proposal;
mod proposal_option;
mod proposal_result;
mod proposal_vote;
mod utils;

use scrypto::prelude::*;

const VERSION: u32 = 1;

#[derive(ScryptoSbor, NonFungibleData)]
struct Dao {
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
    resource_address: String,
    #[mutable]
    about: String,
    #[mutable]
    general: String,
    created: String,
    #[mutable]
    rules: Vec<String>,
    #[mutable]
    additional_data: HashMap<String, String>,
    proposals: ResourceManager,
}

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
            superadmin => updatable_by: [OWNER];
            daocreator => updatable_by: [OWNER];
        },
        methods {
            dao_hoard_set_proposal_creation_price => restrict_to: [superadmin];
            dao_hoard_withdraw_crumb_fees => restrict_to: [superadmin];
            dao_create => restrict_to: [daocreator];
            dao_update => PUBLIC;
            dao_add_proposal => PUBLIC;
            proposal_add_option => PUBLIC;
            proposal_add_vote => PUBLIC;
            proposal_add_result => PUBLIC;
        }
    }
    struct DaoHoard {
        dao_resource_manager: ResourceManager,
        crumb_fees: Vault,
        proposal_creation_price: Decimal,
    }

    impl DaoHoard {
        pub fn dao_hoard_instantiate(crumbs_token_address: ResourceAddress, proposal_creation_price: Decimal) -> Global<DaoHoard> {
            let dao_resource_manager = Self::dao_resource_manager_create();
            let owner_badge_access_rule: AccessRule = rule!(require(OWNER_BADGE.address()));

            let dao_hoard = Self {
                dao_resource_manager,
                crumb_fees: Vault::new(crumbs_token_address),
                proposal_creation_price,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(owner_badge_access_rule))
            .roles(roles!(
                superadmin => OWNER;
                daocreator => OWNER;
            ))
            .metadata(metadata!(
                init {
                    "name" => "Dao Hoard Component", locked;
                    "info_url" => Url::of("https://www.crumbsup.net"), updatable;
                    "icon_url" => Url::of("https://www.crumbsup.net/crumbs_logo.png"), updatable;
                    "claimed_websites" => [Origin::of("https://www.crumbsup.net"), Origin::of("https://stokenet.crumbsup.de")], updatable;
                }
            ))
            .globalize();

            dao_hoard
        }

        fn dao_resource_manager_create() -> ResourceManager {
            let dao_resource_manager =
                ResourceBuilder::new_string_non_fungible::<Dao>(OwnerRole::None)
                    .metadata(metadata!(
                     init {
                        "name" => "CrumbsUp Daos", updatable;
                        "description" => "These are the Daos on CrumbsUp platform", updatable;
                        "info_url" => Url::of("https://www.crumbsup.net"), updatable;
                        "icon_url" => Url::of("https://www.crumbsup.net/crumbs_logo.png"), updatable;
                        "tags" => vec!["CrumbsUp", "DAO"], updatable;
                        "component_type" => "CrumbsUpDaoHoard".to_string(), locked;
                        "version" => VERSION, locked;
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

            dao_resource_manager
        }

        pub fn dao_hoard_set_proposal_creation_price(&mut self, proposal_creation_price: Decimal) {
            self.proposal_creation_price = proposal_creation_price;
        }

        pub fn dao_hoard_withdraw_crumb_fees(&mut self) -> Bucket {
            self.crumb_fees.take_all()
        }

        pub fn dao_create(
            &mut self,
            dao_id: String,
            name: String,
            info_url: String,
            key_image_url: String,
            dao_type: String,
            resource_address: String,
            about: String,
            general: String,
            created: String,
            rules: Vec<String>,
            additional_data: HashMap<String, String>,
        ) -> Bucket {

            let dao_non_fungible_id = utils::create_non_fungible_id_of_uuid(&dao_id);

            let info_unchecked_url = Url::of(info_url);
            let key_image_unchecked_url = Url::of(key_image_url);
            let proposal_resource_manager =
                proposal::create_proposal_resource_manager(&name, key_image_unchecked_url.clone());

                
            let dao = Dao {
                dao_id,
                component_type: "CrumbsUpDao".to_string(),
                version: VERSION,
                name,
                description: about.clone(),
                info_url: info_unchecked_url,
                key_image_url: key_image_unchecked_url,
                dao_type,
                resource_address,
                about,
                general,
                created,
                rules,
                additional_data,
                proposals: proposal_resource_manager,
            };

            let dao_bucket = self
                .dao_resource_manager
                .mint_non_fungible(&dao_non_fungible_id, dao);

            dao_bucket
        }

        pub fn dao_update(
            &self,
            dao_id: String,
            name: String,
            info_url: String,
            key_image_url: String,
            dao_type: String,
            resource_address: String,
            about: String,
            general: String,
            rules: Vec<String>,
            additional_data: HashMap<String, String>,
        ) {
            let dao_non_fungible_id = utils::create_non_fungible_id_of_uuid(&dao_id);

            let info_unchecked_url = Url::of(info_url);
            let key_image_unchecked_url = Url::of(key_image_url);
            self.dao_resource_manager
                .update_non_fungible_data(&dao_non_fungible_id, "name", name);
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "info_url",
                info_unchecked_url,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "key_image_url",
                key_image_unchecked_url,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "dao_type",
                dao_type,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "resource_address",
                resource_address,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "about",
                about,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "general",
                general,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "rules",
                rules,
            );
            self.dao_resource_manager.update_non_fungible_data(
                &dao_non_fungible_id,
                "additional_data",
                additional_data,
            );
        }

        pub fn dao_add_proposal(
            &mut self,
            mut payment: Bucket,
            dao_id: String,
            proposal_id: String,
            title: String,
            proposal_abstract: String,
            specification: String,
            voting_start: String,
            voting_end: String,
            created: String,
            info_url: String,
            additional_data: HashMap<String, String>,
        ) -> (Bucket, Bucket) {

            let crumb_fee = payment.take(self.proposal_creation_price);
            self.crumb_fees.put(crumb_fee);

            let proposal_non_fungible_id = utils::create_non_fungible_id_of_uuid(&proposal_id);

            let dao = self.get_dao(&dao_id);
            let proposal = proposal::create_proposal(
                proposal_id,
                title,
                proposal_abstract,
                specification,
                voting_start,
                voting_end,
                created,
                info_url,
                dao.key_image_url,
                additional_data,
            );

            let proposal_bucket = dao
                .proposals
                .mint_non_fungible(&proposal_non_fungible_id, proposal);

            (proposal_bucket, payment)
        }

        pub fn proposal_add_option(
            &self,
            dao_id: String,
            proposal_id: String,
            proposal_option_id: String,
            rank: u32,
            option: String,
            additional_data: HashMap<String, String>,
        ) {
            let dao = self.get_dao(&dao_id);
            proposal::add_option(dao.proposals, proposal_id, proposal_option_id, rank, option, additional_data);
        }

        pub fn proposal_add_vote(
            &self,
            dao_id: String,
            proposal_id: String,
            proposal_vote_id: String,
            proposal_option_id: String,
            entity: ComponentAddress,
            power: Decimal,
            created: String,
            additional_data: HashMap<String, String>,
        ) {
            let dao = self.get_dao(&dao_id);
            proposal::add_vote(
                dao.proposals,
                proposal_id,
                proposal_vote_id,
                proposal_option_id,
                entity,
                power,
                created,
                additional_data,
            );
        }

        pub fn proposal_add_result(
            &self,
            dao_id: String,
            proposal_id: String,
            vote_count: u32,
            vote_power: Decimal,
            results: HashMap<String, Decimal>,
            additional_data: HashMap<String, String>,
        ) {
            let dao = self.get_dao(&dao_id);
            proposal::add_result(
                dao.proposals,
                proposal_id,
                vote_count,
                vote_power,
                results,
                additional_data,
            );
        }

        fn get_dao(&self, dao_id: &str) -> Dao {
            let dao_non_fungible_id = utils::create_non_fungible_id_of_uuid(dao_id);
            let dao: Dao = self
                .dao_resource_manager
                .get_non_fungible_data(&dao_non_fungible_id);
            dao
        }
    }
}
