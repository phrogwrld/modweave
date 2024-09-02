use anyhow::{Context, Result};
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use std::cmp::Reverse;

#[derive(Deserialize, Debug)]
struct GameVersion {
	version: String,
	stable: bool,
}

#[derive(Deserialize, Debug)]
struct LoaderVersion {
	version: String,
	stable: bool,
}

#[derive(Deserialize, Debug)]
struct YarnVersion {
	#[serde(rename = "gameVersion")]
	game_version: String,
	version: String,
	stable: bool,
}

#[derive(Deserialize, Debug)]
struct FabricApiVersion {
	version_number: String,
	game_versions: Vec<String>,
}

pub struct VersionManager {
	client: Client,
}

pub struct VersionData {
	pub minecraft: Vec<(String, bool)>, // (version, stable)
	pub yarn: Vec<(String, bool)>,      // (version, stable)
	pub loader: Vec<(String, bool)>,    // (version, stable)
	pub fabric_api: Vec<String>,        // Fabric API doesn't have stability info
}

impl VersionManager {
	pub fn new() -> Self {
		Self {
			client: Client::new(),
		}
	}

	pub fn fetch_all_versions(&self) -> Result<VersionData> {
		Ok(VersionData {
			minecraft: self.fetch_minecraft_versions()?,
			yarn: self.fetch_yarn_versions()?,
			loader: self.fetch_loader_versions()?,
			fabric_api: self.fetch_fabric_api_versions()?,
		})
	}

	fn fetch_minecraft_versions(&self) -> Result<Vec<(String, bool)>> {
		let versions: Vec<GameVersion> = self
			.client
			.get("https://meta.fabricmc.net/v2/versions/game")
			.send()
			.context("Failed to fetch Minecraft versions")?
			.json()
			.context("Failed to parse Minecraft versions")?;

		Ok(versions
			.into_iter()
			.map(|v| (v.version, v.stable))
			.collect())
	}

	fn fetch_yarn_versions(&self) -> Result<Vec<(String, bool)>> {
		let versions: Vec<YarnVersion> = self
			.client
			.get("https://meta.fabricmc.net/v2/versions/yarn")
			.send()
			.context("Failed to fetch Yarn versions")?
			.json()
			.context("Failed to parse Yarn versions")?;

		Ok(versions
			.into_iter()
			.map(|v| (v.version, v.stable))
			.collect())
	}

	fn fetch_loader_versions(&self) -> Result<Vec<(String, bool)>> {
		let versions: Vec<LoaderVersion> = self
			.client
			.get("https://meta.fabricmc.net/v2/versions/loader")
			.send()
			.context("Failed to fetch Fabric Loader versions")?
			.json()
			.context("Failed to parse Fabric Loader versions")?;

		Ok(versions
			.into_iter()
			.map(|v| (v.version, v.stable))
			.collect())
	}

	fn fetch_fabric_api_versions(&self) -> Result<Vec<String>> {
		let versions: Vec<FabricApiVersion> = self
			.client
			.get("https://api.modrinth.com/v2/project/P7dR8mSH/version")
			.send()
			.context("Failed to fetch Fabric API versions")?
			.json()
			.context("Failed to parse Fabric API versions")?;

		Ok(self.sort_versions(versions.into_iter().map(|v| v.version_number)))
	}

	pub fn get_compatible_yarn_versions(&self, minecraft_version: &str) -> Result<Vec<String>> {
		let all_yarn_versions: Vec<YarnVersion> = self
			.client
			.get("https://meta.fabricmc.net/v2/versions/yarn")
			.send()
			.context("Failed to fetch Yarn versions")?
			.json()
			.context("Failed to parse Yarn versions")?;

		let compatible_versions: Vec<String> = all_yarn_versions
			.into_iter()
			.filter(|v| v.game_version == minecraft_version)
			.map(|v| v.version)
			.collect();

		Ok(self.sort_versions(compatible_versions.into_iter()))
	}

	pub fn get_compatible_fabric_api_versions(
		&self,
		minecraft_version: &str,
	) -> Result<Vec<String>> {
		let all_versions: Vec<FabricApiVersion> = self
			.client
			.get("https://api.modrinth.com/v2/project/P7dR8mSH/version")
			.send()
			.context("Failed to fetch Fabric API versions")?
			.json()
			.context("Failed to parse Fabric API versions")?;

		let compatible_versions: Vec<String> = all_versions
			.into_iter()
			.filter(|v| v.game_versions.contains(&minecraft_version.to_string()))
			.map(|v| v.version_number)
			.collect();

		Ok(self.sort_versions(compatible_versions.into_iter()))
	}

	fn sort_versions<I>(&self, versions: I) -> Vec<String>
	where
		I: Iterator<Item = String>,
	{
		let mut sorted: Vec<_> = versions
			.filter_map(|v| Version::parse(&v).ok().map(|parsed| (v, parsed)))
			.collect();

		sorted.sort_by_key(|(_, v)| Reverse(v.clone()));
		sorted.into_iter().map(|(v, _)| v).collect()
	}
}
