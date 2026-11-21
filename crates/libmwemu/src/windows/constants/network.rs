pub const AF_UNSPEC: u32 = 0;
pub const AF_INET: u32 = 2;
pub const AF_INET6: u32 = 23;

pub const SOCK_STREAM: u32 = 1;
pub const SOCK_DGRAM: u32 = 0x2;

pub const IPPROTO_TCP: u32 = 6;
pub const IPPROTO_UDP: u32 = 0x11;

// FIXED_INFO node types
pub const BROADCAST_NODETYPE: u32 = 0x0001;
pub const PEER_TO_PEER_NODETYPE: u32 = 0x0002;
pub const MIXED_NODETYPE: u32 = 0x0004;
pub const HYBRID_NODETYPE: u32 = 0x0008;

pub const INTERNET_DEFAULT_HTTP_PORT: u16 = 80;
pub const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443;
pub const CACHEGROUP_FLAG_NONPURGEABLE: u32 = 1;
pub const CACHEGROUP_FLAG_FLUSHURL_ONDELETE: u32 = 2;
pub const CACHEGROUP_FLAG_GIDONLY: u32 = 4;

pub const INTERNET_CONNECTION_OFFLINE: u32 = 0x20;
pub const INTERNET_SERVICE_HTTP: u32 = 3;

pub const INTERNET_HANDLE_TYPE_INTERNET: u32 = 1;
pub const INTERNET_HANDLE_TYPE_CONNECT_HTTP: u32 = 4;
pub const INTERNET_HANDLE_TYPE_HTTP_REQUEST: u32 = 13;

pub const INTERNET_OPTION_HANDLE_TYPE: u32 = 9;
pub const INTERNET_OPTION_URL: u32 = 34;
pub const INTERNET_OPTION_ALTER_IDENTITY: u32 = 80;
pub const INTERNET_OPTION_ASYNC: u32 = 30;
pub const INTERNET_OPTION_ASYNC_ID: u32 = 15;
pub const INTERNET_OPTION_ASYNC_PRIORITY: u32 = 16;
pub const INTERNET_OPTION_BYPASS_EDITED_ENTRY: u32 = 64;
pub const INTERNET_OPTION_CACHE_STREAM_HANDLE: u32 = 27;
pub const INTERNET_OPTION_CACHE_TIMESTAMPS: u32 = 69;
pub const INTERNET_OPTION_CALLBACK: u32 = 1;
pub const INTERNET_OPTION_CALLBACK_FILTER: u32 = 54;
pub const INTERNET_OPTION_CLIENT_CERT_CONTEXT: u32 = 84;
pub const INTERNET_OPTION_CODEPAGE: u32 = 68;
pub const INTERNET_OPTION_CODEPAGE_PATH: u32 = 100;
pub const INTERNET_OPTION_CODEPAGE_EXTRA: u32 = 101;
pub const INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH: u32 = 147;
pub const INTERNET_OPTION_CONNECT_BACKOFF: u32 = 4;
pub const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3;
pub const INTERNET_OPTION_CONNECT_TIME: u32 = 55;
pub const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2;
pub const INTERNET_OPTION_CONNECTED_STATE: u32 = 50;
pub const INTERNET_OPTION_CONTEXT_VALUE: u32 = 45;
pub const INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT: u32 = 6;
pub const INTERNET_OPTION_CONTROL_SEND_TIMEOUT: u32 = 5;
pub const INTERNET_OPTION_DATA_RECEIVE_TIMEOUT: u32 = 8;
pub const INTERNET_OPTION_DATA_SEND_TIMEOUT: u32 = 7;

// https://docs.microsoft.com/en-us/windows/win32/wininet/api-flags
pub const INTERNET_FLAG_SECURE: u64 = 0x00800000;

// Winsock data and return values
pub const WSADESCRIPTION_LEN: usize = 256;
pub const WSASYS_STATUS_LEN: usize = 128;

pub const SOCKET_ERROR: u64 = 0xffff_ffff;

pub const SOL_SOCKET: u32 = 0xffff;
pub const SO_ACCEPTCONN: u32 = 0x2;
pub const SO_KEEPALIVE: u32 = 0x8;
pub const SO_BROADCAST: u32 = 0x20;
pub const SO_LINGER: u32 = 0x80;
pub const SO_OOBINLINE: u32 = 0x100;
pub const SO_SNDBUF: u32 = 0x1001;
pub const SO_RCVBUF: u32 = 0x1002;
pub const SO_SNDTIMEO: u32 = 0x1005;
pub const SO_RCVTIMEO: u32 = 0x1006;
pub const SO_ERROR: u32 = 0x1007;
pub const SO_TYPE: u32 = 0x1008;

pub const TCP_NODELAY: u32 = 0x1;

pub const FIONBIO: u32 = 0x8004667e;
pub const FIONREAD: u32 = 0x4004667f;

pub const SD_RECEIVE: u32 = 0x0;
pub const SD_SEND: u32 = 0x1;
pub const SD_BOTH: u32 = 0x2;

pub const WSA_FLAG_OVERLAPPED: u32 = 0x1;
pub const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 0x80;

pub const INVALID_SOCKET: u64 = 0xffff_ffff_ffff_ffff;

// Name lookup flags
pub const NI_NOFQDN: u32 = 0x01;
pub const NI_NUMERICHOST: u32 = 0x02;
pub const NI_NAMEREQD: u32 = 0x04;
pub const NI_NUMERICSERV: u32 = 0x08;
pub const NI_DGRAM: u32 = 0x10;
