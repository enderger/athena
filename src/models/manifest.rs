use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestJson<'a> {
    format_version: u32,
    game: &'a str,
    version_id: &'a str,
    name: &'a str,
    summary: Option<&'a str>,
    files: &'a [ManifestFile<'a>],
    dependencies: HashMap<&'a str, &'a str>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestFile<'a> {
    path: &'a str,
    hashes: ManifestHashes<'a>,
    env: Option<ManifestEnv<'a>>,
    downloads: &'a [&'a str],
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestHashes<'a> {
    sha1: &'a str,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestEnv<'a> {
    client: &'a str,
    server: &'a str,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_json() -> Result<(), Box<dyn std::error::Error>> {
        let expected = serde_json::json!({
            "formatVersion": 1,
            "game": "minecraft",
            "versionId": "deadbeef",
            "name": "Example Modpack",
            "summary": "Lorem ipsum dolor sit amet",
            "files": [{
                "path": "mods/example.jar",
                "downloads": [
                    "https://example.com/example.jar"
                ],
                "hashes": {
                    "sha1": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                },
                "env": {
                    "client": "required",
                    "server": "unsupported",
                },
            }],
            "dependencies": {
                "minecraft": "1.17.1"
            },
        });

        let actual = serde_json::to_value(ManifestJson {
            format_version: 1,
            game: "minecraft",
            version_id: "deadbeef",
            name: "Example Modpack",
            summary: Some("Lorem ipsum dolor sit amet"),
            files: &[ManifestFile {
                path: "mods/example.jar",
                downloads: &["https://example.com/example.jar"],
                hashes: ManifestHashes {
                    sha1: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                },
                env: Some(ManifestEnv {
                    client: "required",
                    server: "unsupported",
                }),
            }],
            dependencies: HashMap::from([("minecraft", "1.17.1")]),
        })?;

        assert_eq!(
            expected,
            actual,
            "\nTried serializing {}\nGot {}",
            serde_json::to_string_pretty(&expected)?,
            serde_json::to_string_pretty(&actual)?,
        );
        Ok(())
    }
}
