use actix_web::HttpRequest;
use http::HeaderMap;
use http_signing::{Key, PublicKey, Signature};
use std::collections::HashMap;

use crate::{model::queue_job::OriginDataEntry, settings::SETTINGS};

pub fn build_origin_data(req: &HttpRequest) -> Option<HashMap<String, OriginDataEntry>> {
  match SETTINGS.app.secure {
    true => {
      let mut headers: HashMap<String, String> = req
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

      // HACK: With some reverse proxy configurations the host header we receive is incorrect, so to work around
      //       this we have to swap out the host with our known API FQDN which will allow the signature to match.
      if headers.contains_key("host") {
        headers.remove("host");
        let host_without_domain = match SETTINGS.server.api_root_fqdn.split("://").last() {
          Some(h) => h.to_owned(),
          None => return None,
        };
        headers.insert("host".to_owned(), host_without_domain);
      }
      let method = req.method().to_string().to_uppercase();
      let path = req.path().to_string();
      let query = req.query_string().to_string();

      let mut data = HashMap::new();
      data.insert("method".to_string(), OriginDataEntry::Raw(method));
      data.insert("path".to_string(), OriginDataEntry::Raw(path));
      data.insert("query".to_string(), OriginDataEntry::Raw(query));
      data.insert("headers".to_string(), OriginDataEntry::Map(headers));

      Some(data)
    }
    false => None,
  }
}

pub fn verify_http_signature(context: &Option<HashMap<String, OriginDataEntry>>, public_key_pem: &str) -> bool {
  let key = match PublicKey::from_pem(public_key_pem.as_bytes()) {
    Ok(k) => k,
    Err(_) => return false,
  };

  let context = match context {
    Some(ctx) => ctx,
    None => return false,
  };

  if !context.contains_key("headers") || !context.contains_key("path") || !context.contains_key("method") {
    return false;
  }

  let headers = match &context["headers"] {
    OriginDataEntry::Raw(_) => return false,
    OriginDataEntry::Map(data) => data,
  };

  let path = match &context["path"] {
    OriginDataEntry::Raw(data) => data,
    OriginDataEntry::Map(_) => return false,
  };

  let query = match context.contains_key("query") {
    true => match &context["query"] {
      OriginDataEntry::Raw(data) => Some(data.to_owned()),
      OriginDataEntry::Map(_) => return false,
    },
    false => None,
  };

  let method = match &context["method"] {
    OriginDataEntry::Raw(data) => data.to_lowercase(),
    OriginDataEntry::Map(_) => return false,
  };

  let request_target = match query {
    None => format!("{} {}", method, path),
    Some(query) => {
      if query.is_empty() {
        format!("{} {}", method, path)
      } else {
        format!("{} {}?{}", method, path, query)
      }
    }
  };

  let headers: HeaderMap = match headers.try_into() {
    Ok(v) => v,
    Err(_) => return false,
  };

  let sig = Signature::builder()
    .request_target(request_target)
    .headers(headers)
    .build();

  sig.verify(&key).unwrap_or(false)
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
      "headers": {
        "Map": {
          "host": "chameleon.test",
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

  #[test]
  pub fn verifies_signature_2() {
    let context: HashMap<String, OriginDataEntry> = serde_json::from_str(r#"{
      "method": {
        "Raw": "POST"
      },
      "query": {
        "Raw": ""
      },
      "headers": {
        "Map": {
          "content-type": "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
          "accept-encoding": "deflate, gzip, br, zstd",
          "digest": "SHA-256=7fmXGROnmCtlxnElMHv/KD9Pyu8BdXwJ+Btx/mCLgCM=",
          "signature": "keyId=\"https://pixelfed.test/users/boop#main-key\",headers=\"(request-target) date host accept digest content-type user-agent\",algorithm=\"rsa-sha256\",signature=\"ZLHbOtH3Sb1kNKpi/lz5A4i8dhW07EsHs/cEE/MD21ojcayNHbqQV2zp2UowsevLWe2y2GgdwB54DwGudBgwVV6LA/Ewx8My6Axai3jVWKmeXGQZflUfxCrg0L4KpACfAdfWhXDLkFDR8/v5Qk3ZR3LI4DyJhv0eWWzgmkfKIssaJXqSph3aqF5e+UzmkVNRVpltSxIY9yUU1MA/YCqncd2LMnEqWwWihj93i3mtU4wdJc8ks+P2pb0OcWurEST78nz9ExBTXKnEE/MbzpZnNoeDhC6wcqY5AJatE3Olo/MkNxjw4ErncROFfFnchSrkz5byVaWwebZW+gASHEU0BQ==\"",
          "user-agent": "(Pixelfed/0.11.4; +https://pixelfed.test)",
          "connection": "close",
          "date": "Thu, 22 Dec 2022 01:33:39 GMT",
          "accept": "application/activity+json, application/json",
          "host": "chameleon.test",
          "content-length": "1744"
        }
      },
      "path": {
        "Raw": "/api/federate/activitypub/shared-inbox"
      }
    }"#).unwrap();
    let public_key_pem = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmv2ro1ZJ28y9NLnCPiZ0\nhkaY34Oq8sriUBEwlhkmplpIcjR8icWahuiZJM2ILM4aY76zGfQphF8NBD1FlBp9\njLOccZwy4GqpZK7AQxNiCoRWM/qkXmSRbTYPy38WG4x2rFeA15n24hsHuC09yMx7\n+HUSioFipe1c4iDQf14HBuZ5cDfUequKjN49e/wDwN2aTB4hmlPrZVpUbYnrXu80\nlPclYi8idU89n2QposSgKSjloGKKOZjGqOjeeKzK01RDoU3cqVRdP3DBMwjTtTXe\nJordf+Z0HZ/1EqLeYXoR3O2L1ybElQrq50rEyFAVs6gLKT1PTece/m9DpvYZBau1\nDQIDAQAB\n-----END PUBLIC KEY-----\n";

    assert!(verify_http_signature(&Some(context), public_key_pem));
  }
}
