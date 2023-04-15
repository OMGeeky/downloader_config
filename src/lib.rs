use std::env;
use std::fmt::Debug;

use log::{info, trace};
use serde::{Deserialize, Serialize};

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
    pub twitch_downloader_thread_count: u64,

    pub bigquery_project_id: String,
    pub bigquery_dataset_id: String,
    pub bigquery_service_account_path: String,

    pub youtube_client_secret_path: String,
    pub youtube_tags: Vec<String>,
    pub youtube_description_template: String,
    pub youtube_video_length_minutes_soft_cap: i64,
    pub youtube_video_length_minutes_hard_cap: i64,

    pub download_folder_path: String,
}

#[derive(Serialize, Deserialize, Default)]
struct ConfigBuilder {
    pub path_auth_code: Option<String>,
    pub path_authentications: Option<String>,
    pub use_file_auth_response: Option<String>,
    pub use_local_auth_redirect: Option<String>,
    pub auth_file_read_timeout: Option<String>,

    pub twitch_client_id: Option<String>,
    pub twitch_client_secret: Option<String>,
    pub twitch_downloader_id: Option<String>,
    pub twitch_downloader_thread_count: Option<String>,

    pub bigquery_project_id: Option<String>,
    pub bigquery_dataset_id: Option<String>,
    pub bigquery_service_account_path: Option<String>,

    pub youtube_client_secret_path: Option<String>,
    pub youtube_tags: Option<String>,
    pub youtube_description_template: Option<String>,
    pub youtube_video_length_minutes_soft_cap: Option<String>,
    pub youtube_video_length_minutes_hard_cap: Option<String>,

    pub download_folder_path: Option<String>,
}

