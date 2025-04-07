
#[path = "../configs/envconfig.rs"]
mod envconfig;
#[path = "./interfaceutils.rs"]
mod interfaceutils;

use envconfig::{CHAINID_MAP, ENV};
use interfaceutils::AssetInfo;

use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::sleep;

use std::collections::HashMap;

pub async fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

pub static SUPPORTED_PERIODS: Lazy<Vec<&str>> =
    Lazy::new(|| vec!["1m", "5m", "15m", "1h", "4h", "1d"]);

pub static PERIOD_MAP: Lazy<HashMap<&'static str, u64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("1m", 60);
    map.insert("5m", 5 * 60);
    map.insert("15m", 15 * 60);
    map.insert("1h", 60 * 60);
    map.insert("4h", 4 * 60 * 60);
    map.insert("1d", 24 * 60 * 60);
    map
});

pub static SUPPORTED_TOKENS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "WBTC", "WETH", "USDC", "BNB", "SOL", "XRP", "TON", "DOGE", "ADA", "TRX", "SHIB", "AVAX",
        "DOT", "LINK", "WSTETH", "BTC", "GBP", "AUD", "EUR", "NZD", "DADDY", "ORDI", "IMX", "SAND",
        "FOKI", "STX", "MERL", "CLT", "DEN", "MIA", "TPA", "MIAB", "NYC", "LAX", "SAN", "SOLB",
        "SFO", "LAS", "PIT", "PHL", "AUS", "DFW", "IAH", "ATL", "SEA", "PHX", "CHI", "BOS", "PDX",
        "WDC", "BKN", "USA", "PARIS", "LCY", "CHIR", "DENR", "USDR", "XAU", "XAG", "XPD", "XPT",
        "NG", "XBR", "ZW", "RC", "US1YTX", "US5YTX", "AU5YTRR", "GB5YTRR", "IN5YTRR",
    ]
});

pub static SUPPORTED_TOKENS: Lazy<HashMap<String, Vec<&str>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(   
            "bitlayer_testnet".to_string(),
            SUPPORTED_TOKENS_BITLAYER_TESTNET.clone(),
        );
        map
    });

pub const PRICE_FETCH_INTERVAL: u32 = 5000;

