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
pub struct DomainNotificationV1 {
    /// The email of the user who is assigned to this notification
    #[serde(rename = "assigned_to_uid", skip_serializing_if = "Option::is_none")]
    pub assigned_to_uid: Option<String>,
    /// The name of the user who is assigned to this notification
    #[serde(rename = "assigned_to_username", skip_serializing_if = "Option::is_none")]
    pub assigned_to_username: Option<String>,
    /// The unique ID of the user who is assigned to this notification
    #[serde(rename = "assigned_to_uuid", skip_serializing_if = "Option::is_none")]
    pub assigned_to_uuid: Option<String>,
    #[serde(rename = "breach_summary", skip_serializing_if = "Option::is_none")]
    pub breach_summary: Option<Box<crate::models::DomainMatchedBreachSummaryV1>>,
    #[serde(rename = "cid")]
    pub cid: String,
    /// The date when the notification was generated
    #[serde(rename = "created_date")]
    pub created_date: String,
    /// Highlighted content based on the rule that generated the notifications. Highlights are surrounded with a `<cs-highlight>` tag
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<String>>,
    /// The ID of the notification
    #[serde(rename = "id")]
    pub id: String,
    /// The author who posted the intelligence item
    #[serde(rename = "item_author", skip_serializing_if = "Option::is_none")]
    pub item_author: Option<String>,
    /// The ID of the author who posted the intelligence item
    #[serde(rename = "item_author_id", skip_serializing_if = "Option::is_none")]
    pub item_author_id: Option<String>,
    /// Timestamp when the item is considered to have been created
    #[serde(rename = "item_date")]
    pub item_date: String,
    /// ID of the item which matched the rule
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The site where the intelligence item was found
    #[serde(rename = "item_site", skip_serializing_if = "Option::is_none")]
    pub item_site: Option<String>,
    /// The ID of the site where the intelligence item was found
    #[serde(rename = "item_site_id", skip_serializing_if = "Option::is_none")]
    pub item_site_id: Option<String>,
    /// Type of the item which matched the rule: `post`, `reply`, `botnet_config`, `breach`, etc.
    #[serde(rename = "item_type")]
    pub item_type: String,
    /// ID of the raw intel item that matched the rule
    #[serde(rename = "raw_intel_id")]
    pub raw_intel_id: String,
    /// The ID of the rule that generated this notification
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// The name of the rule that generated this notification
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    /// The priority of the rule that generated this notification
    #[serde(rename = "rule_priority")]
    pub rule_priority: String,
    /// The topic of the rule that generated this notification
    #[serde(rename = "rule_topic")]
    pub rule_topic: String,
    /// Category of the source that generated the notification
    #[serde(rename = "source_category", skip_serializing_if = "Option::is_none")]
    pub source_category: Option<String>,
    /// The notification status. This can be one of: `new`, `in-progress`, `closed-false-positive`, `closed-true-positive`.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "typosquatting", skip_serializing_if = "Option::is_none")]
    pub typosquatting: Option<Box<crate::models::SadomainTyposquattingComponent>>,
    /// The date when the notification was updated
    #[serde(rename = "updated_date")]
    pub updated_date: String,
}

impl DomainNotificationV1 {
    pub fn new(cid: String, created_date: String, id: String, item_date: String, item_id: String, item_type: String, raw_intel_id: String, rule_id: String, rule_name: String, rule_priority: String, rule_topic: String, status: String, updated_date: String) -> DomainNotificationV1 {
        DomainNotificationV1 {
            assigned_to_uid: None,
            assigned_to_username: None,
            assigned_to_uuid: None,
            breach_summary: None,
            cid,
            created_date,
            highlights: None,
            id,
            item_author: None,
            item_author_id: None,
            item_date,
            item_id,
            item_site: None,
            item_site_id: None,
            item_type,
            raw_intel_id,
            rule_id,
            rule_name,
            rule_priority,
            rule_topic,
            source_category: None,
            status,
            typosquatting: None,
            updated_date,
        }
    }
}
