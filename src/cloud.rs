use std::sync::Arc;

use arcstr::ArcStr;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Mutex;

mod connection_type;
pub use connection_type::ConnectionType;
pub use connection_type::InvalidConnectionType;
mod granularity;
pub use granularity::Granularity;
pub use granularity::InvalidGranularity;
mod system;
pub use system::Address;
pub use system::Metadata;
pub use system::MicroinverterProduction;
pub use system::System;
pub use system::SystemSummary;
use system::ListSystemsResponse;

#[cfg(feature = "clap")]
#[derive(Debug, clap::Parser)]
pub struct Config {
	#[clap(long, env = "ENPHASE_API_KEY")]
	api_key: String,
	#[clap(long, env = "ENPHASE_CLIENT_ID")]
	client_id: String,
	#[clap(long, env = "ENPHASE_CLIENT_SECRET")]
	client_secret: String,
	#[clap(long, env = "ENPHASE_OAUTH_CODE")]
	code: Option<String>,
	#[clap(long, env = "ENPHASE_ACCESS_TOKEN")]
	access_token: Option<String>,
	#[clap(long, env = "ENPHASE_REFRESH_TOKEN")]
	refresh_token: Option<String>
}

#[cfg(feature = "clap")]
impl Config {
	#[inline]
	pub async fn client(&self) -> Result<Client, reqwest::Error> {
		match (self.code.as_ref(), self.access_token.as_ref(), self.refresh_token.as_ref()) {
			(Some(code), None, None) => Client::oauth(&self.api_key, self.client_id.clone(), self.client_secret.clone(), code).await,
			(None, Some(access_token), Some(refresh_token)) => Ok(Client::preauth(&self.api_key, self.client_id.clone(), self.client_secret.clone(), access_token.to_owned(), refresh_token.to_owned())),
			(Some(_), Some(_), Some(_)) => todo!("Error for having both code and tokens set"),
			_ => todo!("Error for not having either code or both tokens set")
		}
	}
}

pub struct Client {
	client: reqwest::Client,
	api_key_qstr: ArcStr,
	//client_id: String,
	//client_secret: String,
	token_auth_header: ArcStr,
	auth_header: Arc<Mutex<String>>,
	access_token: String,
	refresh_token: String
}

impl Client {
	pub async fn oauth(api_key: &str, client_id: String, client_secret: String, code: &str) -> Result<Self, reqwest::Error> {
		let client = reqwest::Client::new();
		let url = format!("https://api.enphaseenergy.com/oauth/token?grant_type=authorization_code&redirect_uri=https://api.enphaseenergy.com/oauth/redirect_uri&code={code}");
		let token_auth_header = Self::token_auth_header(&client_id, &client_secret);
		let response: AuthResponse = client
			.post(url)
			.header("Authorization", &*token_auth_header)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;
		let auth_header = Arc::new(Mutex::new(format!("Bearer {}", &response.access_token)));
		Ok(Self{
			client,
			api_key_qstr: format!("key={api_key}").into(),
			//client_id,
			//client_secret,
			token_auth_header,
			auth_header,
			access_token: response.access_token,
			refresh_token: response.refresh_token
		})
	}

	pub fn preauth(api_key: &str, client_id: String, client_secret: String, access_token: String, refresh_token: String) -> Self {
		let token_auth_header = Self::token_auth_header(&client_id, &client_secret);
		let auth_header = Arc::new(Mutex::new(format!("Bearer {}", &access_token)));
		Self{
			client: reqwest::Client::new(),
			api_key_qstr: format!("key={api_key}").into(),
			//client_id,
			//client_secret,
			token_auth_header,
			auth_header,
			access_token,
			refresh_token
		}
	}

	pub fn tokens(&self) -> Tokens {
		Tokens{
			access: self.access_token.clone(),
			refresh: self.refresh_token.clone()
		}
	}

	pub async fn refresh(&mut self) -> Result<Tokens, reqwest::Error> {
		let response: AuthResponse = self.client
			.post(format!("https://api.enphaseenergy.com/oauth/token?grant_type=refresh_token&refresh_token={}", self.refresh_token))
			.header("Authorization", &*self.token_auth_header)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;
		*self.auth_header.lock().await = format!("Bearer {}", &response.access_token);
		self.access_token = response.access_token;
		self.refresh_token = response.refresh_token;
		Ok(self.tokens())
	}