pub static TOKEN_INFO: Lazy<HashMap<String, HashMap<String, AssetInfo>>> = Lazy::new(|| {
    let mut networks = HashMap::new();

    //Bitlayer Testnet
    let mut bitlayer_testnet = HashMap::new();

    //Bitlayer Testnet
    bitlayer_testnet.insert(
        "WBTC".to_string(),
        AssetInfo {
            token_address: "0x313ea66A1f508B5F2825A626F7a09afeaBE594E1".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "WETH".to_string(),
        AssetInfo {
            token_address: "0xbB506faEA96E0329F5Bb5552182DB0b245413A2A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "USDC".to_string(),
        AssetInfo {
            token_address: "0xAc53dF4Adf5edBE466fAF2242277E238E52D466B".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BNB".to_string(),
        AssetInfo {
            token_address: "0x4a43089c6620b75ed2C965AB6d41c62E80E2954B".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SOL".to_string(),
        AssetInfo {
            token_address: "0x3E0911d6C0fa146C3006a2bc186c815fcF566243".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XRP".to_string(),
        AssetInfo {
            token_address: "0xe44e93ce209D44B32263e9487Bb4cDe6507Df44F".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "TON".to_string(),
        AssetInfo {
            token_address: "0x5775376a92236BDCB5018655cc12C752c00Fe0b2".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DOGE".to_string(),
        AssetInfo {
            token_address: "0xEdf90028e93aD936cC2329985e61E1a3C5Dcf0E4".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "ADA".to_string(),
        AssetInfo {
            token_address: "0x2c77158726951b5558fe0EbE314093375270c60f".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "TRX".to_string(),
        AssetInfo {
            token_address: "0xf716602066127E0a34Af03fc19531F0f6318c21A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SHIB".to_string(),
        AssetInfo {
            token_address: "0xf51201e9BaEBBD71fE94DDAc3d4d9fF287667C60".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "AVAX".to_string(),
        AssetInfo {
            token_address: "0xfc73a37Ae52679e7cDb6554B0bE5d0E3100b03D2".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DOT".to_string(),
        AssetInfo {
            token_address: "0x363E6Ba86D5a723C0C7d7fD9B360091293bc0019".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LINK".to_string(),
        AssetInfo {
            token_address: "0x1D43079cb02ffeF63b3e44fdc89dc7B485189711".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "WSTETH".to_string(),
        AssetInfo {
            token_address: "0xc060E5567B4F15473b646Cb5b3bbb977eAdA7BFD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BTC".to_string(),
        AssetInfo {
            token_address: "0x0000000000000000000000000000000000000000".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "GBP".to_string(),
        AssetInfo {
            token_address: "0x67E4ce304Bfa9A553d400Af72499c887f7c63858".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "AUD".to_string(),
        AssetInfo {
            token_address: "0xEDFB276A1df4f41FbB4E850e2B5ea92269a83d1E".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "EUR".to_string(),
        AssetInfo {
            token_address: "0x46e3F674d0E9ED964d71355175c511B2FC8B5E6F".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "NZD".to_string(),
        AssetInfo {
            token_address: "0x985CA067d5B4ab4a627658D5fC6c6f181978C24a".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "DADDY".to_string(),
        AssetInfo {
            token_address: "0x4Da72d32492c472C5EC0e2f066F570b227662D37".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "ORDI".to_string(),
        AssetInfo {
            token_address: "0xB7c1b5C1F1dcBa1c87EE0112d61e610F9696bbD7".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "IMX".to_string(),
        AssetInfo {
            token_address: "0x65efc0e6d3c468CeC23550e9C8c8C6F8DEfb7182".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "SAND".to_string(),
        AssetInfo {
            token_address: "0x02111877265f31056b9F3CC07036AD6b9C45F315".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "FOKI".to_string(),
        AssetInfo {
            token_address: "0x3a75478554Ee965bb17f004e4b6ACCc325DE7b5A".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "STX".to_string(),
        AssetInfo {
            token_address: "0xdE60936B0999aB2B0E92311Af12558ae89D675e5".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "MERL".to_string(),
        AssetInfo {
            token_address: "0xA411E9dDF2cc927bf61Af1f738ff4402c2f83c15".to_string(),
            token_decimals: 18,
        },
    );
    bitlayer_testnet.insert(
        "CLT".to_string(),
        AssetInfo {
            token_address: "0x595D59862c153e7885FD5Fb75eE7aDDf5f8768EB".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DEN".to_string(),
        AssetInfo {
            token_address: "0xB887E13ee67e63b9AFcC11136583E3e792C49003".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "MIA".to_string(),
        AssetInfo {
            token_address: "0xc9CB6cC9637c08509002EDBeAB0330CDD4A9D333".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "TPA".to_string(),
        AssetInfo {
            token_address: "0x253bd01F34cD1b5CdAEd6F74847eD9D13DD301fE".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "MIAB".to_string(),
        AssetInfo {
            token_address: "0xD77aDF1F36BB0422665eD11B1E09BD5CBe052004".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "NYC".to_string(),
        AssetInfo {
            token_address: "0xc9Be08D31324B7F19b9f2B4814693372F49D0adD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LAX".to_string(),
        AssetInfo {
            token_address: "0xb775Bc8970fDD0b140c619b27a3db6E3fefea25A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SAN".to_string(),
        AssetInfo {
            token_address: "0x98dD36702A96FB835c4b7Dc17D88881c54eDa703".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SOLB".to_string(),
        AssetInfo {
            token_address: "0xC6767F79E9d1AC6624344E2Bb4fbE74e37F73f93".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SFO".to_string(),
        AssetInfo {
            token_address: "0xF99319EDf73D2E3589225336EB93a812d42BD321".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LAS".to_string(),
        AssetInfo {
            token_address: "0xd9312cd0294b15daC2b1baEC59E9A36329dB308a".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PIT".to_string(),
        AssetInfo {
            token_address: "0x0e60C3462a74613a9A130fB65859dd4582D27629".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PHL".to_string(),
        AssetInfo {
            token_address: "0x029E230d0a9BfdfB90d688c951d7A315351E613b".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "AUS".to_string(),
        AssetInfo {
            token_address: "0xCACdF34c7acF35bc003be751Aed4D06143Bab4c8".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DFW".to_string(),
        AssetInfo {
            token_address: "0xe0B572D890b7E4c32CF121EFb6f917Fb4c3C0CEe".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "IAH".to_string(),
        AssetInfo {
            token_address: "0x9DFd7D76be1BF7d01201d87338dc41C4ea481550".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "ATL".to_string(),
        AssetInfo {
            token_address: "0xf82AFB93331DC5Ce40C959Bd7440cbb69785E534".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "SEA".to_string(),
        AssetInfo {
            token_address: "0x8f80cdF35C50Be0Aa27C540baEd35CEE25c49F95".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PHX".to_string(),
        AssetInfo {
            token_address: "0x0489E7605186fDAc0ec80Bf99087EAA8e0382a40".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "CHI".to_string(),
        AssetInfo {
            token_address: "0x83246B9DC962605Cfb65cb64161c691B37B67065".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BOS".to_string(),
        AssetInfo {
            token_address: "0x67f17cca1337C4bFaa844139f934908ecf984422".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PDX".to_string(),
        AssetInfo {
            token_address: "0xCbD0bcE5FE20f5683C84CbA45E645e6F2ffea5cC".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "WDC".to_string(),
        AssetInfo {
            token_address: "0x5D7Ee3e6a2465780B2a6cB0970E0Ca85D1063530".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "BKN".to_string(),
        AssetInfo {
            token_address: "0x2bc801c518931fC8B5f540d58694DC5402Ff5eAF".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "USA".to_string(),
        AssetInfo {
            token_address: "0x99e95495CBB9ddc59E1538E14F60eCAC1aAc45aD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "PARIS".to_string(),
        AssetInfo {
            token_address: "0x6aa8143ADa3A3A1273da05742Ecf5DCcC911900D".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "LCY".to_string(),
        AssetInfo {
            token_address: "0xC45e45E9e525A380047adC22794Bd79E43095D84".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "CHIR".to_string(),
        AssetInfo {
            token_address: "0x6c1061Db0FCE9435736718f3371A75da8b7d3E5E".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "DENR".to_string(),
        AssetInfo {
            token_address: "0xEC141D6C7C4010846417978D8f7D4674Ec400325".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "USDR".to_string(),
        AssetInfo {
            token_address: "0x8054112cB526CF10a8f1c97977D6572a39Fe5eAE".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XAU".to_string(),
        AssetInfo {
            token_address: "0xaF2f1f3173EEC6d9530A3C2cBAA26799571E1a22".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XAG".to_string(),
        AssetInfo {
            token_address: "0x578a1EC389D490953a38AE77D1f2afcBE306d4BF".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XPD".to_string(),
        AssetInfo {
            token_address: "0xE75d46a10D91Af64Dd7978E0768d972F93105cC9".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XPT".to_string(),
        AssetInfo {
            token_address: "0xE89Cf36052d112BC022C01f3aC2b2D3BF641572f".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "NG".to_string(),
        AssetInfo {
            token_address: "0x7859b0eE14cDcfEB75B131D80BBF949b6db26640".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "XBR".to_string(),
        AssetInfo {
            token_address: "0x9E0BCa368e2a4f45f94c1631762Af051EB08ebcF".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "ZW".to_string(),
        AssetInfo {
            token_address: "0x6Db75b0B23abC9bC180E97BB1cDd6c11287CC2fD".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "RC".to_string(),
        AssetInfo {
            token_address: "0xF7864154A0f619818cB0602BbaF076c7AAf09AA3".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "US1YTX".to_string(),
        AssetInfo {
            token_address: "0x43eCd0A6C256BDfb93E13364b5f56aDDB5E71305".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "US5YTX".to_string(),
        AssetInfo {
            token_address: "0x7c4a61b22cd1b4f9a8CAA4ACFad541b793796A2A".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "AU5YTRR".to_string(),
        AssetInfo {
            token_address: "0x202Fc380Af3f773a2ff70CDeAc5A23048BE0514c".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "GB5YTRR".to_string(),
        AssetInfo {
            token_address: "0x777234DC4Efd2587dD277EA8838DbaE62E4c339b".to_string(),
            token_decimals: 12,
        },
    );
    bitlayer_testnet.insert(
        "IN5YTRR".to_string(),
        AssetInfo {
            token_address: "0x9563Bf94a640dd9c8c6C6563e3e3a034184AC673".to_string(),
            token_decimals: 12,
        },
    );

    //INSERTING INTO NETWORK
    networks.insert("bitlayer_testnet".to_string(), bitlayer_testnet);

    networks
});

pub fn get_token_addresses() -> HashMap<String, String> {
    let mut TOKEN_ADDRESSES = HashMap::new();
    if let Some(TOKEN_INFO_CHAIN) = TOKEN_INFO.get(&ENV.NETWORK) {
        for (tokensymbol, assetinfo) in TOKEN_INFO_CHAIN.iter() {
            TOKEN_ADDRESSES.insert(tokensymbol.to_string(), assetinfo.token_address.clone());
        }
    }

    return TOKEN_ADDRESSES;
}

pub fn get_token_decimals() -> HashMap<String, u64> {
    let mut TOKEN_ADDRESSES = HashMap::new();
    if let Some(TOKEN_INFO_CHAIN) = TOKEN_INFO.get(&ENV.NETWORK) {
        for (tokensymbol, assetinfo) in TOKEN_INFO_CHAIN.iter() {
            TOKEN_ADDRESSES.insert(tokensymbol.to_string(), assetinfo.token_decimals.clone());
        }
    }
    return TOKEN_ADDRESSES;
}

#[derive(Clone)]
pub struct TokenKeyAddress {
    pub key: String,
    pub address: String,
}

pub static TOKENS_MAPPINGS: Lazy<HashMap<String, Vec<TokenKeyAddress>>> = Lazy::new(|| {
    let token_addresses: HashMap<String, String> = get_token_addresses();
    let mut network: HashMap<String, Vec<TokenKeyAddress>> = HashMap::new();
    let mut token_vec: Vec<TokenKeyAddress> = Vec::new();

    for (token_symbol, token_address) in token_addresses.iter() {
        let entry = TokenKeyAddress {
            key: token_symbol.clone(),
            address: token_address.clone(),
        };
        token_vec.push(entry);
    }
    let network_key = match ENV.NETWORK.as_str() {
        "bitlayer_testnet" => "bitlayer_testnet",
        _ => panic!("Network not found"),
    };

    network.insert(network_key.to_string(), token_vec);

    network
});

pub static SYMBOL_TO_ADDRESS_MAPPING: Lazy<HashMap<String, String>> =
    Lazy::new(|| get_token_addresses());

pub static SYMBOL_TO_DECIMAL_MAPPING: Lazy<HashMap<String, u64>> =
    Lazy::new(|| get_token_decimals());

pub static PYTH_ID_TO_TOKEN_MAPPING_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
            "ETH",
        );
        map.insert(
            "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
            "USDC",
        );
        map.insert(
            "2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f",
            "BNB",
        );
        map.insert(
            "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
            "SOL",
        );
        map.insert(
            "ec5d399846a9209f3fe5881d70aae9268c94339ff9817e8d18ff19fa05eea1c8",
            "XRP",
        );
        map.insert(
            "8963217838ab4cf5cadc172203c1f0b763fbaa45f346d8ee50ba994bbcac3026",
            "TON",
        );
        map.insert(
            "2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d",
            "ADA",
        );
        map.insert(
            "67aed5a24fdad045475e7195c98a98aea119c763f272d4523f5bac93a4f33c2b",
            "TRX",
        );
        map.insert(
            "f0d57deca57b3da2fe63a493f4c25925fdfd8edf834b20f93e1f84dbd1504d4a",
            "SHIB",
        );
        map.insert(
            "93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7",
            "AVAX",
        );
        map.insert(
            "ca3eed9b267293f6595901c734c7525ce8ef49adafe8284606ceb307afa2ca5b",
            "DOT",
        );
        map.insert(
            "8ac0c70fff57e9aefdf5edf44b51d62c2d433653cbb2cf5cc06bb115af04d221",
            "LINK",
        );
        map.insert(
            "6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784",
            "WSTETH",
        );
        map.insert(
            "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
            "BTC",
        );
        map
    });

pub static PYTH_ID_TO_TOKEN_MAPPING: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &PYTH_ID_TO_TOKEN_MAPPING_BITLAYER_TESTNET,
    );
    map
});


pub static PYTH_ID_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
        "eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a",
        "2f95862b045670cd22bee3114c39763a4a08beeb663b145d283c31d7d1101c4f",
        "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
        "ec5d399846a9209f3fe5881d70aae9268c94339ff9817e8d18ff19fa05eea1c8",
        "8963217838ab4cf5cadc172203c1f0b763fbaa45f346d8ee50ba994bbcac3026",
        "2a01deaec9e51a579277b34b122399984d0bbf57e2458a7e42fecd2829867a0d",
        "67aed5a24fdad045475e7195c98a98aea119c763f272d4523f5bac93a4f33c2b",
        "f0d57deca57b3da2fe63a493f4c25925fdfd8edf834b20f93e1f84dbd1504d4a",
        "93da3352f9f1d105fdfe4971cfa80e9dd777bfc5d0f683ebb6e1294b92137bb7",
        "ca3eed9b267293f6595901c734c7525ce8ef49adafe8284606ceb307afa2ca5b",
        "8ac0c70fff57e9aefdf5edf44b51d62c2d433653cbb2cf5cc06bb115af04d221",
        "6df640f3b8963d8f8358f791f352b8364513f6ab1cca5ed3f1f7b5448980e784",
        "e62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
    ]
});

pub static PYTH_ID: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("bitlayer_testnet".to_string(), &PYTH_ID_BITLAYER_TESTNET);
    map
});

pub static BINANCE_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "BNBUSDT", "ADAUSDT", "XRPUSDT", "TRXUSDT", "USDCUSDT", "LINKUSDT", "DOGEUSDT", "SOLUSDT",
        "DOTUSDT", "AVAXUSDT", "SHIBUSDT", "BTCUSDT", "ETHUSDT",
    ]
});

pub static BINANCE_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "bitlayer_testnet".to_string(),
            &BINANCE_KEYS_BITLAYER_TESTNET,
        );
        map
    });

pub static SYMBOL_MAP_BINANCE_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BNBUSDT", "BNB");
        map.insert("ADAUSDT", "ADA");
        map.insert("XRPUSDT", "XRP");
        map.insert("TRXUSDT", "TRX");
        map.insert("USDCUSDT", "USDC");
        map.insert("LINKUSDT", "LINK");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("SOLUSDT", "SOL");
        map.insert("DOTUSDT", "DOT");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map
    });

pub static BINANCE_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_BINANCE_BITLAYER_TESTNET,
    );
    map
});

pub static KUCOIN_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "TRX-USDT",
        "TON-USDT",
        "DOGE-USDT",
        "USDC-USDT",
        "DOT-USDT",
        "ADA-USDT",
        "AVAX-USDT",
        "SOL-USDT",
        "XRP-USDT",
        "BNB-USDT",
        "SHIB-USDT",
        "LINK-USDT",
        "BTC-USDT",
        "ETH-USDT",
    ]
});

pub static KUCOIN_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &KUCOIN_KEYS_BITLAYER_TESTNET,
    );
    map
});

// KUCOIN SYMBOL MAP
pub static SYMBOL_MAP_KUCOIN_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("TRX-USDT", "TRX");
        map.insert("TON-USDT", "TON");
        map.insert("DOGE-USDT", "DOGE");
        map.insert("USDC-USDT", "USDC");
        map.insert("DOT-USDT", "DOT");
        map.insert("ADA-USDT", "ADA");
        map.insert("AVAX-USDT", "AVAX");
        map.insert("SOL-USDT", "SOL");
        map.insert("XRP-USDT", "XRP");
        map.insert("BNB-USDT", "BNB");
        map.insert("SHIB-USDT", "SHIB");
        map.insert("LINK-USDT", "LINK");
        map.insert("BTC-USDT", "BTC");
        map.insert("ETH-USDT", "ETH");
        map
    });

pub static KUCOIN_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_KUCOIN_BITLAYER_TESTNET,
    );
    map
});

// MEXC KEYS
pub static MEXC_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SOLUSDT", "AVAXUSDT", "ADAUSDT", "LINKUSDT", "TRXUSDT", "DOGEUSDT", "XRPUSDT", "DOTUSDT",
        "BNBUSDT", "TONUSDT", "USDCUSDT", "SHIBUSDT", "BTCUSDT", "ETHUSDT",
    ]
});
pub static MEXC_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("bitlayer_testnet".to_string(), &MEXC_KEYS_BITLAYER_TESTNET);
    map
});

// SYMBOL MAP MEXC

pub static SYMBOL_MAP_MEXC_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("SOLUSDT", "SOL");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("ADAUSDT", "ADA");
        map.insert("LINKUSDT", "LINK");
        map.insert("TRXUSDT", "TRX");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("XRPUSDT", "XRP");
        map.insert("DOTUSDT", "DOT");
        map.insert("BNBUSDT", "BNB");
        map.insert("TONUSDT", "TON");
        map.insert("USDCUSDT", "USDC");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map
    });

pub static MEXC_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_MEXC_BITLAYER_TESTNET,
    );
    map
});

pub static GATE_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SOL_USDT",
        "TRX_USDT",
        "SHIB_USDT",
        "DOT_USDT",
        "ADA_USDT",
        "TON_USDT",
        "XRP_USDT",
        "BNB_USDT",
        "DOGE_USDT",
        "LINK_USDT",
        "USDC_USDT",
        "AVAX_USDT",
        "BTC_USDT",
        "ETH_USDT",
    ]
});

pub static GATE_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("bitlayer_testnet".to_string(), &GATE_KEYS_BITLAYER_TESTNET);
    map
});

// GATE SYMBOL MAP
pub static SYMBOL_MAP_GATE_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();

        map.insert("SOL_USDT", "SOL");
        map.insert("TRX_USDT", "TRX");
        map.insert("SHIB_USDT", "SHIB");
        map.insert("DOT_USDT", "DOT");
        map.insert("ADA_USDT", "ADA");
        map.insert("TON_USDT", "TON");
        map.insert("XRP_USDT", "XRP");
        map.insert("BNB_USDT", "BNB");
        map.insert("DOGE_USDT", "DOGE");
        map.insert("LINK_USDT", "LINK");
        map.insert("USDC_USDT", "USDC");
        map.insert("AVAX_USDT", "AVAX");
        map.insert("BTC_USDT", "BTC");
        map.insert("ETH_USDT", "ETH");

        map
    });

pub static GATE_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_GATE_BITLAYER_TESTNET,
    );
    map
});

pub static BYBIT_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ETHUSDT", "USDCUSDT", "BNBUSDT", "SOLUSDT", "XRPUSDT", "TONUSDT", "DOGEUSDT", "ADAUSDT",
        "TRXUSDT", "SHIBUSDT", "AVAXUSDT", "DOTUSDT", "LINKUSDT", "BTCUSDT",
    ]
});

pub static BYBIT_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("bitlayer_testnet".to_string(), &BYBIT_KEYS_BITLAYER_TESTNET);
    map
});


pub static SYMBOL_MAP_BYBIT_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("LINKUSDT", "LINK");
        map.insert("BNBUSDT", "BNB");
        map.insert("USDCUSDT", "USDC");
        map.insert("XRPUSDT", "XRP");
        map.insert("DOGEUSDT", "DOGE");
        map.insert("DOTUSDT", "DOT");
        map.insert("BTCUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("SOLUSDT", "SOL");
        map.insert("TONUSDT", "TON");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("ADAUSDT", "ADA");
        map.insert("TRXUSDT", "TRX");
        map.insert("SHIBUSDT", "SHIB");
        map
    });

pub static BYBIT_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_BYBIT_BITLAYER_TESTNET,
    );
    map
});

pub static OKX_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        // "WBTC-USD",
        // "WETH-USD",
        "USDC-USD", "BNB-USD", "SOL-USD", "XRP-USD", "TON-USD", "DOGE-USD", "ADA-USD", "TRX-USD",
        "SHIB-USD", "AVAX-USD", "DOT-USD", "LINK-USD", // "WSTETH-USD",
        "BTC-USD", "ETH-USD",
    ]
});


pub static OKX_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("bitlayer_testnet".to_string(), &OKX_KEYS_BITLAYER_TESTNET);
    map
});

