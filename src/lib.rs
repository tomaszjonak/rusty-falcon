//! Rust-based SDK to CrowdStrike's Falcon APIs
//!
//! rusty_falcon documentation is available on [docs.rs](https://docs.rs/rusty_falcon/latest/rusty_falcon/).
//! Users are advised to consult this rusty_falcon documentation together with the comprehensive CrowdStrike
//! API documentation published on [Developer Portal](https://developer.crowdstrike.com/crowdstrike/docs).
//! The easiest way to learn about the SDK is to consult the set of
//! [examples](https://github.com/CrowdStrike/rusty-falcon/tree/main/examples) built on top of the SDK.
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest and highest-level way to establish API client is to instantiate
//! [`easy::client::FalconHandle`]. The most convenient way is to use [`easy::client::FalconHandle::from_env`]
//! function that will read the following environment variables to authenticate with falcon cloud:
//! `FALCON_CLIENT_ID`, `FALCON_CLIENT_SECRET`, and `FALCON_CLOUD`. Unless you already have a CrowdStrike key
//! pair you can establish a new one in [Falcon Portal](https://falcon.crowdstrike.com/support/api-clients-and-keys).
//!
//! ```
//! use rusty_falcon::apis::sensor_download_api;
//! use rusty_falcon::easy::client::FalconHandle;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Fetch credentials from environment variables and establish OAuth2 connection
//!     let falcon = FalconHandle::from_env()
//!         .await
//!         .expect("Could not authenticate with CrowdStrike API");
//!
//!     // Call one particular API end-point using the authenticated client
//!     let response = sensor_download_api::get_sensor_installers_ccidby_query(&falcon.cfg)
//!         .await
//!         .expect("Could not fetch CCID");
//!
//!     // Response objects returned from APIs usually follow the same pattern of having
//!     // 'errors', 'meta', and 'resources' fields. It is recommended to check for possible
//!     // application errors:
//!     if !response.errors.is_empty() {
//!         eprintln!("Errors occured while getting Falcon CCID: {:?}", response.errors);
//!     }
//!
//!     // Print response from the API:
//!     println!("{:?}", response.resources)
//! }
//! ```
//!
//! # Examples
//! Ready-made examples can be found in [git repo](https://github.com/CrowdStrike/rusty-falcon/tree/main/examples).
//!
//! # Getting Help
//! rusty_falcon is an open source project, not a CrowdStrike product. As such it carries no formal support,
//! expressed or implied.
//!
//! If you encounter any issues while using rusty_falcon, you can create an issue on our
//! [Github repo](https://github.com/CrowdStrike/rusty-falcon) for bugs, enhancements, or other requests.
//!
//! rusty_falcon project is periodically refreshed to reflect the newest additions to the CrowdStrike API. Users
//! of the SDK are advised to track the latest releases rather closely to ensure proper function in the unlikely
//! event of an incompatible change to a CrowdStrike API.
//!

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod easy;
pub mod models;
