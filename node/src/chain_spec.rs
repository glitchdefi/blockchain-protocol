use glitch_node_runtime::{
    AccountId, AuthorityDiscoveryConfig, BabeConfig, Balance, BalancesConfig, ContractsConfig,
    EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig, ImOnlineId, IndicesConfig,
    SessionConfig, SessionKeys, Signature, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    DOLLARS, WASM_BINARY, RevenueConfig
};
use pallet_evm::GenesisAccount;
use primitive_types::H160;
use sc_service::ChainType;
use serde_json as json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use std::collections::BTreeMap;
use std::str::FromStr;
use sc_telemetry::TelemetryEndpoints;
use hex_literal::hex;
use sp_core::crypto::UncheckedInto;

// The URL for the telemetry server.
// const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

const DEFAULT_PROPERTIES_TESTNET: &str = r#"
{
"tokenSymbol": "GLCH",
"tokenDecimals": 18,
"ss58Format": 42
}
"#;

const DEFAULT_PROPERTIES_MAINNET: &str = r#"
{
    "tokenSymbol": "GLCH",
    "tokenDecimals": 18,
    "ss58Format": 42
}
"#;

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(
    s: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<BabeId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<ImOnlineId>(s),
        get_from_seed::<AuthorityDiscoveryId>(s),
    )
}

fn endowed_evm_account() -> BTreeMap<H160, GenesisAccount> {
    let endowed_account = vec![
        // glitch_node fauct
        H160::from_str("8097c3C354652CB1EEed3E5B65fBa2576470678A").unwrap(),
    ];
    get_endowed_evm_accounts(endowed_account)
}

fn dev_endowed_evm_accounts() -> BTreeMap<H160, GenesisAccount> {
    let endowed_account = vec![
        H160::from_str("8097c3C354652CB1EEed3E5B65fBa2576470678A").unwrap(),
        H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b").unwrap(),
        H160::from_str("e6206C7f064c7d77C6d8e3eD8601c9AA435419cE").unwrap(),
        // the dev account key
        // seed: bottom drive obey lake curtain smoke basket hold race lonely fit walk
        // private key: 0x03183f27e9d78698a05c24eb6732630eb17725fcf2b53ee3a6a635d6ff139680
        H160::from_str("aed40f2261ba43b4dffe484265ce82d8ffe2b4db").unwrap(),
    ];

    get_endowed_evm_accounts(endowed_account)
}

fn get_endowed_evm_accounts(endowed_account: Vec<H160>) -> BTreeMap<H160, GenesisAccount> {
    let mut evm_accounts = BTreeMap::new();
    for account in endowed_account {
        evm_accounts.insert(
            account,
            GenesisAccount {
                nonce: U256::from(0),
                balance: U256::from(1_000_000_000 * DOLLARS),
                storage: Default::default(),
                code: vec![],
            },
        );
    }
    evm_accounts
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            glitch_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    //get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
                dev_endowed_evm_accounts(),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("glitch_nodelocal"),
        // Properties
        Some(json::from_str(DEFAULT_PROPERTIES_TESTNET).unwrap()),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "glitch_node",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            glitch_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    //get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    //get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
                endowed_evm_account(),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("glitch_nodelocal"),
        // Properties
        Some(json::from_str(DEFAULT_PROPERTIES_TESTNET).unwrap()),
        // Extensions
        None,
    ))
}

