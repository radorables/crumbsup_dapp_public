use scrypto::prelude::*;

pub(crate) fn create_non_fungible_id_of_uuid(uuid: &str) -> NonFungibleLocalId {
    let nft_id = uuid.replace("-", "_");
    let non_fungible_id = NonFungibleLocalId::string(nft_id);
    non_fungible_id.unwrap()
}