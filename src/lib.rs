mod include_bytes_aligned;

pub const EBPF_BINARY: &[u8] = include_bytes_aligned!(concat!(env!("OUT_DIR"), "/ockam_ebpf"));
