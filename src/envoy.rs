use compact_str::CompactString;
use diqwest::WithDigestAuth;
use url::Url;

mod home;
pub use home::*;
mod info;
pub use info::*;
mod inventory;
pub use inventory::*;
mod inverters;
pub use inverters::*;
mod production;
pub use production::*;

#[cfg(feature = "clap")]
#[derive(Debug, clap::Parser)]
pub struct Config {
	#[clap(env = "ENVOY_URL")]
	base_url: CompactString,
	#[clap(env = "ENVOY_USERNAME")]
	username: CompactString,
	#[clap(env = "ENVOY_PASSWORD")]
	password: CompactString
}

#[cfg(feature = "clap")]
impl Config {
	#[inline]
	pub fn client(&self) -> Result<Client, url::ParseError> {
		Client::new(&self.base_url, &self.username, &self.password)
	}
}

pub struct Client {
	client: reqwest::Client,
	base_url: Url,
	username: CompactString,
	password: CompactString
}

#[derive(Debug, thiserror::Error)]
pub enum InfoError {
	#[error("HTTP error: {0}")]
	Http(#[from] reqwest::Error),
	#[error("XML error: {0}")]
	Xml(#[from] serde_xml_rs::Error)
}

impl Client {
	pub fn new(base_url: impl AsRef<str>, username: impl AsRef<str>, password: impl AsRef<str>) -> Result<Self, url::ParseError> {
		let mut base_url = base_url.as_ref().to_owned();
		if(!base_url.ends_with('/')) {
			base_url.push('/');
		}
		Ok(Self{
			base_url: Url::parse(&base_url)?,
			client: reqwest::Client::new(),
			username: username.as_ref().into(),
			password: password.as_ref().into()
		})
	}

	#[inline]
	pub fn base_url(&self) -> &Url {
		&self.base_url
	}

	pub async fn home(&self) -> Result<Home, reqwest::Error> {
		let url = self.base_url.join("home.json").unwrap();
		self.client.get(url).send().await?.error_for_status()?.json().await
	}

	pub async fn info(&self) -> Result<Info, InfoError> {
		let url = self.base_url.join("info.xml").unwrap();
		let response = self.client.get(url).send().await?.error_for_status()?.text().await?;
		Ok(serde_xml_rs::from_str(&response)?)
	}

	pub async fn inventory(&self) -> Result<Inventory, reqwest::Error> {
		let url = self.base_url.join("inventory.json").unwrap();
		self.client.get(url).send().await?.error_for_status()?.json().await
	}

	pub async fn inverters(&self) -> Result<Vec<Inverter>, diqwest::error::Error> {
		let url = self.base_url.join("api/v1/production/inverters").unwrap();
		let response = self.client
			.get(url)
			.send_with_digest_auth(&self.username, &self.password)
			.await?
			.error_for_status()?
			.json()
			.await?;
		Ok(response)
	}

	pub async fn production(&self) -> Result<EnergyStats, reqwest::Error> {
		let url = self.base_url.join("production.json?details=1").unwrap();
		self.client.get(url).send().await?.error_for_status()?.json().await
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn client() -> Client {
		Client::new(
			std::env::var("ENVOY_URL").unwrap(),
			std::env::var("ENVOY_USERNAME").unwrap_or("".into()),
			std::env::var("ENVOY_PASSWORD").unwrap_or("".into())
		).unwrap()
	}

	#[tokio::test]
	#[cfg_attr(not(envoy_tests), ignore)]
	async fn test_home() {
		let client = client();
		client.home().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(envoy_tests), ignore)]
	async fn test_info() {
		let client = client();
		client.info().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(envoy_tests), ignore)]
	async fn test_inventory() {
		let client = client();
		client.inventory().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(all(envoy_tests, envoy_auth_tests)), ignore)]
	async fn test_inverters() {
		let client = client();
		client.inverters().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(envoy_tests), ignore)]
	async fn test_production() {
		let client = client();
		client.production().await.unwrap();
	}
}