//Glitch testnet
pub fn glitch_testnet_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;
  Ok(ChainSpec::from_genesis(
      //Name
      "Glitch",
      //ID
      "glitch_testnet",
      ChainType::Custom(String::from("glitch_testnet")),
      move || glitch_genesis(
          wasm_binary,
          // Initial PoA authories
          vec![
        // SECRET="release shoulder canyon agree mule snack genre scare furnace coffee fragile sun"
        // 5CmEYEyMpoyKHWde8gZcPUo39dzzhD6y5jp9TxC23opLUzq4
        // subkey inspect "$SECRET//glitch//1//validator"
        // subkey inspect "$SECRET//glitch//1//babe"
        // subkey inspect --scheme ed25519 "$SECRET//glitch//1//grandpa"
        // subkey inspect "$SECRET//glitch//1//imonline"
        // subkey inspect "$SECRET//glitch//1//discovery"
        (
          hex!["aee0df231aed47a99beb26960d156de55751a48741b1cfcfa99e42a5f499e63f"].into(),
          hex!["aee0df231aed47a99beb26960d156de55751a48741b1cfcfa99e42a5f499e63f"].into(),
          hex!["2eed8720f0cc2697ef251da006980c0873cfc108f4c91a96744eedf1b59aff44"].unchecked_into(), // babe key
          hex!["04315411f66a58e838690017ac4102374e4f1dd3f429176ebaa15d55d61d9e80"].unchecked_into(), // grandpa
          hex!["a86ab6e2458078bc49f7277ba00642c1ba2336f8001dd7de6494fd0751a1c057"].unchecked_into(), // imonline
          hex!["1eaa69cbee5e16337c52440455fbc97b839d01bdc4e6d158cf3061d44ba4fc7b"].unchecked_into(), // discovery
        ),
        // SECRET="pumpkin carpet nest snack yard eternal nation fame scare obvious toe promote"
        // 5G6nMq5x8xm3PLxyKXkKEkzAjzQLGbuiDarjBWp6d5XHg3FM
        // subkey inspect "$SECRET//glitch//2//validator"
        // subkey inspect "$SECRET//glitch//2//babe"
        // subkey inspect --scheme ed25519 "$SECRET//glitch//2//grandpa"
        // subkey inspect "$SECRET//glitch//2//imonline"
        // subkey inspect "$SECRET//glitch//2//discovery"
        (
          hex!["5a44973835f643d6d2c45c4490e1c0095755d469ae5db20d480c937588abd164"].into(),
          hex!["5a44973835f643d6d2c45c4490e1c0095755d469ae5db20d480c937588abd164"].into(),
          hex!["381bab65c6c3c0a9bfb06e7a6c8a3b109bed9e97303b22fcd0157ba0871a6566"].unchecked_into(), // babe
          hex!["72f22b083c995d0c4bf07a46f7ad326bcc25780483c8eb0523f53ed2a5b7915d"].unchecked_into(), // grandpa
          hex!["46bd8c5f164df0a6db9199a74376aea3cf8f0d4ecc8b4da5289ebf70c6d0496a"].unchecked_into(), // imonline
          hex!["98a818de9aa0ea6d376a8690bfebb24b1d5d7d9c6f36e1ab45983b0fa7f2e328"].unchecked_into(), // discovery
        ),
        // SECRET="light file noodle black twenty mixed gesture trip desk business reason rural"
        // 5EcCtaankoR4eRuBFFbRKaydUWG7JFp7YPp9oVD7mSQpbhBV
        // subkey inspect "$SECRET//glitch//3//validator"
        // subkey inspect "$SECRET//glitch//3//babe"
        // subkey inspect --scheme ed25519 "$SECRET//glitch//3//grandpa"
        // subkey inspect "$SECRET//glitch//3//imonline"
        // subkey inspect "$SECRET//glitch//3//discovery"
        (
          hex!["94f1d376734418cc137900c7ad4bb5a4911af24d00e4fd72803e3817a50c334d"].into(),
          hex!["94f1d376734418cc137900c7ad4bb5a4911af24d00e4fd72803e3817a50c334d"].into(),
          hex!["6eb9b6f2680b69714d38d0d34108288c9498616589afa6ca453f7a71a88cc64c"].unchecked_into(), // babe
          hex!["0b9f59981f7b9a654f9819c7cb774f3fe39fc9e4c5ac22202f77be52677a5fd3"].unchecked_into(), // grandpa
          hex!["1efe62dcf7953227eb99413539b2d58c584f8568ab72d847310a17030882ef0d"].unchecked_into(), // imonline
          hex!["82aba2132ab4ff2e7fcafeb4d4932f73413d08b7f3a54e18ea2c8d4cffa88e2b"].unchecked_into(), // discovery
        ),
      ],
          // SECRET="assume blanket feature indicate enhance lounge nation spray venture connect kidney dish"
          // 5CFPN3erbXSA7KyE8kM66QZ7izgMba7aHQApvZeT4VeNZYbv
          // subkey inspect "$SECRET//glitch//root"
          hex!["9c6418d848434a5eefcd9c7640e9518850ac4f268b985d73e94d94bede19eb4c"].into(),
          vec![
        // 5CPQQYs3wf32fr5PhmmfFQEeVzD1Zy9Hdo8LFzQYuhP8XHW6
        hex!["9c6418d848434a5eefcd9c7640e9518850ac4f268b985d73e94d94bede19eb4c"].into(),
      ],
          true,
          endowed_evm_account()
      ),
      // Bootnodes
      vec![],
      //Telemetry
      None,
      // Protocol ID
      Some("glitch_testnet"),
      // Properties
      Some(json::from_str(DEFAULT_PROPERTIES_TESTNET).unwrap()),
      // Extension
      None,
  ))
}

