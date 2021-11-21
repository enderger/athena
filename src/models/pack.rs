use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Pack {
    modpack: PackMetadata,
    game: PackGame,
    #[serde(default)]
    sources: BTreeMap<String, PackSource>,
    #[serde(default)]
    files: Vec<PackFile>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct PackMetadata {
    name: String,
    version: String,
    summary: Option<String>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PackGame {
    MinecraftFabric {
        minecraft: String,
        #[serde(rename = "fabric-loader")]
        fabric_loader: String,
    },
    MinecraftForge {
        minecraft: String,
        forge: String,
    },
    MinecraftVanilla {
        minecraft: String,
    },
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum PackSource {
    #[serde(rename = "labrinth.v1")]
    LabrinthV1 { url: String },
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct PackFile {
    path: PathBuf,
    sources: Option<HashSet<String>>,
    version: PackVersion,
    #[serde(default)]
    environment: Option<PackEnv>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PackVersion {
    Latest {
        #[serde(default)]
        channel: PackChannel,
    },
    SemVer {
        version: crate::serde::de::VersionReq,
    },
    Exact {
        version: String,
    },
    Download {
        sources: Vec<String>,
        sha1: Option<String>,
    },
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PackChannel {
    Release,
    Beta,
    Alpha,
}

impl Default for PackChannel {
    fn default() -> Self {
        Self::Release
    }
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PackEnv {
    Both,
    Client,
    Server,
}

impl Default for PackEnv {
    fn default() -> Self {
        Self::Both
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::*;

    static PACK_HEADER: &str = r#"
        [modpack]
        name = "deadbeef"
        version = "0.1.0"
        summary = "Lorem ipsum dolor sit amet"
    "#;

    lazy_static::lazy_static! {
        static ref DEFAULT_PACK: Pack = Pack {
            modpack: PackMetadata {
                name: String::from("deadbeef"),
                version: String::from("0.1.0"),
                summary: Some(String::from("Lorem ipsum dolor sit amet")),
            },
            game: PackGame::MinecraftVanilla {
                minecraft: String::from("1.17.1"),
            },
            sources: BTreeMap::new(),
            files: Vec::new(),
        };
    }

    fn test_pack(ser: &str, de: Pack) -> Result<(), Box<dyn Error>> {
        let pack_str = [PACK_HEADER, ser]
            .iter()
            .map(|it| it.trim_start())
            .flat_map(|it| it.split('\n'))
            .map(&str::trim)
            .fold(String::new(), |acc, it| format!("{}{}\n", acc, it));
        println!("{}", pack_str);
        let actual = toml::de::from_str::<Pack>(&pack_str)?;

        assert_eq!(de, actual);
        Ok(())
    }

    #[test]
    fn vanilla() -> Result<(), Box<dyn Error>> {
        const PACK: &str = r#"
            [game]
            minecraft = "1.17.1"
        "#;

        let expected = Pack {
            game: PackGame::MinecraftVanilla {
                minecraft: String::from("1.17.1"),
            },
            ..DEFAULT_PACK.clone()
        };

        test_pack(PACK, expected)
    }

    #[test]
    fn pure_fabric() -> Result<(), Box<dyn Error>> {
        const PACK: &str = r#"
            [game]
            minecraft = "1.17.1"
            fabric-loader = "0.9.0"
        "#;

        let expected = Pack {
            game: PackGame::MinecraftFabric {
                minecraft: String::from("1.17.1"),
                fabric_loader: String::from("0.9.0"),
            },
            ..DEFAULT_PACK.clone()
        };

        test_pack(PACK, expected)
    }

    #[test]
    fn sodium_fabric() -> Result<(), Box<dyn Error>> {
        const PACK: &str = r#"
            [game]
            minecraft = "1.17.1"
            fabric-loader = "0.9.0"

            [sources.modrinth]
            type = "labrinth.v1"
            url = "https://api.modrinth.com"

            [[files]]
            path = "mods/sodium.jar"
            sources = ["modrinth"]
            environment = "client"

            [files.version]
            type = "latest"
            channel = "release"
        "#;

        let expected = Pack {
            game: PackGame::MinecraftFabric {
                minecraft: String::from("1.17.1"),
                fabric_loader: String::from("0.9.0"),
            },
            sources: BTreeMap::from([(
                String::from("modrinth"),
                PackSource::LabrinthV1 {
                    url: String::from("https://api.modrinth.com"),
                },
            )]),
            files: vec![PackFile {
                path: PathBuf::from("mods/sodium.jar"),
                sources: Some(HashSet::from([String::from("modrinth")])),
                version: PackVersion::Latest {
                    channel: PackChannel::Release,
                },
                environment: Some(PackEnv::Client),
            }],
            ..DEFAULT_PACK.clone()
        };

        test_pack(PACK, expected)
    }
}
