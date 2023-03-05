//! Constants for using Poseidon with the Pallas field.
//!
//! The constants can be reproduced by running the following Sage script from
//! [this repository](https://github.com/daira/pasta-hadeshash):
//!
//! ```text
//! $ sage generate_parameters_grain.sage 1 0 255 3 8 56 0x40000000000000000000000000000000224698fc094cf91b992d30ed00000001
//! ```
// use halo2curves::pasta::pallas;
use halo2_proofs::halo2curves::{
    pasta::{pallas, Fp},
    secp256k1,
    // secp256k1
    secp256k1::Fp as SecFp,
    secp256k1::Fq as SecFq,
    // secp256k1::Fp as SecFp,
};

// Number of round constants: 192
// Round constants for GF(p):
pub(crate) const ROUND_CONSTANTS: [[SecFp; 3]; 64] = [
    [
        SecFp::from_raw([
            0xcf27ab8062e32d20,
            0x62ebf45db50b3c82,
            0x9000679706932707,
            0x218fe5b866ffd7f1,
        ]),
        SecFp::from_raw([
            0xc7f1a5012e3364d2,
            0x4420d11efd62db69,
            0x5bb31eefb1706629,
            0xd9024a16f5ea7a31,
        ]),
        SecFp::from_raw([
            0x464f891edd725401,
            0x4420d11efd62db69,
            0x14c839c75ec06c41,
            0x480abcea775cf581,
        ]),
    ],
    [
        SecFp::from_raw([
            0xdab03b3d1ab89812,
            0x5ec1cf0b5ff1ae75,
            0x1426b46cfcec030b,
            0x937ec4a1d18ec619,
        ]),
        SecFp::from_raw([
            0x5e85b08e05fce7c8,
            0x134ced2d07f50024,
            0xe77fec27c813f03e,
            0x87845234ff2f657d,
        ]),
        SecFp::from_raw([
            0xea3b4a59f1857a10,
            0x134ced2d07f50024,
            0xead599da6dfe7994,
            0x8eb4ec86a61fc535,
        ]),
    ],
    [
        SecFp::from_raw([
            0xba61e205dadfbd78,
            0x4f0d4a2922bab79f,
            0xe290b34f91e736aa,
            0xefb46e12766cb804,
        ]),
        SecFp::from_raw([
            0x3790579438ce60d0,
            0x6d4e30e1fa8cd7e0,
            0xc17598d9451f2940,
            0xcfba216b4f00dac0,
        ]),
        SecFp::from_raw([
            0x4a2f32bcd3251e7b,
            0x6d4e30e1fa8cd7e0,
            0xdbc2b1ddb6b4d243,
            0x467f4558ebfeb775,
        ]),
    ],
    [
        SecFp::from_raw([
            0x2a7240096abb90cd,
            0xd22ee17641145fd3,
            0x04a3814616074b80,
            0x50b8d7812001a841,
        ]),
        SecFp::from_raw([
            0x2b0b020bfc0124f5,
            0x47bfd0704f539d9b,
            0xba63a31f3c4489dd,
            0x2edaf2d9b6dc4211,
        ]),
        SecFp::from_raw([
            0xe5587ec04028b2f8,
            0x47bfd0704f539d9b,
            0x57abd467fd91d342,
            0x2183cc13652e8cb7,
        ]),
    ],
    [
        SecFp::from_raw([
            0xed63223cae10e702,
            0xe437776381778f19,
            0x03a1da508148d857,
            0x2a9bc6ed40a707b4,
        ]),
        SecFp::from_raw([
            0x37cb6027509b812d,
            0x09faa67b6b633661,
            0xd572a54322883ddf,
            0x33245c2edf1e3222,
        ]),
        SecFp::from_raw([
            0x1bed2c1525133850,
            0x09faa67b6b633661,
            0x5af2c0f07b9c2d92,
            0x15d215160cf2e0a5,
        ]),
    ],
    [
        SecFp::from_raw([
            0xa90959126673b41f,
            0x4119f759023fa412,
            0x807b23ee25815011,
            0x50c6d61b56fe4b3d,
        ]),
        SecFp::from_raw([
            0x79521405497817ea,
            0xa1e9e33183ee6938,
            0xf30dc8a6b4431039,
            0x8a5c4ac8724eb4a7,
        ]),
        SecFp::from_raw([
            0xcf4255f15ee2b168,
            0xa1e9e33183ee6938,
            0xa9ac30acb8c6c6fe,
            0xd5902ae9cb153c61,
        ]),
    ],
    [
        SecFp::from_raw([
            0x9fd15eab57070ee1,
            0x104e068a1603a820,
            0x1adc68136039c704,
            0xdbed679ec3ca3471,
        ]),
        SecFp::from_raw([
            0xa6b2ffb97a14af83,
            0xf2a8a45d169afd7a,
            0x83dee0aa7208e361,
            0x7324825f3bd54ebc,
        ]),
        SecFp::from_raw([
            0x57060969c8af8019,
            0xf2a8a45d169afd7a,
            0xc55f0ea59885b5e1,
            0xe1563b41aca1f869,
        ]),
    ],
    [
        SecFp::from_raw([
            0x2e370a08f73fd25d,
            0x1e268275d34be575,
            0xd90c9079b0a9d389,
            0x3ba679cc115e909e,
        ]),
        SecFp::from_raw([
            0x03034fe965643856,
            0x9eedca077e241eae,
            0x669f505a39b0a618,
            0x0f9bd05372e74000,
        ]),
        SecFp::from_raw([
            0x87cb1d6eeb697ef1,
            0x9eedca077e241eae,
            0x5c03245051afba12,
            0x2abc4951c596b4ac,
        ]),
    ],
    [
        SecFp::from_raw([
            0xb196c1108aa1174a,
            0x6896973485961aac,
            0x90f8d5a46f0ffb45,
            0x90f7a7fca1d19bcb,
        ]),
        SecFp::from_raw([
            0xce9c45b7aae68d1e,
            0xb2091278ef77d84d,
            0x0766c51930c75f51,
            0x811f577500e45186,
        ]),
        SecFp::from_raw([
            0x2dfb8d2074945023,
            0xb2091278ef77d84d,
            0x6583eaf5fa849b18,
            0x6388feb7e34bcfae,
        ]),
    ],
    [
        SecFp::from_raw([
            0x03d4403e3a2ca03b,
            0xc45da5b1dfb44651,
            0x7e33ca9c6719614e,
            0x00b16aac29c19585,
        ]),
        SecFp::from_raw([
            0xbea5d9a66759838d,
            0x9b652b435d85d96f,
            0x1406227cafbb7407,
            0xdfff5c85699a99cb,
        ]),
        SecFp::from_raw([
            0xd71c36e80e285d30,
            0x9b652b435d85d96f,
            0x296cce918222ae49,
            0x345ca1eaf0ac3bac,
        ]),
    ],
    [
        SecFp::from_raw([
            0x1ff51df6db2c71ea,
            0xfc515dec1e06754f,
            0xd5dedb9638c58ba1,
            0x64878579c501dc26,
        ]),
        SecFp::from_raw([
            0xd621c0ecd001df11,
            0x435b1c6645ce0141,
            0x189edb24ac15da42,
            0x6a6ee0e8068ed5fd,
        ]),
        SecFp::from_raw([
            0x790b690f93045162,
            0x435b1c6645ce0141,
            0xc10424ef5130cf02,
            0x5e153b1150b533ae,
        ]),
    ],
    [
        SecFp::from_raw([
            0x0e67ec68fb9506fb,
            0x1ae7d30216eec8ad,
            0xaf4c6382767ba68b,
            0x198a2ad8fe376c74,
        ]),
        SecFp::from_raw([
            0x1f118234e2ce4cd5,
            0x34ce00c0ff6dfaf9,
            0xa5abfcbd453154eb,
            0x850ff4851e8dfaba,
        ]),
        SecFp::from_raw([
            0x095a9b19d3146409,
            0x34ce00c0ff6dfaf9,
            0xf6cda3f41e023013,
            0xb4e64488e1224a78,
        ]),
    ],
    [
        SecFp::from_raw([
            0x99a6521c6ad4bbdf,
            0xa176d33a2cf45d2e,
            0x1d64a2f3482b94f4,
            0x0fd79cc4fe76de03,
        ]),
        SecFp::from_raw([
            0xd145ead2f6db6abd,
            0xe00d595c37aefa8c,
            0xea6505795bf6527c,
            0xa67476e131d2a9b2,
        ]),
        SecFp::from_raw([
            0x97a2b1fd505095d9,
            0xe00d595c37aefa8c,
            0x4736f6698f24d178,
            0x0fcf3bcba62e8ac7,
        ]),
    ],
    [
        SecFp::from_raw([
            0x58057b60daa72d72,
            0xd605033e1e34aefd,
            0xcc6ed89d73797c40,
            0x35e71a60ebcd87f0,
        ]),
        SecFp::from_raw([
            0x37f56c53f93cd99c,
            0xc7e4320394a5d806,
            0xf56780bd7d9c9f2d,
            0x15826935ea9dfbc1,
        ]),
        SecFp::from_raw([
            0x41799197aa332cc8,
            0xc7e4320394a5d806,
            0x977b1d8e991101fb,
            0x1763b7d1c7e4b4ab,
        ]),
    ],
    [
        SecFp::from_raw([
            0xb0c88006380ae399,
            0x719818b795e76255,
            0xb53afd7e64342bc7,
            0x12112f5b5991350d,
        ]),
        SecFp::from_raw([
            0x78c7a8ae6434655a,
            0x0d220e2e4f1ab336,
            0xb938029ae8d79238,
            0x43413105eb01be49,
        ]),
        SecFp::from_raw([
            0xdab08b79ca670d9d,
            0x0d220e2e4f1ab336,
            0xa9a14371a359eab4,
            0x8d694c20161f56bf,
        ]),
    ],
    [
        SecFp::from_raw([
            0xfd1d8236c2ac0f34,
            0xc800bd29919a0cdd,
            0x5ec8246f2aa67000,
            0x37e40345d90178b5,
        ]),
        SecFp::from_raw([
            0x112ca74bd8f1d86a,
            0x0180e16ccf9cef4e,
            0x1b32563e7b17cfd0,
            0x21d6d115826560f4,
        ]),
        SecFp::from_raw([
            0xca9f79fad59b8160,
            0x0180e16ccf9cef4e,
            0x7082f5d7ec16e244,
            0x71f5c70fb0ff4dd6,
        ]),
    ],
    [
        SecFp::from_raw([
            0x910d583a1743706f,
            0x66b94d323982b9ac,
            0x6278b6e276667931,
            0xc96e407a7e6499c7,
        ]),
        SecFp::from_raw([
            0x9b07b33009bfb672,
            0x39024d93c82b8f96,
            0x20a5f47f3db72bae,
            0x9f91dbd0974ed335,
        ]),
        SecFp::from_raw([
            0xaf0e8ef0951f6e81,
            0x39024d93c82b8f96,
            0x6c0b186d28c1c94e,
            0x432046e14ca352c2,
        ]),
    ],
    [
        SecFp::from_raw([
            0x3d4387473e555b60,
            0xdad605f0637ef488,
            0x18fff269e779454d,
            0x54633f5e9175aae2,
        ]),
        SecFp::from_raw([
            0x8694ebb3f916566b,
            0xc43eb836e55b79b4,
            0x2509968fa43c2872,
            0x6da3991b096e10b5,
        ]),
        SecFp::from_raw([
            0x2cd5f65f9011e478,
            0xc43eb836e55b79b4,
            0x9f458e504334ae74,
            0x68cda42da84a391b,
        ]),
    ],
    [
        SecFp::from_raw([
            0xffe07d1e17628e4f,
            0x9f6d6241232cf2e6,
            0xc44603654747b562,
            0xcfcc5ea5b020b1c5,
        ]),
        SecFp::from_raw([
            0x11ceb268e42931f7,
            0xdcfe116192c07b27,
            0xdaab2e1c481a1203,
            0x1769dd1c7934ca9e,
        ]),
        SecFp::from_raw([
            0x76d2d4bacd47b66b,
            0xdcfe116192c07b27,
            0x7ce30db97a880f92,
            0x4085640721e80863,
        ]),
    ],
    [
        SecFp::from_raw([
            0x2feb2023e10d1a73,
            0x3fe81feefd9b871c,
            0xe373afd2d7641fe9,
            0x09226e6059900df0,
        ]),
        SecFp::from_raw([
            0xde31499c62d690a4,
            0x732691ffa2c20d55,
            0x171a7d866fd393b6,
            0x5ff0fd13dc382a9e,
        ]),
        SecFp::from_raw([
            0xd34918abc12efe20,
            0x732691ffa2c20d55,
            0xda7adbb55831cde3,
            0xd293c68f70d58c18,
        ]),
    ],
    [
        SecFp::from_raw([
            0x69b5d268c98c61a8,
            0xb1ba32cf9dcaf152,
            0x4c7b35c72903970a,
            0x0d4aca2a26f4ebb2,
        ]),
        SecFp::from_raw([
            0x956b1929778f01e3,
            0x279d2c4d9cbbc764,
            0xfe54486f0a1912f9,
            0x39e32f536f54af39,
        ]),
        SecFp::from_raw([
            0x62a3b84a7b251619,
            0x279d2c4d9cbbc764,
            0xb3cd901096f03d76,
            0x399cde4a3d4fed41,
        ]),
    ],
    [
        SecFp::from_raw([
            0xc9126ab1bf8ea7cf,
            0x64cfc0d4ae9c0ee9,
            0x82c731f4897cf3ae,
            0xed17f6c23e941b39,
        ]),
        SecFp::from_raw([
            0x3f525ffaf56bf4bb,
            0x9403e4ded3350249,
            0xdc9418badeaae35a,
            0xba9ff6a3be47138a,
        ]),
        SecFp::from_raw([
            0xdabc1ad1b188c250,
            0x9403e4ded3350249,
            0x342b555db47c2b42,
            0x73d198e49a5ba537,
        ]),
    ],
    [
        SecFp::from_raw([
            0xc8b8a9027eec859b,
            0x1a439fd276a75842,
            0x15fe9db8712a4f7e,
            0xd3ae59e3c068e2c7,
        ]),
        SecFp::from_raw([
            0x10bec72850f5168a,
            0x4cab23799a5b1ab0,
            0x4785461f0532e532,
            0x323628ee1cc8ffca,
        ]),
        SecFp::from_raw([
            0xcf760eb73656ae37,
            0x4cab23799a5b1ab0,
            0xb79ecdc7592b0809,
            0x257eed339cc8e1be,
        ]),
    ],
    [
        SecFp::from_raw([
            0x3edda3c65ccf7746,
            0xb73dc0169957d31b,
            0x601b6f4eec5523ae,
            0x216b97559558b222,
        ]),
        SecFp::from_raw([
            0x96ab1fae07097325,
            0x9307ae029ef2f81f,
            0x24b27a27735968e9,
            0xee813ade2cf0ae6d,
        ]),
        SecFp::from_raw([
            0xf2f53a7864cd1974,
            0x9307ae029ef2f81f,
            0x9e9fbfa6a814dc24,
            0x61be08729aa65715,
        ]),
    ],
    [
        SecFp::from_raw([
            0x7c015c043538a78f,
            0xec28565b8bd58f21,
            0xcfbd8bd9d0f98baa,
            0xc7a761dcc08b07e3,
        ]),
        SecFp::from_raw([
            0xab4f40071ed57728,
            0xb00194d5f9ccefd2,
            0x64a8be8901d252dc,
            0xe80310e1b7cc5e6f,
        ]),
        SecFp::from_raw([
            0x115249fc32ddd87e,
            0xb00194d5f9ccefd2,
            0xb2cd6d9a4d861a03,
            0xdd2ada833d3ab9f5,
        ]),
    ],
    [
        SecFp::from_raw([
            0xe361e5bc9f60fdc4,
            0xf61244ee819d0884,
            0xeaf3474bb6553877,
            0xe459917758069926,
        ]),
        SecFp::from_raw([
            0x9b3a1e249c585ca9,
            0xcaa5b152a372d6c5,
            0x7d2435f4cf5a8853,
            0xb5ba3eda3af61cec,
        ]),
        SecFp::from_raw([
            0x703627403f2bd298,
            0xcaa5b152a372d6c5,
            0xeb8f4dbbc2c0a414,
            0x1632df1f9f57382a,
        ]),
    ],
    [
        SecFp::from_raw([
            0x11addd157aad8176,
            0xfeeee41985aad1a3,
            0x0a11c1f7410a08ae,
            0x3c5da4efb0671150,
        ]),
        SecFp::from_raw([
            0xf04b4867469be3d4,
            0xa8ff845148fd6a79,
            0xb65b3f9213296b3d,
            0x5e3e288c966b736d,
        ]),
        SecFp::from_raw([
            0x657cea5b71c6885f,
            0xa8ff845148fd6a79,
            0x44da838746b9b90e,
            0xaa2cd4e6f5ea3cf8,
        ]),
    ],
    [
        SecFp::from_raw([
            0x6bf75c6e788504c3,
            0x826372971eca766a,
            0x2b65fece716cd64a,
            0x57fc4260962d37cb,
        ]),
        SecFp::from_raw([
            0xa4b86d65cc7868fa,
            0x5724f74f2b61c22f,
            0xa206e7b2d684b76e,
            0x209966a1b1356993,
        ]),
        SecFp::from_raw([
            0x26760f6984fbeae7,
            0x5724f74f2b61c22f,
            0xef7ef7fd8048ea02,
            0x5fffafb75ae81f1b,
        ]),
    ],
    [
        SecFp::from_raw([
            0x17e0056592256852,
            0xc83d2b0c712b7b4f,
            0xfff32a35d95eef28,
            0xee7d204c3acd6f10,
        ]),
        SecFp::from_raw([
            0xbc911aa8bd6e709b,
            0xafcda98256aebbc3,
            0x5976a0accbcb6c18,
            0x57579227970323b4,
        ]),
        SecFp::from_raw([
            0x09b837b98c358b54,
            0xafcda98256aebbc3,
            0x68d2b1efc9db6a77,
            0x83efab5492e67470,
        ]),
    ],
    [
        SecFp::from_raw([
            0x0cd88a78d02b2ef4,
            0xc05de9baf3a2b475,
            0x63dca583a2a97895,
            0xd0dc6c609006837e,
        ]),
        SecFp::from_raw([
            0xea44f537f8d7be30,
            0xa9ec38876b22108e,
            0x6a47f54e8b92245f,
            0x21430da26ac07a79,
        ]),
        SecFp::from_raw([
            0x429e6d2fa89922f3,
            0xa9ec38876b22108e,
            0x529ef5ed25237fe8,
            0x9479ed819f2af2a1,
        ]),
    ],
    [
        SecFp::from_raw([
            0x337ada2fbc223746,
            0xb1fc92119ffaa7eb,
            0xa4b8deaf3b2967a6,
            0xa13235ec974c16b7,
        ]),
        SecFp::from_raw([
            0xac5e053ae0a493e5,
            0x73ec8170639b6a12,
            0x88359ec4171ee6ec,
            0xe3a8df05fd6de940,
        ]),
        SecFp::from_raw([
            0x19a50b5989f7b7d4,
            0x73ec8170639b6a12,
            0x40ad0671545761c9,
            0x7f17a712ff1602a9,
        ]),
    ],
    [
        SecFp::from_raw([
            0x16b782941f1c11f4,
            0x43d905641f26217a,
            0x73b27d5a1fb6924f,
            0x3319159a8546df85,
        ]),
        SecFp::from_raw([
            0x4c54a334b90fb300,
            0x844aa6f19cfcce19,
            0x703b3c1a27099072,
            0xedee2146127b0d8b,
        ]),
        SecFp::from_raw([
            0xe65fbcd1024dad19,
            0x844aa6f19cfcce19,
            0xd6c3ffd30a697182,
            0xb9a063aaf3e20768,
        ]),
    ],
    [
        SecFp::from_raw([
            0xe0afecf8d9941f15,
            0x99ea647a9c192039,
            0x9b830095057f229f,
            0x5e769953faa47fe0,
        ]),
        SecFp::from_raw([
            0x6926e7de7fe25bcc,
            0x3e48ab13f4df2017,
            0x7c32db5248667815,
            0x57100c1b33e3b1eb,
        ]),
        SecFp::from_raw([
            0x3f758d5b0545d3e9,
            0x3e48ab13f4df2017,
            0xe58ea700e1bb32e1,
            0x9fa1d1667272504f,
        ]),
    ],
    [
        SecFp::from_raw([
            0x7125e7c4c67f6580,
            0x1f959de3e3f70412,
            0x98d8f9eff80b1ccb,
            0x1d9a47898a267dd4,
        ]),
        SecFp::from_raw([
            0xbbfabbd5ef0f09d1,
            0xd5000f9ef105a183,
            0xb8a64f1f131bc69e,
            0xe0f1c7454e6ac737,
        ]),
        SecFp::from_raw([
            0x931f4499fc532729,
            0xd5000f9ef105a183,
            0x7c38295dd7ed80a5,
            0xa9e71885e72950d7,
        ]),
    ],
    [
        SecFp::from_raw([
            0xf62472b2397170af,
            0x0684f62d0056dc3b,
            0x44da703a3c65e996,
            0x77283336565fa122,
        ]),
        SecFp::from_raw([
            0x6001e0132e6050f8,
            0x5b8034069e331dd4,
            0x39db1ca47b3dba94,
            0x36955c8200a957be,
        ]),
        SecFp::from_raw([
            0xb385d0f313275345,
            0x5b8034069e331dd4,
            0x033e379f1658c6e9,
            0x98d7b66399475fd6,
        ]),
    ],
    [
        SecFp::from_raw([
            0x721ce12411ae5f72,
            0xada0e8f5e972437b,
            0xbe76df190b1c2c76,
            0x0653d30f8c1437e5,
        ]),
        SecFp::from_raw([
            0xb37ca20f0142eefc,
            0x495b1fc8db7587c4,
            0xd3e76666ebb86404,
            0x527d7a42ff4b7b4b,
        ]),
        SecFp::from_raw([
            0x0979295a7b355f21,
            0x495b1fc8db7587c4,
            0xe2a2942bf78ff78f,
            0xb916c1e39d644e78,
        ]),
    ],
    [
        SecFp::from_raw([
            0x2cc92c95629f8628,
            0xa820deb6dc230020,
            0x958fe4e836c3b49f,
            0xe51e4cc917c198c0,
        ]),
        SecFp::from_raw([
            0x389a67713058af5d,
            0x0e40667b785542c7,
            0x50c32ee930d82eb9,
            0xbbda6a9943af04ae,
        ]),
        SecFp::from_raw([
            0x3ad44a701c6796a3,
            0x0e40667b785542c7,
            0x2c2cbabd2e16a46e,
            0x5c8ece775f9c331c,
        ]),
    ],
    [
        SecFp::from_raw([
            0x5fe3cc3a5750f351,
            0x9bc50f2af3ef1530,
            0x991468807be52b22,
            0x660bb9527ddf146f,
        ]),
        SecFp::from_raw([
            0x502f61a10c5b01ff,
            0x90c30767337ea912,
            0x52944cddb83d5d7d,
            0x8e6a596c72906f84,
        ]),
        SecFp::from_raw([
            0xf9461588446bbc49,
            0x90c30767337ea912,
            0xad5737f685ac30c5,
            0xd1e788d189864814,
        ]),
    ],
    [
        SecFp::from_raw([
            0x7df799b3029038f0,
            0x38cba443d018f198,
            0x55a1050fb69fa198,
            0xf27e2989dd5724a3,
        ]),
        SecFp::from_raw([
            0xb49a7bd516d9ea3f,
            0x3614c73a09a9cbb3,
            0xe20a95ab9b82a535,
            0x8809e09aa92e55ef,
        ]),
        SecFp::from_raw([
            0x10efb10ec12ddc98,
            0x3614c73a09a9cbb3,
            0x31f93becda9ef3b9,
            0x2758f1cedf6dc152,
        ]),
    ],
    [
        SecFp::from_raw([
            0xe4048193ccaf0b5d,
            0x581b094f38ac4790,
            0x23202eeec66b3b4f,
            0x0522db529ca02ea1,
        ]),
        SecFp::from_raw([
            0xe5b4b9bdc21c04db,
            0x9e63835abd69879e,
            0x18db6881f2f777ac,
            0x259489aab216c081,
        ]),
        SecFp::from_raw([
            0x27ae69b1afc0bd5a,
            0x9e63835abd69879e,
            0xdb99a1ef9c679aa3,
            0xf4991e6461b363f0,
        ]),
    ],
    [
        SecFp::from_raw([
            0xc8dc95b30a529664,
            0xfb6167ad4debba31,
            0x8feae347e5e739f1,
            0x45f35912978af5ee,
        ]),
        SecFp::from_raw([
            0xe7d84c488600abe0,
            0x9fe1977b37d30f78,
            0x499da665701ac0ce,
            0xee6e2a660ac7538c,
        ]),
        SecFp::from_raw([
            0xd04484e04551223f,
            0x9fe1977b37d30f78,
            0xbb3c3bf8352a4749,
            0x663b14b9a4ad467c,
        ]),
    ],
    [
        SecFp::from_raw([
            0x4135d12f27fe8250,
            0x0994dfcdb6651d0d,
            0x4aee748d99afeb3c,
            0x755211bb4f52e9eb,
        ]),
        SecFp::from_raw([
            0xb7c611bb9a4c0455,
            0xecb5800114911d7b,
            0xa58847f2a90a38a3,
            0x256f1aeea44f6d1f,
        ]),
        SecFp::from_raw([
            0x87b7afadedcdc521,
            0xecb5800114911d7b,
            0xbce88c1b3505c3dd,
            0xaf383cb07bedecb9,
        ]),
    ],
    [
        SecFp::from_raw([
            0x7a650712d7114801,
            0x19d8c8ca3382861f,
            0x35639161e83a876c,
            0x5d7329a0fe0ca1f3,
        ]),
        SecFp::from_raw([
            0x409240811df26857,
            0x767792139d03058d,
            0xcae57e5a3a286685,
            0x158e18cebfccecab,
        ]),
        SecFp::from_raw([
            0xff34d1e6f96fe170,
            0x767792139d03058d,
            0x2428de2734095eac,
            0x7f5c2dc6718e3073,
        ]),
    ],
    [
        SecFp::from_raw([
            0x424d2907889e2c6b,
            0x6d41a27855aec39b,
            0x1ad7e65eb2e98122,
            0x7bfd6dcc77a88b97,
        ]),
        SecFp::from_raw([
            0x90b8ace2757529a0,
            0xecc365603e4803b9,
            0x58308a4c3758e60d,
            0x27b370e06752ce47,
        ]),
        SecFp::from_raw([
            0xa337f5bf7e9fa954,
            0xecc365603e4803b9,
            0xcfecb0935ea3efea,
            0xd09bbbbcc7dbbade,
        ]),
    ],
    [
        SecFp::from_raw([
            0xecd5f7da529444c2,
            0xb922cf3fdb25f748,
            0x79120d73badfcfba,
            0x7426adf4d3977ea6,
        ]),
        SecFp::from_raw([
            0x9a77b489038f1f71,
            0xac392033e588377c,
            0x6986b2be66d04083,
            0xee51c38955f32050,
        ]),
        SecFp::from_raw([
            0x46966c2c2377a488,
            0xac392033e588377c,
            0x840f918057190301,
            0xcd9349a4436133f5,
        ]),
    ],
    [
        SecFp::from_raw([
            0xfdf98ce193ede7fa,
            0x31b1f4394f4f93d5,
            0x538ed9a93ed65065,
            0x83e2262631db9974,
        ]),
        SecFp::from_raw([
            0x44a04a315525be94,
            0x0eff145b1ea4fe31,
            0xcf479fc2ab9879de,
            0x6c5c2c97c676e289,
        ]),
        SecFp::from_raw([
            0x11c8c5773e7420fa,
            0x0eff145b1ea4fe31,
            0xa5107679894b11b2,
            0xaeb6e434f4ce8ea0,
        ]),
    ],
    [
        SecFp::from_raw([
            0x540a80dc14a0d0a1,
            0xc2d1aa60b30a86d8,
            0xed8eea4ab1b6dd1a,
            0x0c0572562c2a7606,
        ]),
        SecFp::from_raw([
            0x9716c09c15ae037c,
            0x5c242e7165f58408,
            0x431a4bb77e869279,
            0xde9b285b58d5de28,
        ]),
        SecFp::from_raw([
            0x7794b71d788b5c98,
            0x5c242e7165f58408,
            0x09780cacbd492ff2,
            0xdb0a28aa338f0943,
        ]),
    ],
    [
        SecFp::from_raw([
            0x58d08608d9b6e442,
            0x94462aa0d82ba6a0,
            0xc7b86896e77687cd,
            0x952e8e786a9f45b8,
        ]),
        SecFp::from_raw([
            0xa8cb7c8bd559f9f2,
            0x081fe283d84f80db,
            0xab6657c73bc75b9a,
            0xe465c2bfdade7f7f,
        ]),
        SecFp::from_raw([
            0xff0d2b3437dd9496,
            0x081fe283d84f80db,
            0xe6b06dde21817bdc,
            0xa3cbf907aa46e71f,
        ]),
    ],
    [
        SecFp::from_raw([
            0x7b305b81a12ef110,
            0x71a9d9db37e7ae42,
            0x33b3f96420014bc4,
            0x5a7a30e251c53d5e,
        ]),
        SecFp::from_raw([
            0x3fededa64f9f40be,
            0x68f90b34ea9a2c68,
            0xfe93e07e19bca6de,
            0x481012e0b4210bc9,
        ]),
        SecFp::from_raw([
            0xdd446fa9893d302b,
            0x68f90b34ea9a2c68,
            0x9da9e9a6b15e0de4,
            0xdadabad5b8e62e0c,
        ]),
    ],
    [
        SecFp::from_raw([
            0x85a19fe8445bbdce,
            0x92ab463ec72f216a,
            0x6004394306237cbe,
            0x7b69fffabd274da4,
        ]),
        SecFp::from_raw([
            0x4431280d2f884e4a,
            0xaddf05a6e7c7a5c2,
            0xbc83cf82f666beb9,
            0x08d904df23b3a49a,
        ]),
        SecFp::from_raw([
            0x3cd54e56acca6231,
            0xaddf05a6e7c7a5c2,
            0xc780eee97d1a4594,
            0x42a09538bac6ba33,
        ]),
    ],
    [
        SecFp::from_raw([
            0xe4fad83c2b0e79b0,
            0x7d226ec28ccddeda,
            0x18a030199e6e062a,
            0x4d412751767fff58,
        ]),
        SecFp::from_raw([
            0x9d8b7a3b7fed8678,
            0x1811b30f76261795,
            0x80c5c0632971723d,
            0x3d52c5d9c05129e5,
        ]),
        SecFp::from_raw([
            0xa79a054403220f4e,
            0x1811b30f76261795,
            0x308b97add329b9ae,
            0xd4e390fb6fee7ebe,
        ]),
    ],
    [
        SecFp::from_raw([
            0xdda3c47dea256cb2,
            0x098474831bd8f8b8,
            0x67c397633fba5944,
            0xd6c7716b6f858a64,
        ]),
        SecFp::from_raw([
            0x4ea9173bd803481c,
            0x359d953ca4c14793,
            0x01d0206316ce4bd7,
            0x36ba8fc22e880175,
        ]),
        SecFp::from_raw([
            0x07b4818440bb9242,
            0x359d953ca4c14793,
            0xc4e1fcd03baff048,
            0xa8455ba40430f98e,
        ]),
    ],
    [
        SecFp::from_raw([
            0xb3d013b109e76806,
            0x4915ddd709e3450c,
            0x9477529055cbb1a4,
            0x4aa5cc42f8c1bb19,
        ]),
        SecFp::from_raw([
            0x685f27ce7134baa9,
            0x77ac907b74406310,
            0x6b9d05f186cf3754,
            0xde45eb902f1d3742,
        ]),
        SecFp::from_raw([
            0xa14b3ae22fe97407,
            0x77ac907b74406310,
            0x28551d7cf83b0e20,
            0x7bca58f4c9d30dd8,
        ]),
    ],
    [
        SecFp::from_raw([
            0xf85885e2d6374a70,
            0x6068baffbb66066c,
            0x7e1bff3f59deec74,
            0x48a6a4e9ec176e22,
        ]),
        SecFp::from_raw([
            0xe53a2730da221642,
            0x75069ce88b39714f,
            0x0092a2cc34519d34,
            0x9aecf0d7165b7e90,
        ]),
        SecFp::from_raw([
            0x414a926a711e3762,
            0x75069ce88b39714f,
            0x26dcd5ccb13b7d96,
            0x662c29da05dc3b95,
        ]),
    ],
    [
        SecFp::from_raw([
            0xcfa74ebf2ee77d0d,
            0x15ad23f052e62326,
            0x334ac7a1dd9ff118,
            0xaa3ccf13155dfe91,
        ]),
        SecFp::from_raw([
            0x5e4a2709de6cf8f2,
            0x85abd6990be0a99c,
            0x71255491bbc83422,
            0x3004e2d1fbeeffd2,
        ]),
        SecFp::from_raw([
            0xf0bc8d1030f775bd,
            0x85abd6990be0a99c,
            0x597aebdfbd3d21c6,
            0x40940f8c1cd58ae6,
        ]),
    ],
    [
        SecFp::from_raw([
            0xf47b078e3b6e6fc4,
            0x442780e053c27b38,
            0x36deb76535d7091c,
            0x05683dbea800c340,
        ]),
        SecFp::from_raw([
            0x22f793b7bf652008,
            0x98d96ecc63ca139f,
            0x362dc3c3c4451e4f,
            0xc0018dd1e4c4188c,
        ]),
        SecFp::from_raw([
            0x6348685206a487b7,
            0x98d96ecc63ca139f,
            0x5a77b04f8bdaa9a3,
            0x5f1fa64cfd22bea7,
        ]),
    ],
    [
        SecFp::from_raw([
            0x01c0d72e5f0f6194,
            0x3cb39b2e286d6f4a,
            0xd87df6a9d2b8dc99,
            0xf0e3da061d35a4cd,
        ]),
        SecFp::from_raw([
            0x39bfba8b851e7d87,
            0x9e9b14e338c06855,
            0x32ab6b63d865530a,
            0xb978ee99c25ff418,
        ]),
        SecFp::from_raw([
            0x995b7fbac35ad5ef,
            0x9e9b14e338c06855,
            0xb76bd44ec783ed82,
            0x5e056fc78c2aebde,
        ]),
    ],
    [
        SecFp::from_raw([
            0xa66282488a51b916,
            0xc6208099e836fef0,
            0x06f4c411887c49c0,
            0xfea5ae0b8ae64758,
        ]),
        SecFp::from_raw([
            0x7dfa8d0013afd584,
            0x7466be6817ae67fb,
            0xdb34984d5e4cf8bb,
            0x4011df0dcf4ef82d,
        ]),
        SecFp::from_raw([
            0xeb24722f1b8a5f22,
            0x7466be6817ae67fb,
            0x6a52f93cd305707a,
            0x971d78d355eac2a5,
        ]),
    ],
    [
        SecFp::from_raw([
            0x9bc9ec0812730e29,
            0xc899b8be8ac065ee,
            0x8b45d207ea21a0fe,
            0x888ab55ab647d135,
        ]),
        SecFp::from_raw([
            0xfbc446c9cfd0686d,
            0x9c1e30cde524933b,
            0x5965882063d03e2e,
            0xb272dbea28c3cd8d,
        ]),
        SecFp::from_raw([
            0x9465acb3e1d3871b,
            0x9c1e30cde524933b,
            0xd4b55777860810fb,
            0x4910d1d2c2872a5c,
        ]),
    ],
    [
        SecFp::from_raw([
            0x5170e9a8d870a2a4,
            0x2505c5787dc27186,
            0x787177f7eb2126cd,
            0x700d1f3b78d6a013,
        ]),
        SecFp::from_raw([
            0x7f2ce3dc0c837c6d,
            0x307d64819009a045,
            0x60fb581719536705,
            0x09716bd482c01f1e,
        ]),
        SecFp::from_raw([
            0x353be238841a20f6,
            0x307d64819009a045,
            0xd711b354fa2404bc,
            0xdffdb7e489adf91d,
        ]),
    ],
    [
        SecFp::from_raw([
            0x91f81d22a549ed5d,
            0x65e67cb3322a4c08,
            0xa9459e12a4ab36f0,
            0x7ad8f5663f7305de,
        ]),
        SecFp::from_raw([
            0xeacda6e99d6f0be1,
            0xd243b5ae2e3e7391,
            0x28b860af4c057beb,
            0x90efd0590e408127,
        ]),
        SecFp::from_raw([
            0xf0afcfccd85c4905,
            0xd243b5ae2e3e7391,
            0x272a8afa65cf64eb,
            0x93db57bc26e0644d,
        ]),
    ],
    [
        SecFp::from_raw([
            0x2477883051e01c83,
            0x87ea5eb3f4361dc6,
            0xb39892ad2a3291ea,
            0x4374ab7ac7a70caf,
        ]),
        SecFp::from_raw([
            0xdec825db6ae2a22f,
            0x74f000c555f2f433,
            0x1748dc0fd32a5668,
            0xd40595e3ca026021,
        ]),
        SecFp::from_raw([
            0xcb49801730977315,
            0x74f000c555f2f433,
            0x1d3200b73365af36,
            0x53bf4a2ba34ec788,
        ]),
    ],
    [
        SecFp::from_raw([
            0x7744e24ca6acc401,
            0x8df8fe077d4de32b,
            0x46c5ab20b4a3a035,
            0xb1edc902961b17f5,
        ]),
        SecFp::from_raw([
            0x4717cf66067337b1,
            0xbe9178726c0e3f1a,
            0x2d78857ecec18053,
            0x1d7665b172ef246a,
        ]),
        SecFp::from_raw([
            0xb6ec49e6b0afd008,
            0xbe9178726c0e3f1a,
            0xdac092e4cc344409,
            0x237a5717c098fbf1,
        ]),
    ],
    [
        SecFp::from_raw([
            0x643fda6ee7e50071,
            0x22b768a423e41a50,
            0x86e6b60d7fd8e607,
            0x9e1333925b735445,
        ]),
        SecFp::from_raw([
            0x423edbde2f1783ce,
            0x10241ec5550166b7,
            0x31dcf4a26b975801,
            0x3edfe4ac56205a97,
        ]),
        SecFp::from_raw([
            0x3a1736f3e2d02f1b,
            0x10241ec5550166b7,
            0x766f3e12d9a67d84,
            0x7bf46946a106bffc,
        ]),
    ],
];
// Secure MDS: 0
// n: 255
// t: 3
// N: 765
// Result Algorithm 1:
//  [True, 0]
// Result Algorithm 2:
//  [True, None]
// Result Algorithm 3:
//  [True, None]
// Prime number: 0x40000000000000000000000000000000224698fc094cf91b992d30ed00000001
// MDS matrix:
pub(crate) const MDS: [[SecFp; 3]; 3] = [
    [
        SecFp::from_raw([
            0x48f1df34f2861001,
            0xfafaff8d8c26ed07,
            0xfa348898bb4f6dfb,
            0x5ffeefc27b17af70,
        ]),
        SecFp::from_raw([
            0xe0f763263098c9de,
            0x450fd711ec77419e,
            0x500d25b304a74ec8,
            0xc43a210c45bce293,
        ]),
        SecFp::from_raw([
            0xfba753e3d7412b56,
            0x450fd711ec77419e,
            0x26cd5ee07a888435,
            0x77155019b73b1bf1,
        ]),
    ],
    [
        SecFp::from_raw([
            0xa29ef7aa7d1b4831,
            0x12bdbef49a1ab8e1,
            0x29b47f3cad13f066,
            0xc3336cb20b4884fb,
        ]),
        SecFp::from_raw([
            0x269f2de074301d24,
            0x15381a5aed8f0e33,
            0x17eb7131afe0f66d,
            0x8148016b599973a2,
        ]),
        SecFp::from_raw([
            0xd00a32ff4ff80ecf,
            0x15381a5aed8f0e33,
            0x52c3698c75ab850b,
            0x5f15f5cf88f884dc,
        ]),
    ],
    [
        SecFp::from_raw([
            0x53a88d36b2643dce,
            0x94659408adaa490d,
            0x858decd16f8cd9f3,
            0x9661531f738e1cac,
        ]),
        SecFp::from_raw([
            0x69267dc2b66a4e4b,
            0x2c1fb2997efa9ecc,
            0xf8cf878ef201ab85,
            0x07fb4579eefc609b,
        ]),
        SecFp::from_raw([
            0xd71d281c90c56d0c,
            0x2c1fb2997efa9ecc,
            0x005d3789601645b2,
            0x8cb4b6a908bbf668,
        ]),
    ],
];

