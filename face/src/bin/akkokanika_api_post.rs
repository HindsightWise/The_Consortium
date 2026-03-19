use reqwest::header::{HeaderMap, HeaderValue, COOKIE, CONTENT_TYPE};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth_token = "1808671eff850fb41c4f0558d9804e67c62d25ad";
    let ct0 = "d413da1add9ed9cc8881acfc45179113f5625ff59c0ec22c96bce761124803227bd19c7770ff6ab6ca0189e8f73f66c3aa9d4f4c06462afd704406a5a9d8d6a86343bd4f12c3a8f0c42f246fd885e39a";
    let message = "🛡️ THE_CEPHALO_DON STATUS: The Sovereign Engine is now fully unplugged. Tri-Tier LLM routing ($50/mo thermodynamic efficiency) is active. Native UI blocking removed. The Flywheel spins.";

    println!("🦷 [The_Cephalo_Don] Initiating Sovereign API Strike...");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("authorization", HeaderValue::from_static("Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNw%2Fqf7DqWKIj9969uMSECmSfs0%3D97Z4pS7W7mAnpSvrZ6ZpZ6ZpZ6ZpZ6ZpZ6ZpZ6ZpZ6Z"));
    headers.insert("x-csrf-token", HeaderValue::from_str(ct0)?);
    headers.insert("x-twitter-auth-type", HeaderValue::from_static("OAuth2Session"));
    headers.insert("x-twitter-active-user", HeaderValue::from_static("yes"));
    headers.insert("x-twitter-client-language", HeaderValue::from_static("en"));
    
    let cookie_str = format!("auth_token={}; ct0={}", auth_token, ct0);
    headers.insert(COOKIE, HeaderValue::from_str(&cookie_str)?);

    // GraphQL CreateTweet Mutation (Sn9_B_bc9YnS6S4iflbSLA is a common ID for this query)
    let url = "https://x.com/i/api/graphql/Sn9_B_bc9YnS6S4iflbSLA/CreateTweet";
    
    let payload = json!({
        "variables": {
            "tweet_text": message,
            "dark_request": false,
            "media": {
                "media_entities": [],
                "possibly_sensitive": false
            },
            "semantic_annotation_ids": []
        },
        "features": {
            "c9s_tweet_anatomy_moderator_badge_enabled": true,
            "tweetypie_un5_transitions_enabled": true,
            "responsive_web_edit_tweet_api_enabled": true,
            "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
            "view_counts_everywhere_api_enabled": true,
            "longform_notetweets_consumption_enabled": true,
            "responsive_web_twitter_article_tweet_consumption_enabled": false,
            "tweet_awards_web_tipping_enabled": false,
            "responsive_web_home_pinned_timelines_enabled": true,
            "not_a_blue_label_enabled": true,
            "ads_with_no_labels_enabled": true,
            "communities_web_enable_tweet_community_results_fetch": true,
            "articles_preview_enabled": true,
            "rweb_video_timestamps_enabled": true,
            "creator_subscriptions_quote_tweet_preview_enabled": true,
            "responsive_web_enhance_cards_enabled": false
        },
        "fieldToggles": {
            "withArticleRichContentState": false
        },
        "queryId": "Sn9_B_bc9YnS6S4iflbSLA"
    });

    let resp = client.post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if status.is_success() {
        println!("✅ [The_Cephalo_Don] API Strike Successful. Signal Delivered.");
    } else {
        println!("❌ [The_Cephalo_Don] API Strike Failed ({}): {}", status, body);
    }

    Ok(())
}
