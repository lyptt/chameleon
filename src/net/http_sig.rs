use actix_web::{web::Bytes, HttpRequest};
use lazy_static::lazy_static;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use url::Url;

use crate::{model::queue_job::OriginDataEntry, settings::SETTINGS};

lazy_static! {
  pub static ref SIGNATURE_REGEX: Regex = Regex::new(r#"(.+)="(.+)""#).unwrap();
}

pub fn build_origin_data(req: &HttpRequest, bytes: &Bytes) -> Option<HashMap<String, OriginDataEntry>> {
  match SETTINGS.app.secure {
    true => {
      let headers: HashMap<String, String> = req
        .headers()
        .iter()
        .map(|(k, v)| {
          (
            k.to_string(),
            match v.to_str() {
              Ok(v) => Some(v.to_string()),
              Err(_) => None,
            },
          )
        })
        .filter(|(_k, v)| !v.is_none())
        .map(|(k, v)| (k, v.unwrap()))
        .collect();
      let method = req.method().to_string().to_uppercase();
      let path = req.path().to_string();
      let query = req.query_string().to_string();

      let mut data = HashMap::new();
      data.insert("method".to_string(), OriginDataEntry::Raw(method));
      data.insert("path".to_string(), OriginDataEntry::Raw(path));
      data.insert("query".to_string(), OriginDataEntry::Raw(query));
      data.insert("headers".to_string(), OriginDataEntry::Map(headers));

      let mut body_hasher = Sha256::new();
      body_hasher.update(bytes);
      let body_hash_digest = body_hasher.finalize();
      let body_hash_digest = base64::encode(body_hash_digest);

      let digest = format!("SHA-256={}", body_hash_digest);
      data.insert("digest".to_string(), OriginDataEntry::Raw(digest));

      Some(data)
    }
    false => None,
  }
}

pub fn verify_http_signature(context: &Option<HashMap<String, OriginDataEntry>>, public_key_pem: &str) -> bool {
  let context = match context {
    Some(ctx) => ctx,
    None => return false,
  };

  if !context.contains_key("headers") || !context.contains_key("digest") {
    return false;
  }

  let headers = match &context["headers"] {
    OriginDataEntry::Raw(_) => return false,
    OriginDataEntry::Map(data) => data,
  };

  let digest = match &context["digest"] {
    OriginDataEntry::Raw(data) => data.to_owned(),
    OriginDataEntry::Map(_) => return false,
  };

  if !headers.contains_key("signature") {
    return false;
  }

  let signature = &headers["signature"].replace(r#"\""#, r#"""#);
  let mut signature_data: HashMap<String, String> = HashMap::new();

  for component in signature.split(',') {
    for cap in SIGNATURE_REGEX.captures_iter(component) {
      signature_data.insert(cap[1].to_string(), cap[2].to_string());
    }
  }

  if !signature_data.contains_key("keyId")
    || !signature_data.contains_key("headers")
    || !signature_data.contains_key("signature")
  {
    return false;
  }

  if Url::parse(&signature_data["keyId"]).is_err() {
    return false;
  }

  false
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::{model::queue_job::OriginDataEntry, net::http_sig::verify_http_signature};

  #[test]
  pub fn verifies_signature() {
    let context: HashMap<String, OriginDataEntry> = serde_json::from_str(r#"{
      "method": {
        "Raw": "POST"
      },
      "query": {
        "Raw": ""
      },
      "path": {
        "Raw": "/api/federate/activitypub/shared-inbox"
      },
      "digest": {
        "Raw": "SHA-256=8ABV5BVEYHozFIb5xm2epgd7eb2SYQgGt4K5ndxCwx0="
      },
      "headers": {
        "Map": {
          "host": "127.0.0.1:8000",
          "content-length": "1744",
          "digest": "SHA-256=8ABV5BVEYHozFIb5xm2epgd7eb2SYQgGt4K5ndxCwx0=",
          "user-agent": "(Pixelfed/0.11.4; +https://pixelfed.test)",
          "signature": "keyId=\"https://pixelfed.test/users/boop#main-key\",headers=\"(request-target) date host accept digest content-type user-agent\",algorithm=\"rsa-sha256\",signature=\"hq60ApVjXIGbbYNX4vJujLmcrTswQHLDlUUtpGSSFuNhPrR4d/FEkDM64W8/DQZ2GKj1Gu17xOOIUMgV537S6cya3Pk5f4pwsXWjnm9B1PKEH0bOmN297o9hgq77nIeg5YAuHAecyoqlHRAfaW5eT2E5VtP9gzEGtr+SIBJ2t6vBCbmFwOg314Wa38sxZ3wdCu/m918L1kdl1eEnf9sALs74iRikMmNaorcoPoHHw40Z5nu+yAVzgnEg0eFZMYSHT8vTlbxyBHh12OGuNeVArO9tWqRD4wT7qfWMAMELtwcgYwUHm8FG4Y4Asy6h0qesVa3VRCgMbHP5i1NI/teptQ==\"",
          "accept-encoding": "deflate, gzip, br, zstd",
          "content-type": "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
          "date": "Fri, 16 Dec 2022 02:35:46 GMT",
          "accept": "application/activity+json, application/json",
          "connection": "close"
        }
      }
    }"#).unwrap();
    let public_key_pem = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmv2ro1ZJ28y9NLnCPiZ0\nhkaY34Oq8sriUBEwlhkmplpIcjR8icWahuiZJM2ILM4aY76zGfQphF8NBD1FlBp9\njLOccZwy4GqpZK7AQxNiCoRWM/qkXmSRbTYPy38WG4x2rFeA15n24hsHuC09yMx7\n+HUSioFipe1c4iDQf14HBuZ5cDfUequKjN49e/wDwN2aTB4hmlPrZVpUbYnrXu80\nlPclYi8idU89n2QposSgKSjloGKKOZjGqOjeeKzK01RDoU3cqVRdP3DBMwjTtTXe\nJordf+Z0HZ/1EqLeYXoR3O2L1ybElQrq50rEyFAVs6gLKT1PTece/m9DpvYZBau1\nDQIDAQAB\n-----END PUBLIC KEY-----\n";

    assert!(verify_http_signature(&Some(context), public_key_pem));
  }
}