pub(crate) const MDS_INV: [[SecFp; 3]; 3] = [
    [
        SecFp::from_raw([
            0xd39186e8c0398777,
            0xde9eccad954263fb,
            0xa70ce4f2f48ec4f9,
            0x57d4162cb5e23639,
        ]),
        SecFp::from_raw([
            0x04fc29cd210f64e7,
            0xd75456e37b7c27c6,
            0x6f7f686ac9247a9f,
            0x28aa17a1e958fab9,
        ]),
        SecFp::from_raw([
            0x05c58eef0bad000b,
            0xd75456e37b7c27c6,
            0x9546e1dc225eb9bc,
            0xdf24d5bd4438f742,
        ]),
    ],
    [
        SecFp::from_raw([
            0x3517386e101e35ac,
            0xe86e72f9b111a1ef,
            0x7110a21c7f4ea902,
            0x807ad3b8eb103159,
        ]),
        SecFp::from_raw([
            0xe1051be5e7e87b5d,
            0x9539127e0120b041,
            0x8989732fb524babb,
            0x3905e360ac867d6b,
        ]),
        SecFp::from_raw([
            0xf09c2ca7d7be5e8c,
            0x9539127e0120b041,
            0x3944db3b36aa583b,
            0xf6397239ee90b820,
        ]),
    ],
    [
        SecFp::from_raw([
            0x6955cfa76b26c1ed,
            0xf14a5cde98254955,
            0xba15df9a0a49763e,
            0x78129348227a7cbd,
        ]),
        SecFp::from_raw([
            0xa3b1b933b14ea55f,
            0x119d0561166fb62b,
            0xb4c6d8ce3cf13b9b,
            0x827f0fcd5fd51672,
        ]),
        SecFp::from_raw([
            0x562727e8cf121dc4,
            0x119d0561166fb62b,
            0xab7e6cea7ec81c25,
            0xad54b2fc8a52f487,
        ]),
    ],
];
