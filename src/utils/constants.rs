pub const TX_ID_LEN: usize = 64;
pub const PUBKEY_LEN: usize = 33;
pub const SIG_LEN: usize = 64;
pub const TX_DB_PATH: &str = ".geodesic/db/";
pub const IBLT_CHECKSUM_LEN: usize = 8;
pub const IBLT_PAYLOAD_LEN: usize = 64;
pub const SKETCH_CAPACITY: usize = 8;
pub const NONCE_HEARTBEAT_PERIOD_SEC: u64 = 3;
pub const NONCE_HEARTBEAT_PERIOD_NANO: u32 = 0;
pub const ODDSKETCH_HEARTBEAT_PERIOD_SEC: u64 = 7;
pub const ODDSKETCH_HEARTBEAT_PERIOD_NANO: u32 = 0;
pub const RECONCILE_HEARTBEAT_PERIOD_SEC: u64 = 10;
pub const RECONCILE_HEARTBEAT_PERIOD_NANO: u32 = 0;
pub const SERVER_PORT: u16 = 8080;
pub const RPC_SERVER_PORT: u16 = 8332;
pub const MINER: bool = true;