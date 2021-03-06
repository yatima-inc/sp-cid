pub type Codec = u64;

pub const IDENTITY: Codec = 0x00;
pub const CIDV1: Codec = 0x01;
pub const CIDV2: Codec = 0x02;
pub const CIDV3: Codec = 0x03;
pub const IP4: Codec = 0x04;
pub const TCP: Codec = 0x06;
pub const SHA1: Codec = 0x11;
pub const SHA2_256: Codec = 0x12;
pub const SHA2_512: Codec = 0x13;
pub const SHA3_512: Codec = 0x14;
pub const SHA3_384: Codec = 0x15;
pub const SHA3_256: Codec = 0x16;
pub const SHA3_224: Codec = 0x17;
pub const SHAKE_128: Codec = 0x18;
pub const SHAKE_256: Codec = 0x19;
pub const KECCAK_224: Codec = 0x1a;
pub const KECCAK_256: Codec = 0x1b;
pub const KECCAK_384: Codec = 0x1c;
pub const KECCAK_512: Codec = 0x1d;
pub const BLAKE3: Codec = 0x1e;
pub const DCCP: Codec = 0x21;
pub const MURMUR3_128: Codec = 0x22;
pub const MURMUR3_32: Codec = 0x23;
pub const IP6: Codec = 0x29;
pub const IP6ZONE: Codec = 0x2a;
pub const PATH: Codec = 0x2f;
pub const MULTICODEC: Codec = 0x30;
pub const MULTIHASH: Codec = 0x31;
pub const MULTIADDR: Codec = 0x32;
pub const MULTIBASE: Codec = 0x33;
pub const DNS: Codec = 0x35;
pub const DNS4: Codec = 0x36;
pub const DNS6: Codec = 0x37;
pub const DNSADDR: Codec = 0x38;
pub const PROTOBUF: Codec = 0x50;
pub const CBOR: Codec = 0x51;
pub const RAW: Codec = 0x55;
pub const DBL_SHA2_256: Codec = 0x56;
pub const RLP: Codec = 0x60;
pub const BENCODE: Codec = 0x63;
pub const DAG_PB: Codec = 0x70;
pub const DAG_CBOR: Codec = 0x71;
pub const LIBP2P_KEY: Codec = 0x72;
pub const GIT_RAW: Codec = 0x78;
pub const TORRENT_INFO: Codec = 0x7b;
pub const TORRENT_FILE: Codec = 0x7c;
pub const LEOFCOIN_BLOCK: Codec = 0x81;
pub const LEOFCOIN_TX: Codec = 0x82;
pub const LEOFCOIN_PR: Codec = 0x83;
pub const SCTP: Codec = 0x84;
pub const DAG_JOSE: Codec = 0x85;
pub const DAG_COSE: Codec = 0x86;
pub const ETH_BLOCK: Codec = 0x90;
pub const ETH_BLOCK_LIST: Codec = 0x91;
pub const ETH_TX_TRIE: Codec = 0x92;
pub const ETH_TX: Codec = 0x93;
pub const ETH_TX_RECEIPT_TRIE: Codec = 0x94;
pub const ETH_TX_RECEIPT: Codec = 0x95;
pub const ETH_STATE_TRIE: Codec = 0x96;
pub const ETH_ACCOUNT_SNAPSHOT: Codec = 0x97;
pub const ETH_STORAGE_TRIE: Codec = 0x98;
pub const ETH_RECEIPT_LOG_TRIE: Codec = 0x99;
pub const ETH_RECIEPT_LOG: Codec = 0x9a;
pub const AES_128: Codec = 0xa0;
pub const AES_192: Codec = 0xa1;
pub const AES_256: Codec = 0xa2;
pub const CHACHA_128: Codec = 0xa3;
pub const CHACHA_256: Codec = 0xa4;
pub const BITCOIN_BLOCK: Codec = 0xb0;
pub const BITCOIN_TX: Codec = 0xb1;
pub const BITCOIN_WITNESS_COMMITMENT: Codec = 0xb2;
pub const ZCASH_BLOCK: Codec = 0xc0;
pub const ZCASH_TX: Codec = 0xc1;
pub const CAIP_50: Codec = 0xca;
pub const STREAMID: Codec = 0xce;
pub const STELLAR_BLOCK: Codec = 0xd0;
pub const STELLAR_TX: Codec = 0xd1;
pub const MD4: Codec = 0xd4;
pub const MD5: Codec = 0xd5;
pub const BMT: Codec = 0xd6;
pub const DECRED_BLOCK: Codec = 0xe0;
pub const DECRED_TX: Codec = 0xe1;
pub const IPLD_NS: Codec = 0xe2;
pub const IPFS_NS: Codec = 0xe3;
pub const SWARM_NS: Codec = 0xe4;
pub const IPNS_NS: Codec = 0xe5;
pub const ZERONET: Codec = 0xe6;
pub const SECP256K1_PUB: Codec = 0xe7;
pub const BLS12_381_G1_PUB: Codec = 0xea;
pub const BLS12_381_G2_PUB: Codec = 0xeb;
pub const X25519_PUB: Codec = 0xec;
pub const ED25519_PUB: Codec = 0xed;
pub const BLS12_381_G1G2_PUB: Codec = 0xee;
pub const DASH_BLOCK: Codec = 0xf0;
pub const DASH_TX: Codec = 0xf1;
pub const SWARM_MANIFEST: Codec = 0xfa;
pub const SWARM_FEED: Codec = 0xfb;
pub const UDP: Codec = 0x0111;
pub const P2P_WEBRTC_STAR: Codec = 0x0113;
pub const P2P_WEBRTC_DIRECT: Codec = 0x0114;
pub const P2P_STARDUST: Codec = 0x0115;
pub const P2P_CIRCUIT: Codec = 0x0122;
pub const DAG_JSON: Codec = 0x0129;
pub const UDT: Codec = 0x012d;
pub const UTP: Codec = 0x012e;
pub const UNIX: Codec = 0x0190;
pub const THREAD: Codec = 0x0196;
pub const P2P: Codec = 0x01a5;
pub const IPFS: Codec = 0x01a5;
pub const HTTPS: Codec = 0x01bb;
pub const ONION: Codec = 0x01bc;
pub const ONION3: Codec = 0x01bd;
pub const GARLIC64: Codec = 0x01be;
pub const GARLIC32: Codec = 0x01bf;
pub const TLS: Codec = 0x01c0;
pub const NOISE: Codec = 0x01c6;
pub const QUIC: Codec = 0x01cc;
pub const WS: Codec = 0x01dd;
pub const WSS: Codec = 0x01de;
pub const P2P_WEBSOCKET_STAR: Codec = 0x01df;
pub const HTTP: Codec = 0x01e0;
pub const SWHID_1_SNP: Codec = 0x01f0;
pub const JSON: Codec = 0x0200;
pub const MESSAGEPACK: Codec = 0x0201;
pub const LIBP2P_PEER_RECORD: Codec = 0x0301;
pub const LIBP2P_RELAY_RSVP: Codec = 0x0302;
pub const CAR_INDEX_SORTED: Codec = 0x0400;
pub const CAR_MULTIHASH_INDEX_SORTED: Codec = 0x0401;
pub const SHA2_256_TRUNC254_PADDED: Codec = 0x1012;
pub const RIPEMD_128: Codec = 0x1052;
pub const RIPEMD_160: Codec = 0x1053;
pub const RIPEMD_256: Codec = 0x1054;
pub const RIPEMD_320: Codec = 0x1055;
pub const X11: Codec = 0x1100;
pub const P256_PUB: Codec = 0x1200;
pub const P384_PUB: Codec = 0x1201;
pub const P521_PUB: Codec = 0x1202;
pub const ED448_PUB: Codec = 0x1203;
pub const X448_PUB: Codec = 0x1204;
pub const RSA_X509_PUB: Codec = 0x1205;
pub const ED25519_PRIV: Codec = 0x1300;
pub const SECP256K1_PRIV: Codec = 0x1301;
pub const X25519_PRIV: Codec = 0x1302;
pub const KANGAROOTWELVE: Codec = 0x1d01;
pub const SM3_256: Codec = 0x534d;
pub const BLAKE2B_8: Codec = 0xb201;
pub const BLAKE2B_16: Codec = 0xb202;
pub const BLAKE2B_24: Codec = 0xb203;
pub const BLAKE2B_32: Codec = 0xb204;
pub const BLAKE2B_40: Codec = 0xb205;
pub const BLAKE2B_48: Codec = 0xb206;
pub const BLAKE2B_56: Codec = 0xb207;
pub const BLAKE2B_64: Codec = 0xb208;
pub const BLAKE2B_72: Codec = 0xb209;
pub const BLAKE2B_80: Codec = 0xb20a;
pub const BLAKE2B_88: Codec = 0xb20b;
pub const BLAKE2B_96: Codec = 0xb20c;
pub const BLAKE2B_104: Codec = 0xb20d;
pub const BLAKE2B_112: Codec = 0xb20e;
pub const BLAKE2B_120: Codec = 0xb20f;
pub const BLAKE2B_128: Codec = 0xb210;
pub const BLAKE2B_136: Codec = 0xb211;
pub const BLAKE2B_144: Codec = 0xb212;
pub const BLAKE2B_152: Codec = 0xb213;
pub const BLAKE2B_160: Codec = 0xb214;
pub const BLAKE2B_168: Codec = 0xb215;
pub const BLAKE2B_176: Codec = 0xb216;
pub const BLAKE2B_184: Codec = 0xb217;
pub const BLAKE2B_192: Codec = 0xb218;
pub const BLAKE2B_200: Codec = 0xb219;
pub const BLAKE2B_208: Codec = 0xb21a;
pub const BLAKE2B_216: Codec = 0xb21b;
pub const BLAKE2B_224: Codec = 0xb21c;
pub const BLAKE2B_232: Codec = 0xb21d;
pub const BLAKE2B_240: Codec = 0xb21e;
pub const BLAKE2B_248: Codec = 0xb21f;
pub const BLAKE2B_256: Codec = 0xb220;
pub const BLAKE2B_264: Codec = 0xb221;
pub const BLAKE2B_272: Codec = 0xb222;
pub const BLAKE2B_280: Codec = 0xb223;
pub const BLAKE2B_288: Codec = 0xb224;
pub const BLAKE2B_296: Codec = 0xb225;
pub const BLAKE2B_304: Codec = 0xb226;
pub const BLAKE2B_312: Codec = 0xb227;
pub const BLAKE2B_320: Codec = 0xb228;
pub const BLAKE2B_328: Codec = 0xb229;
pub const BLAKE2B_336: Codec = 0xb22a;
pub const BLAKE2B_344: Codec = 0xb22b;
pub const BLAKE2B_352: Codec = 0xb22c;
pub const BLAKE2B_360: Codec = 0xb22d;
pub const BLAKE2B_368: Codec = 0xb22e;
pub const BLAKE2B_376: Codec = 0xb22f;
pub const BLAKE2B_384: Codec = 0xb230;
pub const BLAKE2B_392: Codec = 0xb231;
pub const BLAKE2B_400: Codec = 0xb232;
pub const BLAKE2B_408: Codec = 0xb233;
pub const BLAKE2B_416: Codec = 0xb234;
pub const BLAKE2B_424: Codec = 0xb235;
pub const BLAKE2B_432: Codec = 0xb236;
pub const BLAKE2B_440: Codec = 0xb237;
pub const BLAKE2B_448: Codec = 0xb238;
pub const BLAKE2B_456: Codec = 0xb239;
pub const BLAKE2B_464: Codec = 0xb23a;
pub const BLAKE2B_472: Codec = 0xb23b;
pub const BLAKE2B_480: Codec = 0xb23c;
pub const BLAKE2B_488: Codec = 0xb23d;
pub const BLAKE2B_496: Codec = 0xb23e;
pub const BLAKE2B_504: Codec = 0xb23f;
pub const BLAKE2B_512: Codec = 0xb240;
pub const BLAKE2S_8: Codec = 0xb241;
pub const BLAKE2S_16: Codec = 0xb242;
pub const BLAKE2S_24: Codec = 0xb243;
pub const BLAKE2S_32: Codec = 0xb244;
pub const BLAKE2S_40: Codec = 0xb245;
pub const BLAKE2S_48: Codec = 0xb246;
pub const BLAKE2S_56: Codec = 0xb247;
pub const BLAKE2S_64: Codec = 0xb248;
pub const BLAKE2S_72: Codec = 0xb249;
pub const BLAKE2S_80: Codec = 0xb24a;
pub const BLAKE2S_88: Codec = 0xb24b;
pub const BLAKE2S_96: Codec = 0xb24c;
pub const BLAKE2S_104: Codec = 0xb24d;
pub const BLAKE2S_112: Codec = 0xb24e;
pub const BLAKE2S_120: Codec = 0xb24f;
pub const BLAKE2S_128: Codec = 0xb250;
pub const BLAKE2S_136: Codec = 0xb251;
pub const BLAKE2S_144: Codec = 0xb252;
pub const BLAKE2S_152: Codec = 0xb253;
pub const BLAKE2S_160: Codec = 0xb254;
pub const BLAKE2S_168: Codec = 0xb255;
pub const BLAKE2S_176: Codec = 0xb256;
pub const BLAKE2S_184: Codec = 0xb257;
pub const BLAKE2S_192: Codec = 0xb258;
pub const BLAKE2S_200: Codec = 0xb259;
pub const BLAKE2S_208: Codec = 0xb25a;
pub const BLAKE2S_216: Codec = 0xb25b;
pub const BLAKE2S_224: Codec = 0xb25c;
pub const BLAKE2S_232: Codec = 0xb25d;
pub const BLAKE2S_240: Codec = 0xb25e;
pub const BLAKE2S_248: Codec = 0xb25f;
pub const BLAKE2S_256: Codec = 0xb260;
pub const SKEIN256_8: Codec = 0xb301;
pub const SKEIN256_16: Codec = 0xb302;
pub const SKEIN256_24: Codec = 0xb303;
pub const SKEIN256_32: Codec = 0xb304;
pub const SKEIN256_40: Codec = 0xb305;
pub const SKEIN256_48: Codec = 0xb306;
pub const SKEIN256_56: Codec = 0xb307;
pub const SKEIN256_64: Codec = 0xb308;
pub const SKEIN256_72: Codec = 0xb309;
pub const SKEIN256_80: Codec = 0xb30a;
pub const SKEIN256_88: Codec = 0xb30b;
pub const SKEIN256_96: Codec = 0xb30c;
pub const SKEIN256_104: Codec = 0xb30d;
pub const SKEIN256_112: Codec = 0xb30e;
pub const SKEIN256_120: Codec = 0xb30f;
pub const SKEIN256_128: Codec = 0xb310;
pub const SKEIN256_136: Codec = 0xb311;
pub const SKEIN256_144: Codec = 0xb312;
pub const SKEIN256_152: Codec = 0xb313;
pub const SKEIN256_160: Codec = 0xb314;
pub const SKEIN256_168: Codec = 0xb315;
pub const SKEIN256_176: Codec = 0xb316;
pub const SKEIN256_184: Codec = 0xb317;
pub const SKEIN256_192: Codec = 0xb318;
pub const SKEIN256_200: Codec = 0xb319;
pub const SKEIN256_208: Codec = 0xb31a;
pub const SKEIN256_216: Codec = 0xb31b;
pub const SKEIN256_224: Codec = 0xb31c;
pub const SKEIN256_232: Codec = 0xb31d;
pub const SKEIN256_240: Codec = 0xb31e;
pub const SKEIN256_248: Codec = 0xb31f;
pub const SKEIN256_256: Codec = 0xb320;
pub const SKEIN512_8: Codec = 0xb321;
pub const SKEIN512_16: Codec = 0xb322;
pub const SKEIN512_24: Codec = 0xb323;
pub const SKEIN512_32: Codec = 0xb324;
pub const SKEIN512_40: Codec = 0xb325;
pub const SKEIN512_48: Codec = 0xb326;
pub const SKEIN512_56: Codec = 0xb327;
pub const SKEIN512_64: Codec = 0xb328;
pub const SKEIN512_72: Codec = 0xb329;
pub const SKEIN512_80: Codec = 0xb32a;
pub const SKEIN512_88: Codec = 0xb32b;
pub const SKEIN512_96: Codec = 0xb32c;
pub const SKEIN512_104: Codec = 0xb32d;
pub const SKEIN512_112: Codec = 0xb32e;
pub const SKEIN512_120: Codec = 0xb32f;
pub const SKEIN512_128: Codec = 0xb330;
pub const SKEIN512_136: Codec = 0xb331;
pub const SKEIN512_144: Codec = 0xb332;
pub const SKEIN512_152: Codec = 0xb333;
pub const SKEIN512_160: Codec = 0xb334;
pub const SKEIN512_168: Codec = 0xb335;
pub const SKEIN512_176: Codec = 0xb336;
pub const SKEIN512_184: Codec = 0xb337;
pub const SKEIN512_192: Codec = 0xb338;
pub const SKEIN512_200: Codec = 0xb339;
pub const SKEIN512_208: Codec = 0xb33a;
pub const SKEIN512_216: Codec = 0xb33b;
pub const SKEIN512_224: Codec = 0xb33c;
pub const SKEIN512_232: Codec = 0xb33d;
pub const SKEIN512_240: Codec = 0xb33e;
pub const SKEIN512_248: Codec = 0xb33f;
pub const SKEIN512_256: Codec = 0xb340;
pub const SKEIN512_264: Codec = 0xb341;
pub const SKEIN512_272: Codec = 0xb342;
pub const SKEIN512_280: Codec = 0xb343;
pub const SKEIN512_288: Codec = 0xb344;
pub const SKEIN512_296: Codec = 0xb345;
pub const SKEIN512_304: Codec = 0xb346;
pub const SKEIN512_312: Codec = 0xb347;
pub const SKEIN512_320: Codec = 0xb348;
pub const SKEIN512_328: Codec = 0xb349;
pub const SKEIN512_336: Codec = 0xb34a;
pub const SKEIN512_344: Codec = 0xb34b;
pub const SKEIN512_352: Codec = 0xb34c;
pub const SKEIN512_360: Codec = 0xb34d;
pub const SKEIN512_368: Codec = 0xb34e;
pub const SKEIN512_376: Codec = 0xb34f;
pub const SKEIN512_384: Codec = 0xb350;
pub const SKEIN512_392: Codec = 0xb351;
pub const SKEIN512_400: Codec = 0xb352;
pub const SKEIN512_408: Codec = 0xb353;
pub const SKEIN512_416: Codec = 0xb354;
pub const SKEIN512_424: Codec = 0xb355;
pub const SKEIN512_432: Codec = 0xb356;
pub const SKEIN512_440: Codec = 0xb357;
pub const SKEIN512_448: Codec = 0xb358;
pub const SKEIN512_456: Codec = 0xb359;
pub const SKEIN512_464: Codec = 0xb35a;
pub const SKEIN512_472: Codec = 0xb35b;
pub const SKEIN512_480: Codec = 0xb35c;
pub const SKEIN512_488: Codec = 0xb35d;
pub const SKEIN512_496: Codec = 0xb35e;
pub const SKEIN512_504: Codec = 0xb35f;
pub const SKEIN512_512: Codec = 0xb360;
pub const SKEIN1024_8: Codec = 0xb361;
pub const SKEIN1024_16: Codec = 0xb362;
pub const SKEIN1024_24: Codec = 0xb363;
pub const SKEIN1024_32: Codec = 0xb364;
pub const SKEIN1024_40: Codec = 0xb365;
pub const SKEIN1024_48: Codec = 0xb366;
pub const SKEIN1024_56: Codec = 0xb367;
pub const SKEIN1024_64: Codec = 0xb368;
pub const SKEIN1024_72: Codec = 0xb369;
pub const SKEIN1024_80: Codec = 0xb36a;
pub const SKEIN1024_88: Codec = 0xb36b;
pub const SKEIN1024_96: Codec = 0xb36c;
pub const SKEIN1024_104: Codec = 0xb36d;
pub const SKEIN1024_112: Codec = 0xb36e;
pub const SKEIN1024_120: Codec = 0xb36f;
pub const SKEIN1024_128: Codec = 0xb370;
pub const SKEIN1024_136: Codec = 0xb371;
pub const SKEIN1024_144: Codec = 0xb372;
pub const SKEIN1024_152: Codec = 0xb373;
pub const SKEIN1024_160: Codec = 0xb374;
pub const SKEIN1024_168: Codec = 0xb375;
pub const SKEIN1024_176: Codec = 0xb376;
pub const SKEIN1024_184: Codec = 0xb377;
pub const SKEIN1024_192: Codec = 0xb378;
pub const SKEIN1024_200: Codec = 0xb379;
pub const SKEIN1024_208: Codec = 0xb37a;
pub const SKEIN1024_216: Codec = 0xb37b;
pub const SKEIN1024_224: Codec = 0xb37c;
pub const SKEIN1024_232: Codec = 0xb37d;
pub const SKEIN1024_240: Codec = 0xb37e;
pub const SKEIN1024_248: Codec = 0xb37f;
pub const SKEIN1024_256: Codec = 0xb380;
pub const SKEIN1024_264: Codec = 0xb381;
pub const SKEIN1024_272: Codec = 0xb382;
pub const SKEIN1024_280: Codec = 0xb383;
pub const SKEIN1024_288: Codec = 0xb384;
pub const SKEIN1024_296: Codec = 0xb385;
pub const SKEIN1024_304: Codec = 0xb386;
pub const SKEIN1024_312: Codec = 0xb387;
pub const SKEIN1024_320: Codec = 0xb388;
pub const SKEIN1024_328: Codec = 0xb389;
pub const SKEIN1024_336: Codec = 0xb38a;
pub const SKEIN1024_344: Codec = 0xb38b;
pub const SKEIN1024_352: Codec = 0xb38c;
pub const SKEIN1024_360: Codec = 0xb38d;
pub const SKEIN1024_368: Codec = 0xb38e;
pub const SKEIN1024_376: Codec = 0xb38f;
pub const SKEIN1024_384: Codec = 0xb390;
pub const SKEIN1024_392: Codec = 0xb391;
pub const SKEIN1024_400: Codec = 0xb392;
pub const SKEIN1024_408: Codec = 0xb393;
pub const SKEIN1024_416: Codec = 0xb394;
pub const SKEIN1024_424: Codec = 0xb395;
pub const SKEIN1024_432: Codec = 0xb396;
pub const SKEIN1024_440: Codec = 0xb397;
pub const SKEIN1024_448: Codec = 0xb398;
pub const SKEIN1024_456: Codec = 0xb399;
pub const SKEIN1024_464: Codec = 0xb39a;
pub const SKEIN1024_472: Codec = 0xb39b;
pub const SKEIN1024_480: Codec = 0xb39c;
pub const SKEIN1024_488: Codec = 0xb39d;
pub const SKEIN1024_496: Codec = 0xb39e;
pub const SKEIN1024_504: Codec = 0xb39f;
pub const SKEIN1024_512: Codec = 0xb3a0;
pub const SKEIN1024_520: Codec = 0xb3a1;
pub const SKEIN1024_528: Codec = 0xb3a2;
pub const SKEIN1024_536: Codec = 0xb3a3;
pub const SKEIN1024_544: Codec = 0xb3a4;
pub const SKEIN1024_552: Codec = 0xb3a5;
pub const SKEIN1024_560: Codec = 0xb3a6;
pub const SKEIN1024_568: Codec = 0xb3a7;
pub const SKEIN1024_576: Codec = 0xb3a8;
pub const SKEIN1024_584: Codec = 0xb3a9;
pub const SKEIN1024_592: Codec = 0xb3aa;
pub const SKEIN1024_600: Codec = 0xb3ab;
pub const SKEIN1024_608: Codec = 0xb3ac;
pub const SKEIN1024_616: Codec = 0xb3ad;
pub const SKEIN1024_624: Codec = 0xb3ae;
pub const SKEIN1024_632: Codec = 0xb3af;
pub const SKEIN1024_640: Codec = 0xb3b0;
pub const SKEIN1024_648: Codec = 0xb3b1;
pub const SKEIN1024_656: Codec = 0xb3b2;
pub const SKEIN1024_664: Codec = 0xb3b3;
pub const SKEIN1024_672: Codec = 0xb3b4;
pub const SKEIN1024_680: Codec = 0xb3b5;
pub const SKEIN1024_688: Codec = 0xb3b6;
pub const SKEIN1024_696: Codec = 0xb3b7;
pub const SKEIN1024_704: Codec = 0xb3b8;
pub const SKEIN1024_712: Codec = 0xb3b9;
pub const SKEIN1024_720: Codec = 0xb3ba;
pub const SKEIN1024_728: Codec = 0xb3bb;
pub const SKEIN1024_736: Codec = 0xb3bc;
pub const SKEIN1024_744: Codec = 0xb3bd;
pub const SKEIN1024_752: Codec = 0xb3be;
pub const SKEIN1024_760: Codec = 0xb3bf;
pub const SKEIN1024_768: Codec = 0xb3c0;
pub const SKEIN1024_776: Codec = 0xb3c1;
pub const SKEIN1024_784: Codec = 0xb3c2;
pub const SKEIN1024_792: Codec = 0xb3c3;
pub const SKEIN1024_800: Codec = 0xb3c4;
pub const SKEIN1024_808: Codec = 0xb3c5;
pub const SKEIN1024_816: Codec = 0xb3c6;
pub const SKEIN1024_824: Codec = 0xb3c7;
pub const SKEIN1024_832: Codec = 0xb3c8;
pub const SKEIN1024_840: Codec = 0xb3c9;
pub const SKEIN1024_848: Codec = 0xb3ca;
pub const SKEIN1024_856: Codec = 0xb3cb;
pub const SKEIN1024_864: Codec = 0xb3cc;
pub const SKEIN1024_872: Codec = 0xb3cd;
pub const SKEIN1024_880: Codec = 0xb3ce;
pub const SKEIN1024_888: Codec = 0xb3cf;
pub const SKEIN1024_896: Codec = 0xb3d0;
pub const SKEIN1024_904: Codec = 0xb3d1;
pub const SKEIN1024_912: Codec = 0xb3d2;
pub const SKEIN1024_920: Codec = 0xb3d3;
pub const SKEIN1024_928: Codec = 0xb3d4;
pub const SKEIN1024_936: Codec = 0xb3d5;
pub const SKEIN1024_944: Codec = 0xb3d6;
pub const SKEIN1024_952: Codec = 0xb3d7;
pub const SKEIN1024_960: Codec = 0xb3d8;
pub const SKEIN1024_968: Codec = 0xb3d9;
pub const SKEIN1024_976: Codec = 0xb3da;
pub const SKEIN1024_984: Codec = 0xb3db;
pub const SKEIN1024_992: Codec = 0xb3dc;
pub const SKEIN1024_1000: Codec = 0xb3dd;
pub const SKEIN1024_1008: Codec = 0xb3de;
pub const SKEIN1024_1016: Codec = 0xb3df;
pub const SKEIN1024_1024: Codec = 0xb3e0;
pub const POSEIDON_BLS12_381_A2_FC1: Codec = 0xb401;
pub const POSEIDON_BLS12_381_A2_FC1_SC: Codec = 0xb402;
pub const ZEROXCERT_IMPRINT_256: Codec = 0xce11;
pub const FIL_COMMITMENT_UNSEALED: Codec = 0xf101;
pub const FIL_COMMITMENT_SEALED: Codec = 0xf102;
pub const HOLOCHAIN_ADR_V0: Codec = 0x807124;
pub const HOLOCHAIN_ADR_V1: Codec = 0x817124;
pub const HOLOCHAIN_KEY_V0: Codec = 0x947124;
pub const HOLOCHAIN_KEY_V1: Codec = 0x957124;
pub const HOLOCHAIN_SIG_V0: Codec = 0xa27124;
pub const HOLOCHAIN_SIG_V1: Codec = 0xa37124;
pub const SKYNET_NS: Codec = 0xb19910;
pub const ARWEAVE_NS: Codec = 0xb29910;
