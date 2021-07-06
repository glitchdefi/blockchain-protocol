use serde_json::json;
use sp_core::{Pair, Public, sr25519, U256};
use glitch_node_runtime::{
	AccountId, BabeConfig, Balance, AuthorityDiscoveryConfig, BalancesConfig, ContractsConfig, IndicesConfig, GenesisConfig, ImOnlineId,
	GrandpaConfig, SessionConfig, SessionKeys, StakingConfig, SudoConfig, SystemConfig, WASM_BINARY,
	Signature, StakerStatus,
	EVMConfig, EthereumConfig, DOLLARS
};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{traits::{IdentifyAccount, Verify}, Perbill};
use sc_service::ChainType;
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;
use sp_core::crypto::UncheckedInto;
use std::collections::BTreeMap;
use pallet_evm::GenesisAccount;
use primitive_types::H160;
use std::str::FromStr;
use serde_json as json;

// The URL for the telemetry server.
const TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

const DEFAULT_PROPERTIES_TESTNET: &str = r#"
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
	SessionKeys { grandpa, babe, im_online, authority_discovery, }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

fn endowed_evm_account() -> BTreeMap<H160, GenesisAccount>{
	let endowed_account = vec![
		// glitch_node fauct
		H160::from_str("8097c3C354652CB1EEed3E5B65fBa2576470678A").unwrap()
	];
	get_endowed_evm_accounts(endowed_account)
}

fn dev_endowed_evm_accounts() -> BTreeMap<H160, GenesisAccount>{
	let endowed_account = vec![
		H160::from_str("8097c3C354652CB1EEed3E5B65fBa2576470678A").unwrap(),
		H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b").unwrap(),
		H160::from_str("e6206C7f064c7d77C6d8e3eD8601c9AA435419cE").unwrap(),
		// the dev account key
		// seed: bottom drive obey lake curtain smoke basket hold race lonely fit walk
		// private key: 0x03183f27e9d78698a05c24eb6732630eb17725fcf2b53ee3a6a635d6ff139680
		H160::from_str("aed40f2261ba43b4dffe484265ce82d8ffe2b4db").unwrap()
	];

	get_endowed_evm_accounts(endowed_account)
}

fn get_endowed_evm_accounts(endowed_account: Vec<H160>) -> BTreeMap<H160, GenesisAccount>{
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
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
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
			dev_endowed_evm_accounts()
		),
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
		move || testnet_genesis(
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
			endowed_evm_account()
		),
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

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId,)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
	endowed_eth_accounts: BTreeMap<H160, GenesisAccount>,
) -> GenesisConfig {
	let enable_println = true;

	const ENDOWMENT: Balance = 1_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;
	const AUTHOR_BALANCE: Balance = 200 * DOLLARS;

	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), AUTHOR_BALANCE)))
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
		pallet_indices: Some(IndicesConfig {
			indices: vec![],
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.3.clone(),
					x.2.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(Default::default()),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_collective_Instance1: Some(Default::default()),
		pallet_collective_Instance2: Some(Default::default()),
		pallet_democracy: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_elections_phragmen: Some(Default::default()),
		pallet_membership_Instance1: Some(Default::default()),
	}
}
