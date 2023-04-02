use std::env;
use std::fmt::{Debug};

use log::{info, trace};

#[derive(Clone, Debug)]
pub struct Config {
    pub path_auth_code: String,
    pub path_authentications: String,
    pub use_file_auth_response: bool,
    pub use_local_auth_redirect: bool,
    pub auth_file_read_timeout: u64,

    pub twitch_client_id: String,
    pub twitch_client_secret: String,
    pub twitch_downloader_id: String,

    pub bigquery_project_id: String,
    pub bigquery_dataset_id: String,
    pub bigquery_service_account_path: String,

    pub youtube_client_secret_path: String,
    pub youtube_tags: Vec<String>,
    pub youtube_description_template: String,
    pub youtube_video_length_minutes: i64,

    pub download_folder_path: String,
    pub downloader_threads: usize,

}

pub fn load_config() -> Config {
    trace!("load_config()");
    info!("Loading config from environment variables");
    let twitch_client_id = env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID not set");
    let twitch_client_secret = env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET not set");
    let twitch_downloader_id = "kimne78kx3ncx6brgo4mv6wki5h1ko".to_string();

    let path_auth_code =
        env::var("PATH_AUTH_CODE").unwrap_or("/tmp/twba/auth/code.txt".to_string());
    let path_authentications =
        env::var("PATH_AUTHENTICATIONS").unwrap_or("/tmp/twba/auth/{user}.json".to_string());
    let use_file_auth_response =
        env::var("USE_FILE_AUTH_RESPONSE").unwrap_or("1".to_string()) == "1";
    let use_local_auth_redirect =
        env::var("USE_LOCAL_AUTH_REDIRECT").unwrap_or("0".to_string()) == "1";
    let auth_file_read_timeout = env::var("AUTH_FILE_READ_TIMEOUT")
        .unwrap_or("5".to_string())
        .parse()
        .unwrap();

    let bigquery_project_id =
        env::var("BIGQUERY_PROJECT_ID").unwrap_or("twitchbackup-v1".to_string());
    let bigquery_dataset_id = env::var("BIGQUERY_DATASET_ID").unwrap_or("backup_data".to_string());
    let bigquery_service_account_path = env::var("BIGQUERY_SERVICE_ACCOUNT_PATH")
        .unwrap_or("auth/bigquery_service_account.json".to_string());

    let youtube_client_secret_path = env::var("YOUTUBE_CLIENT_SECRET_PATH")
        .unwrap_or("auth/youtube_client_secret.json".to_string());

    let youtube_tags = env::var("YOUTUBE_TAGS")
        .unwrap_or("".to_string())
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let youtube_description_template =
        env::var("YOUTUBE_DESCRIPTION_TEMPLATE").unwrap_or("test description for \"$$video_title$$\"".to_string());

    let youtube_video_length_minutes =
        env::var("YOUTUBE_VIDEO_LENGTH_MINUTES").unwrap_or("30".to_string()).parse().unwrap_or(30i64);


    let download_folder_path =
        env::var("DOWNLOAD_FOLDER_PATH").unwrap_or("/tmp/twba/videos/".to_string());
    let downloader_threads =
        env::var("DOWNLOADER_THREADS").unwrap_or(10.to_string()).parse().unwrap();
    trace!("load_config() done loading fields");
    Config {
        path_auth_code,
        use_file_auth_response,
        path_authentications,
        use_local_auth_redirect,
        auth_file_read_timeout,
        twitch_client_id,
        twitch_client_secret,
        twitch_downloader_id,
        bigquery_project_id,
        bigquery_dataset_id,
        bigquery_service_account_path,
        youtube_client_secret_path,
        youtube_tags,
        youtube_description_template,
        youtube_video_length_minutes,
        download_folder_path,
        downloader_threads,
    }
}
