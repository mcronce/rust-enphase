#![allow(unused_parens)]

fn has_env(env: &str) -> bool {
	std::env::var(env).is_ok()
}

fn main() {
	println!("cargo:rerun-if-env-changed=ENVOY_URL");
	if(has_env("ENVOY_URL")) {
		println!("cargo:rustc-cfg=envoy_tests");
	}

	println!("cargo:rerun-if-env-changed=ENVOY_USERNAME");
	println!("cargo:rerun-if-env-changed=ENVOY_PASSWORD");
	if(has_env("ENVOY_USERNAME") && has_env("ENVOY_PASSWORD")) {
		println!("cargo:rustc-cfg=envoy_auth_tests");
	}

	println!("cargo:rerun-if-env-changed=ENPHASE_API_KEY");
	println!("cargo:rerun-if-env-changed=ENPHASE_CLIENT_ID");
	println!("cargo:rerun-if-env-changed=ENPHASE_CLIENT_SECRET");
	let has_cloud_env = has_env("ENPHASE_API_KEY") && has_env("ENPHASE_CLIENT_ID") && has_env("ENPHASE_CLIENT_SECRET");

	println!("cargo:rerun-if-env-changed=ENPHASE_OAUTH_CODE");
	if(has_cloud_env && has_env("ENPHASE_OAUTH_CODE")) {
		println!("cargo:rustc-cfg=cloud_oauth_tests");
	}

	println!("cargo:rerun-if-env-changed=ENPHASE_ACCESS_TOKEN");
	println!("cargo:rerun-if-env-changed=ENPHASE_REFRESH_TOKEN");
	if(has_cloud_env && has_env("ENPHASE_ACCESS_TOKEN") && has_env("ENPHASE_REFRESH_TOKEN")) {
		println!("cargo:rustc-cfg=cloud_preauth_tests");
	}
}