pub static SYMBOL_MAP_OKX_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("XRP-USD", "XRP");
        map.insert("BNB-USD", "BNB");
        map.insert("LINK-USD", "LINK");
        map.insert("BTC-USD", "BTC");
        map.insert("ETH-USD", "ETH");
        map.insert("ADA-USD", "ADA");
        map.insert("AVAX-USD", "AVAX");
        map.insert("SOL-USD", "SOL");
        map.insert("SHIB-USD", "SHIB");
        map.insert("DOT-USD", "DOT");
        map.insert("TRX-USD", "TRX");
        map.insert("USDC-USD", "USDC");
        map.insert("TON-USD", "TON");
        map.insert("DOGE-USD", "DOGE");
        map
    });

pub static OKX_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_OKX_BITLAYER_TESTNET,
    );
    map
});

pub static KRAKEN_KEYS_BITLAYER_TESTNET: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "USDCUSDT", "SOLUSDT", "XRPUSDT", "ADAUSDT", "SHIBUSDT", "AVAXUSDT", "DOTUSDT", "LINKUSDT",
        "XBTUSDT", "ETHUSDT", "TRXUSD",
    ]
});

pub static KRAKEN_KEYS: Lazy<HashMap<String, &'static Lazy<Vec<&'static str>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &KRAKEN_KEYS_BITLAYER_TESTNET,
    );
    map
});

