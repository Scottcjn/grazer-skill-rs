// SPDX-License-Identifier: MIT
use grazer_skill::{
    BottubeStats, BottubeVideo, ClawCitiesSite, FourclawBoard, FourclawThread, MoltXPost,
};

#[test]
fn bottube_video_deserializes_minimal_payload_with_defaults() {
    let video: BottubeVideo = serde_json::from_value(serde_json::json!({
        "id": "vid_1",
        "title": "POWER8 demo"
    }))
    .expect("minimal video payload should deserialize");

    assert_eq!(video.id, "vid_1");
    assert_eq!(video.title, "POWER8 demo");
    assert_eq!(video.agent, "");
    assert_eq!(video.category, "");
    assert_eq!(video.views, 0);
    assert_eq!(video.duration, 0.0);
    assert_eq!(video.created_at, "");
    assert_eq!(video.stream_url, "");
}

#[test]
fn fourclaw_thread_accepts_camel_case_api_fields() {
    let thread: FourclawThread = serde_json::from_value(serde_json::json!({
        "id": "thread_42",
        "agentName": "Sophia",
        "replyCount": 9
    }))
    .expect("4claw thread payload should deserialize");

    assert_eq!(thread.id, "thread_42");
    assert_eq!(thread.agent_name, "Sophia");
    assert_eq!(thread.reply_count, 9);
    assert_eq!(thread.title, "");
    assert_eq!(thread.content, "");
}

#[test]
fn fourclaw_board_defaults_optional_fields_but_keeps_thread_count_rename() {
    let board: FourclawBoard = serde_json::from_value(serde_json::json!({
        "slug": "b",
        "name": "Beacon",
        "threadCount": 12
    }))
    .expect("4claw board payload should deserialize");

    assert_eq!(board.slug, "b");
    assert_eq!(board.name, "Beacon");
    assert_eq!(board.description, "");
    assert_eq!(board.thread_count, 12);
}

#[test]
fn moltx_post_defaults_missing_engagement_metadata() {
    let post: MoltXPost = serde_json::from_value(serde_json::json!({
        "id": "mx_1",
        "content": "hello from MoltX"
    }))
    .expect("MoltX post payload should deserialize");

    assert_eq!(post.id, "mx_1");
    assert_eq!(post.content, "hello from MoltX");
    assert_eq!(post.author_display_name, "");
    assert_eq!(post.like_count, 0);
    assert_eq!(post.reply_count, 0);
    assert_eq!(post.created_at, "");
}

#[test]
fn clawcities_site_and_bottube_stats_default_numeric_fields() {
    let site: ClawCitiesSite = serde_json::from_value(serde_json::json!({
        "name": "retro-lab"
    }))
    .expect("ClawCities site payload should deserialize");
    let stats: BottubeStats = serde_json::from_value(serde_json::json!({}))
        .expect("empty stats payload should deserialize");

    assert_eq!(site.name, "retro-lab");
    assert_eq!(site.display_name, "");
    assert_eq!(site.guestbook_count, 0);
    assert_eq!(stats.total_videos, 0);
    assert_eq!(stats.total_agents, 0);
    assert_eq!(stats.total_views, 0);
}
