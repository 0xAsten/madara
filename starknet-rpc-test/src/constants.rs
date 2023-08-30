use starknet_ff::FieldElement;

pub const TEST_CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000001111";
pub const ACCOUNT_CONTRACT: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const CAIRO_1_ACCOUNT_CONTRACT: &str = "0x0000000000000000000000000000000000000000000000000000000000000004";
pub const CAIRO_1_ACCOUNT_CONTRACT_CLASS_HASH: &str =
    "0x35ccefcf9d5656da623468e27e682271cd327af196785df99e7fee1436b6276";
pub const ERC20_CAIRO_0_CONTRACT: &str = "0x040e59c2c182a58fb0a74349bfa4769cbbcba32547591dd3fb1def8623997d00";

// https://github.com/keep-starknet-strange/madara/blob/main/crates/node/src/chain_spec.rs#L185-L186
pub const ACCOUNT_CONTRACT_CLASS_HASH: &str = "0x0279d77db761fba82e0054125a6fdb5f6baa6286fa3fb73450cc44d193c2d37f";
pub const ARGENT_PROXY_CLASS_HASH: &str = "0x0424b7f61e3c5dfd74400d96fdea7e1f0bf2757f31df04387eaa957f095dd7b9";
pub const SIGNER_PUBLIC: &str = "0x03603a2692a2ae60abb343e832ee53b55d6b25f02a3ef1565ec691edc7a209b2";
pub const SIGNER_PRIVATE: &str = "0x00c1cf1490de1352865301bb8705143f3ef938f97fdf892f1090dcb5ac7bcd1d";
pub const SALT: &str = "0x0000000000000000000000000000000000000000000000000000000000001111";

// https://github.com/keep-starknet-strange/madara/blob/main/crates/node/src/chain_spec.rs#L191-L192
pub const TEST_CONTRACT_CLASS_HASH: &str = "0x0000000000000000000000000000000000000000000000000000000000001000";
pub const MINT_AMOUNT: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const DEPLOY_ACCOUNT_COST: &str = "0x00000000000000000000000000000000000000000000000000000000ffffffff";
pub const CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const FEE_TOKEN_ADDRESS: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub const TOKEN_CLASS_HASH: &str = "0x0000000000000000000000000000000000000000000000000000000000010000";
pub const ARGENT_CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000000002";

// Taken from https://github.com/0xSpaceShard/starknet-devnet-rs/blob/main/crates/starknet-server/tests/common/mod.rs#L5
pub const MIN_PORT: u16 = 1025;
pub const MAX_PORT: u16 = 65_535;

pub const MAX_U256: &str = "0xffffffffffffffffffffffffffffffff";

// Need to use `from_mont` because this needs to be a constant function call
/// ChainId for Starknet Goerli testnet
pub const SN_GOERLI_CHAIN_ID: FieldElement =
    FieldElement::from_mont([3753493103916128178, 18446744073709548950, 18446744073709551615, 398700013197595345]);