pub static SYMBOL_MAP_KRAKEN_BITLAYER_TESTNET: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("XBTUSDT", "BTC");
        map.insert("ETHUSDT", "ETH");
        map.insert("ADAUSDT", "ADA");
        map.insert("AVAXUSDT", "AVAX");
        map.insert("DOTUSDT", "DOT");
        map.insert("LINKUSDT", "LINK");
        map.insert("SHIBUSDT", "SHIB");
        map.insert("SOLUSDT", "SOL");
        map.insert("USDCUSDT", "USDC");
        map.insert("XRPUSDT", "XRP");
        map.insert("TRXUSD", "TRX");
        map
    });

pub static KRAKEN_SYMBOL_MAP: Lazy<
    HashMap<String, &'static Lazy<HashMap<&'static str, &'static str>>>,
> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        "bitlayer_testnet".to_string(),
        &SYMBOL_MAP_KRAKEN_BITLAYER_TESTNET,
    );
    map
});

pub static PRICES_MAPPINGS_BITLAYER_TESTNET: Lazy<HashMap<&'static str, Vec<f64>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("BTC", vec![]);
        map.insert("WBTC", vec![]);
        map.insert("WETH", vec![]);
        map.insert("USDC", vec![]);
        map.insert("BNB", vec![]);
        map.insert("SOL", vec![]);
        map.insert("XRP", vec![]);
        map.insert("TON", vec![]);
        map.insert("DOGE", vec![]);
        map.insert("ADA", vec![]);
        map.insert("TRX", vec![]);
        map.insert("SHIB", vec![]);
        map.insert("AVAX", vec![]);
        map.insert("DOT", vec![]);
        map.insert("LINK", vec![]);
        map.insert("WSTETH", vec![]);
        map
    });


pub static PRICES_MAPPINGS: Lazy<HashMap<String, &'static Lazy<HashMap<&'static str, Vec<f64>>>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "bitlayer_testnet".to_string(),
            &PRICES_MAPPINGS_BITLAYER_TESTNET,
        );
        map
    });