//Glitch Mainnet
pub fn glitch_mainnet_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;
  Ok(ChainSpec::from_genesis(
      //Name
      "Glitch",
      //ID
      "glitch_mainnet",
      ChainType::Custom(String::from("glitch_mainnet")),
      move || glitch_genesis(
          wasm_binary,
          // Initial PoA authories
          vec![
        // SECRET="release shoulder canyon agree mule snack genre scare furnace coffee fragile sun"
        // 5CmEYEyMpoyKHWde8gZcPUo39dzzhD6y5jp9TxC23opLUzq4
        // subkey inspect "$SECRET//glitch//1//validator"
        // subkey inspect "$SECRET//glitch//1//babe"
        // subkey inspect --scheme ed25519 "$SECRET//glitch//1//grandpa"
        // subkey inspect "$SECRET//glitch//1//imonline"
        // subkey inspect "$SECRET//glitch//1//discovery"
        (
          hex!["aee0df231aed47a99beb26960d156de55751a48741b1cfcfa99e42a5f499e63f"].into(),
          hex!["aee0df231aed47a99beb26960d156de55751a48741b1cfcfa99e42a5f499e63f"].into(),
          hex!["2eed8720f0cc2697ef251da006980c0873cfc108f4c91a96744eedf1b59aff44"].unchecked_into(), // babe key
          hex!["04315411f66a58e838690017ac4102374e4f1dd3f429176ebaa15d55d61d9e80"].unchecked_into(), // grandpa
          hex!["a86ab6e2458078bc49f7277ba00642c1ba2336f8001dd7de6494fd0751a1c057"].unchecked_into(), // imonline
          hex!["1eaa69cbee5e16337c52440455fbc97b839d01bdc4e6d158cf3061d44ba4fc7b"].unchecked_into(), // discovery
        ),
        // SECRET="pumpkin carpet nest snack yard eternal nation fame scare obvious toe promote"
        // 5G6nMq5x8xm3PLxyKXkKEkzAjzQLGbuiDarjBWp6d5XHg3FM
        // subkey inspect "$SECRET//glitch//2//validator"
        // subkey inspect "$SECRET//glitch//2//babe"
        // subkey inspect --scheme ed25519 "$SECRET//glitch//2//grandpa"
        // subkey inspect "$SECRET//glitch//2//imonline"
        // subkey inspect "$SECRET//glitch//2//discovery"
        (
          hex!["5a44973835f643d6d2c45c4490e1c0095755d469ae5db20d480c937588abd164"].into(),
          hex!["5a44973835f643d6d2c45c4490e1c0095755d469ae5db20d480c937588abd164"].into(),
          hex!["381bab65c6c3c0a9bfb06e7a6c8a3b109bed9e97303b22fcd0157ba0871a6566"].unchecked_into(), // babe
          hex!["72f22b083c995d0c4bf07a46f7ad326bcc25780483c8eb0523f53ed2a5b7915d"].unchecked_into(), // grandpa
          hex!["46bd8c5f164df0a6db9199a74376aea3cf8f0d4ecc8b4da5289ebf70c6d0496a"].unchecked_into(), // imonline
          hex!["98a818de9aa0ea6d376a8690bfebb24b1d5d7d9c6f36e1ab45983b0fa7f2e328"].unchecked_into(), // discovery
        ),
        // SECRET="light file noodle black twenty mixed gesture trip desk business reason rural"
        // 5EcCtaankoR4eRuBFFbRKaydUWG7JFp7YPp9oVD7mSQpbhBV
        // subkey inspect "$SECRET//glitch//3//validator"
        // subkey inspect "$SECRET//glitch//3//babe"
        // subkey inspect --scheme ed25519 "$SECRET//glitch//3//grandpa"
        // subkey inspect "$SECRET//glitch//3//imonline"
        // subkey inspect "$SECRET//glitch//3//discovery"
        (
          hex!["94f1d376734418cc137900c7ad4bb5a4911af24d00e4fd72803e3817a50c334d"].into(),
          hex!["94f1d376734418cc137900c7ad4bb5a4911af24d00e4fd72803e3817a50c334d"].into(),
          hex!["6eb9b6f2680b69714d38d0d34108288c9498616589afa6ca453f7a71a88cc64c"].unchecked_into(), // babe
          hex!["0b9f59981f7b9a654f9819c7cb774f3fe39fc9e4c5ac22202f77be52677a5fd3"].unchecked_into(), // grandpa
          hex!["1efe62dcf7953227eb99413539b2d58c584f8568ab72d847310a17030882ef0d"].unchecked_into(), // imonline
          hex!["82aba2132ab4ff2e7fcafeb4d4932f73413d08b7f3a54e18ea2c8d4cffa88e2b"].unchecked_into(), // discovery
        ),
      ],
          // SECRET="assume blanket feature indicate enhance lounge nation spray venture connect kidney dish"
          // 5CFPN3erbXSA7KyE8kM66QZ7izgMba7aHQApvZeT4VeNZYbv
          // subkey inspect "$SECRET//glitch//root"
          hex!["9c6418d848434a5eefcd9c7640e9518850ac4f268b985d73e94d94bede19eb4c"].into(),
          vec![
        // 5CPQQYs3wf32fr5PhmmfFQEeVzD1Zy9Hdo8LFzQYuhP8XHW6
        hex!["9c6418d848434a5eefcd9c7640e9518850ac4f268b985d73e94d94bede19eb4c"].into(),
      ],
          true,
          endowed_evm_account()
      ),
      // Bootnodes
      vec![],
      //Telemetry
      None,
      // Protocol ID
      Some("glitch_mainnet"),
      // Properties
      Some(json::from_str(DEFAULT_PROPERTIES_MAINNET).unwrap()),
      // Extension
      None,
  ))
}

