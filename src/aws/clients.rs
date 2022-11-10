use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_s3::{Client, Credentials, Region};
use aws_types::credentials::SharedCredentialsProvider;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;

use crate::settings::{AppCdnStore, SETTINGS};

lazy_static! {
  pub static ref S3_CLIENT: OnceCell<Client> = OnceCell::new();
}

pub struct AWSClient {}

impl AWSClient {
  pub async fn create_s3_client() {
    if SETTINGS.cdn.file_store != AppCdnStore::S3 {
      return;
    }

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // It takes a bit of cajoling to configure the AWS SDK from explicit credentials, probably will need to be refactored
    // at some point in the future.
    let config = match &SETTINGS.cdn.credentials {
      Some(creds) => {
        let region = region_provider
          .region()
          .await
          .unwrap_or(Region::from_static("us-east-1"));

        SdkConfig::builder()
          .region(region)
          .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            &creds.access_key,
            &creds.secret_key,
            None,
            None,
            "example",
          )))
          .build()
      }
      None => aws_config::from_env().region(region_provider).load().await,
    };

    S3_CLIENT.set(Client::new(&config)).unwrap();
  }
}
