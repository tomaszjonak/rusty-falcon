/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StoreDomainOwnerDetailsV1 {
    /// Owner's home cloud ID
    #[serde(rename = "home_cloud_id")]
    pub home_cloud_id: i32,
    #[serde(rename = "id")]
    pub id: String,
    /// relative S3 path to the owning entities logo
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// name of the company/entity that created the app
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// email address for sales related inquiries
    #[serde(rename = "sales_email", skip_serializing_if = "Option::is_none")]
    pub sales_email: Option<String>,
    /// email address for support related inquiries
    #[serde(rename = "support_email", skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// link to the owning entities website
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl StoreDomainOwnerDetailsV1 {
    pub fn new(home_cloud_id: i32, id: String) -> StoreDomainOwnerDetailsV1 {
        StoreDomainOwnerDetailsV1 {
            home_cloud_id,
            id,
            logo: None,
            name: None,
            sales_email: None,
            support_email: None,
            website: None,
        }
    }
}