	pub async fn list_systems(&self) -> Result<Vec<System>, reqwest::Error> {
		let response: ListSystemsResponse = self.client
			.get(format!("https://api.enphaseenergy.com/api/v4/systems?{}", self.api_key_qstr))
			.header("Authorization", &*self.auth_header.lock().await)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await?;
		let response = response
			.systems
			.into_iter()
			.map(|s| System::from((self.client.clone(), self.api_key_qstr.clone(), self.auth_header.clone(), s)))
			.collect();
		Ok(response)
	}

	#[inline]
	fn token_auth_header(client_id: &str, client_secret: &str) -> ArcStr {
		format!("Basic {}", base64::encode(format!("{client_id}:{client_secret}"))).into()
	}
}

#[derive(Clone, Debug, Deserialize)]
struct AuthResponse {
	access_token: String,
	//token_type: CompactString,
	refresh_token: String,
	//expires_in: u32,
	//scope: CompactString,
	//enl_uid: CompactString,
	//enl_cid: CompactString,
	//enl_password_last_changed: CompactString,
	//is_internal_app: bool,
	//app_type: CompactString,
	//jti: Uuid
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tokens {
	access: String,
	refresh: String
}

#[cfg(test)]
mod test {
	use std::env;
	use time::macros::date;
	use super::*;

	#[allow(dead_code)]
	async fn client_oauth() -> Client {
		Client::oauth(
			&env::var("ENPHASE_API_KEY").unwrap(),
			env::var("ENPHASE_CLIENT_ID").unwrap(),
			env::var("ENPHASE_CLIENT_SECRET").unwrap(),
			&env::var("ENPHASE_OAUTH_CODE").unwrap()
		).await.unwrap()
	}

	#[allow(dead_code)]
	fn client_preauth() -> Client {
		Client::preauth(
			&env::var("ENPHASE_API_KEY").unwrap(),
			env::var("ENPHASE_CLIENT_ID").unwrap(),
			env::var("ENPHASE_CLIENT_SECRET").unwrap(),
			env::var("ENPHASE_ACCESS_TOKEN").unwrap(),
			env::var("ENPHASE_REFRESH_TOKEN").unwrap()
		)
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_oauth_tests), ignore)]
	/// The OAUTH code is only usable once, so we have to generate a new one for each test run,
	/// and generating them programmatically is not practical.  As a result, we have to do this
	/// unfortunate single test for all methods when logging in with OAUTH.
	async fn test_oauth_complete() {
		let mut client = client_oauth().await;
		client.refresh().await.unwrap();
		client.list_systems().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_preauth_tests), ignore)]
	async fn test_preauth_refresh() {
		let mut client = client_preauth();
		client.refresh().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_preauth_tests), ignore)]
	async fn test_preauth_list_systems() {
		let client = client_preauth();
		client.list_systems().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_preauth_tests), ignore)]
	async fn test_preauth_system_get_summary() {
		let client = client_preauth();
		let system = client.list_systems().await.unwrap().pop().unwrap();
		system.get_summary().await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_preauth_tests), ignore)]
	async fn test_preauth_system_get_lifetime_production() {
		let client = client_preauth();
		let system = client.list_systems().await.unwrap().pop().unwrap();
		system.get_lifetime_production(None, None, false).await.unwrap();
		system.get_lifetime_production(None, None, true).await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_preauth_tests), ignore)]
	async fn test_preauth_system_get_microinverter_production() {
		let client = client_preauth();
		let system = client.list_systems().await.unwrap().pop().unwrap();
		system.get_microinverter_production(&date!(2022-12-23), None).await.unwrap();
	}

	#[tokio::test]
	#[cfg_attr(not(cloud_preauth_tests), ignore)]
	async fn test_preauth_system_calls_with_token_refresh() {
		let mut client = client_preauth();
		client.refresh().await.unwrap();
		let system = client.list_systems().await.unwrap().pop().unwrap();
		client.refresh().await.unwrap();
		system.get_summary().await.unwrap();
		client.refresh().await.unwrap();
		system.get_lifetime_production(None, None, false).await.unwrap();
		client.refresh().await.unwrap();
		system.get_microinverter_production(&date!(2021-12-25), None).await.unwrap();
	}
}

