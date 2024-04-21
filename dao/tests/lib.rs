use crumbsup_dao::test_bindings::DaoHoard;
use radix_engine_interface::prelude::*;
use scrypto::this_package;
use scrypto_test::prelude::*;
use scrypto_unit::*;

#[test]
fn test_create_dao_hoard() {
    let mut test_runner = TestRunnerBuilder::new().build();
    let package_address = test_runner.compile_and_publish(this_package!());
    let token_address = ResourceAddress::try_from_hex(
        "5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6",
    )
    .unwrap();

    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "DaoHoard",
            "dao_hoard_instantiate",
            manifest_args!(
                token_address,
                dec!("234")
            ),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(manifest, vec![]);
    println!("{:?}\n", receipt);
    receipt.expect_commit(true);
}

#[test]
fn test_dao_instantiate() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;
    let token_address = ResourceAddress::try_from_hex(
        "5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6",
    )
    .unwrap();
    let mut dao_hoard =
        DaoHoard::dao_hoard_instantiate(token_address, dec!("15"), package_address, &mut env)?;

    // Act
    let rules: Vec<String> = vec![
        "rule 1".to_string(),
        "rule 2".to_string(),
        "rule 3".to_string(),
    ];
    let additional_data: HashMap<String, String> = HashMap::from_iter([
        ("add_key_1".to_string(), "add_value_1".to_string()),
        ("add_key_2".to_string(), "add_value_2".to_string()),
    ]);

    let _ = dao_hoard.dao_create(
        "9ca67daa-2f84-4db2-aec3-8deaa2bdd093".to_string(),
        "dao name".to_string(),
        "https://info.url".to_string(),
        "https://logo.url".to_string(),
        "token".to_string(),
        "resource_address".to_string(),
        "dao about".to_string(),
        "dao general".to_string(),
        "2024-01-26T22:16:32.256163Z".to_string(),
        rules,
        additional_data,
        &mut env,
    )?;

    // Assert

    Ok(())
}
