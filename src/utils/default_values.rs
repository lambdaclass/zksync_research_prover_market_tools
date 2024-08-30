use zksync_ethers_rs::types::{zksync::protocol_version::VersionPatch, H256};

pub const DEFAULT_DATABASE_SERVER_URL: &str =
    "postgres://postgres:notsecurepassword@localhost/zksync_local";
pub const DEFAULT_DATABASE_PROVER_URL: &str =
    "postgres://postgres:notsecurepassword@localhost/prover_local";
// 0x14f97b81e54b35fe673d8708cc1a19e1ea5b5e348e12d31e39824ed4f42bbca2
pub const DEFAULT_RECURSION_SCHEDULER_VK_HASH: H256 = H256([
    0x14, 0xf9, 0x7b, 0x81, 0xe5, 0x4b, 0x35, 0xfe, 0x67, 0x3d, 0x87, 0x08, 0xcc, 0x1a, 0x19, 0xe1,
    0xea, 0x5b, 0x5e, 0x34, 0x8e, 0x12, 0xd3, 0x1e, 0x39, 0x82, 0x4e, 0xd4, 0xf4, 0x2b, 0xbc, 0xa2,
]);
// 0xf520cd5b37e74e19fdb369c8d676a04dce8a19457497ac6686d2bb95d94109c8
pub const DEFAULT_RECURSION_NODE_VK_HASH: H256 = H256([
    0xf5, 0x20, 0xcd, 0x5b, 0x37, 0xe7, 0x4e, 0x19, 0xfd, 0xb3, 0x69, 0xc8, 0xd6, 0x76, 0xa0, 0x4d,
    0xce, 0x8a, 0x19, 0x45, 0x74, 0x97, 0xac, 0x66, 0x86, 0xd2, 0xbb, 0x95, 0xd9, 0x41, 0x09, 0xc8,
]);
// 0xf9664f4324c1400fa5c3822d667f30e873f53f1b8033180cd15fe41c1e2355c6
pub const DEFAULT_RECURSION_LEAF_VK_HASH: H256 = H256([
    0xf9, 0x66, 0x4f, 0x43, 0x24, 0xc1, 0x40, 0x0f, 0xa5, 0xc3, 0x82, 0x2d, 0x66, 0x7f, 0x30, 0xe8,
    0x73, 0xf5, 0x3f, 0x1b, 0x80, 0x33, 0x18, 0x0c, 0xd1, 0x5f, 0xe4, 0x1c, 0x1e, 0x23, 0x55, 0xc6,
]);
// 0x0000000000000000000000000000000000000000000000000000000000000000
pub const DEFAULT_RECURSION_CIRCUITS_SET_VK_HASH: H256 = H256::zero();
pub const DEFAULT_VERSION_PATCH: VersionPatch = VersionPatch(2);
pub const DEFAULT_PROTOCOL_VERSION: u16 = 24;