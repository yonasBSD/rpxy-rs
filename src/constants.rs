pub const LISTEN_ADDRESSES: &[&str] = &["0.0.0.0", "[::]"];
pub const HTTP_LISTEN_PORT: u32 = 8080;
pub const HTTPS_LISTEN_PORT: u32 = 8443;
pub const TIMEOUT_SEC: u64 = 10;
pub const MAX_CLIENTS: usize = 512;
pub const MAX_CONCURRENT_STREAMS: u32 = 16;
// #[cfg(feature = "tls")]
pub const CERTS_WATCH_DELAY_SECS: u32 = 10;