pub fn load_config() -> Config {
    trace!("load_config()");
    let config_builder: ConfigBuilder;
    let use_env;

    #[cfg(not(feature = "file"))]
    {
        use_env = true;
    }
    #[cfg(feature = "file")]
    let config_file_path: Option<String>;
    #[cfg(feature = "file")]
    {
        trace!("getting config file path from environment variable");
        config_file_path = env::var("CONFIG_FILE_PATH").ok();
        if config_file_path.is_none() {
            log::warn!("Failed to load config file path from environment variable. Using environment variables instead.");
            use_env = true;
        } else {
            trace!("found config file path: {}", config_file_path.as_ref().unwrap());
            use_env = false;
        }
    }
    if use_env {
        info!("Loading config from environment variables");
        config_builder = ConfigBuilder {
            twitch_client_id: env::var("TWITCH_CLIENT_ID").ok(),
            twitch_client_secret: env::var("TWITCH_CLIENT_SECRET").ok(),
            twitch_downloader_id: env::var("TWITCH_DOWNLOADER_ID").ok(),

            twitch_downloader_thread_count: env::var("TWITCH_DOWNLOADER_THREAD_COUNT").ok(),
            path_auth_code: env::var("PATH_AUTH_CODE").ok(),
            path_authentications: env::var("PATH_AUTHENTICATIONS").ok(),
            use_file_auth_response: env::var("USE_FILE_AUTH_RESPONSE").ok(),
            use_local_auth_redirect: env::var("USE_LOCAL_AUTH_REDIRECT").ok(),
            auth_file_read_timeout: env::var("AUTH_FILE_READ_TIMEOUT").ok(),

            bigquery_project_id: env::var("BIGQUERY_PROJECT_ID").ok(),
            bigquery_dataset_id: env::var("BIGQUERY_DATASET_ID").ok(),
            bigquery_service_account_path: env::var("BIGQUERY_SERVICE_ACCOUNT_PATH").ok(),

            youtube_client_secret_path: env::var("YOUTUBE_CLIENT_SECRET_PATH").ok(),
            youtube_tags: env::var("YOUTUBE_TAGS").ok(),
            youtube_description_template: env::var("YOUTUBE_DESCRIPTION_TEMPLATE").ok(),
            youtube_video_length_minutes_soft_cap: env::var(
                "YOUTUBE_VIDEO_LENGTH_MINUTES_SOFT_CAP",
            )
            .ok(),
            youtube_video_length_minutes_hard_cap: env::var(
                "YOUTUBE_VIDEO_LENGTH_MINUTES_HARD_CAP",
            )
            .ok(),

            download_folder_path: env::var("DOWNLOAD_FOLDER_PATH").ok(),
        };
        trace!("load_config() done loading fields from environment variables");
    } else {
        #[cfg(feature = "file")]
        {
            info!("load_config() loading fields from file");
            let config_file_path = config_file_path.expect(
                "Failed to load config file path from environment variable, \
                but still ended up in the file config loading code.",
            );
            let config_file = std::fs::read_to_string(&config_file_path).expect(&format!(
                "Failed to read config file at path: {}",
                config_file_path
            ));
            config_builder =
                serde_json::from_str(&config_file).expect("Failed to parse config file");
            trace!("load_config() done loading fields from file");
        }
        #[cfg(not(feature = "file"))]
        panic!("Failed to load config from file and environment variables");
    }
    trace!("load_config() building config");
    let config = Config {
        path_auth_code: config_builder
            .path_auth_code
            .unwrap_or("/tmp/twba/auth/code.txt".to_string()),
        use_file_auth_response: config_builder
            .use_file_auth_response
            .unwrap_or("1".to_string())
            == "1",
        path_authentications: config_builder
            .path_authentications
            .unwrap_or("/tmp/twba/auth/{user}.json".to_string()),
        use_local_auth_redirect: config_builder
            .use_local_auth_redirect
            .unwrap_or("0".to_string())
            == "1",
        auth_file_read_timeout: config_builder
            .auth_file_read_timeout
            .unwrap_or("5".to_string())
            .parse()
            .expect("AUTH_FILE_READ_TIMEOUT is not a number"),
        twitch_client_id: config_builder
            .twitch_client_id
            .expect("TWITCH_CLIENT_ID not set"),
        twitch_client_secret: config_builder
            .twitch_client_secret
            .expect("TWITCH_CLIENT_SECRET not set"),
        twitch_downloader_id: config_builder
            .twitch_downloader_id
            .unwrap_or("kimne78kx3ncx6brgo4mv6wki5h1ko".to_string()),
        twitch_downloader_thread_count: config_builder
            .twitch_downloader_thread_count
            .unwrap_or("50".to_string())
            .parse()
            .expect("TWITCH_DOWNLOADER_THREAD_COUNT is not a number"),
        bigquery_project_id: config_builder
            .bigquery_project_id
            .unwrap_or("twitchbackup-v1".to_string()),
        bigquery_dataset_id: config_builder
            .bigquery_dataset_id
            .unwrap_or("backup_data".to_string()),
        bigquery_service_account_path: config_builder
            .bigquery_service_account_path
            .unwrap_or("auth/bigquery_service_account.json".to_string()),
        youtube_client_secret_path: config_builder
            .youtube_client_secret_path
            .unwrap_or("auth/youtube_client_secret.json".to_string()),
        youtube_tags: config_builder
            .youtube_tags
            .unwrap_or("".to_string())
            .split(",")
            .map(|s| s.to_string())
            .collect(),
        youtube_description_template: config_builder
            .youtube_description_template
            .unwrap_or("test description for \"$$video_title$$\"".to_string()),
        youtube_video_length_minutes_soft_cap: config_builder
            .youtube_video_length_minutes_soft_cap
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(30i64),
        youtube_video_length_minutes_hard_cap: config_builder
            .youtube_video_length_minutes_hard_cap
            .unwrap_or("359".to_string())
            .parse()
            .unwrap_or(60i64),
        download_folder_path: config_builder
            .download_folder_path
            .unwrap_or("/tmp/twba/videos/".to_string()),
    };

    trace!("load_config() done");
    config
}