/// Configure initial storage state for FRAME modules.
fn glitch_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        BabeId,
        GrandpaId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
    endowed_eth_accounts: BTreeMap<H160, GenesisAccount>,
) -> GenesisConfig {
    let enable_println = true;

    const ENDOWMENT: Balance = 58_888_588 * DOLLARS;
    const STASH: Balance = 100 * DOLLARS;
    const AUTHOR_BALANCE: Balance = 10_000_0000 * DOLLARS;

    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(
                    initial_authorities
                        .iter()
                        .map(|x| (x.0.clone(), AUTHOR_BALANCE)),
                )
                .collect(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
        }),
        pallet_evm: Some(EVMConfig {
            accounts: endowed_eth_accounts,
        }),
        pallet_ethereum: Some(EthereumConfig {}),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.3.clone(), x.2.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
        pallet_collective_Instance1: Some(Default::default()),
        pallet_collective_Instance2: Some(Default::default()),
        pallet_democracy: Some(Default::default()),
        pallet_treasury: Some(Default::default()),
        pallet_fund: Some(Default::default()),
        pallet_elections_phragmen: Some(Default::default()),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_revenue: Some(RevenueConfig {
            admin_genesis: get_account_id_from_seed::<sr25519::Public>("Alice")
            // admin_genesis: AccountId::from_str("0x1a93011e1af13b6f83ac556c15561b100d06ecaad3c75e37bc77229aa182f92a").unwrap()
        })
    }
}
