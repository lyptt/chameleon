use std::collections::HashMap;

use actix_web::HttpRequest;

use crate::{model::queue_job::OriginDataEntry, settings::SETTINGS};

pub fn build_origin_data(req: &HttpRequest) -> Option<HashMap<String, OriginDataEntry>> {
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

  false
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::{model::queue_job::OriginDataEntry, net::http_sig::verify_http_signature};

  #[test]
  pub fn verifies_signature() {
    let context: HashMap<String, OriginDataEntry> = serde_json::from_str(r#"{
      "path": {
        "Raw": "/api/federate/activitypub/shared-inbox"
      },
      "method": {
        "Raw": "POST"
      },
      "headers": {
        "Map": {
          "signature": "keyId=\"https://pixelfed.test/users/boop#main-key\",headers=\"(request-target) date host accept digest content-type user-agent\",algorithm=\"rsa-sha256\",signature=\"UTL8/WHagh8zgK9GoGHhHchb/gQhmkc8CU0OKdAhbRGgfJRru70KNLwZo494u7rSX+4OQeTZEBZLZmex6w7VJmSsfrOkvD+Mv5dkkNjUp7eSXV13uNNpPSekbH7lBvc4REOLxtuzHBs62V1L+2mdb8xdmIX9O9WRXlefM2ByZA/ejN1Sa/6uZxjwUNAehdVIPzdPbqDCivZs5lvOfP1YmO8NqONwLCsMa/ujh1mOHqNpbF/dLNoK4xUn/BJ5vO64J9yFthTN4exa3ipLfzQPfH4lk2AYEAFPWCKtbSCtHVzIUY4nWmmfwSLXYxypQ13EJeIMUJXQvkr3RFqYZxDs2A==\"",
          "accept-encoding": "deflate, gzip, br, zstd",
          "date": "Fri, 16 Dec 2022 01:33:14 GMT",
          "content-type": "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"",
          "host": "127.0.0.1:8000",
          "accept": "application/activity+json, application/json",
          "digest": "SHA-256=QlhkztqVNq2egSxD5vMkLtM+LQ4KuovTBzQ1JECVJVg=",
          "content-length": "1744",
          "connection": "close",
          "user-agent": "(Pixelfed/0.11.4; +https://pixelfed.test)"
        }
      },
      "query": {
        "Raw": ""
      }
    }"#).unwrap();
    let public_key_pem = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmv2ro1ZJ28y9NLnCPiZ0\nhkaY34Oq8sriUBEwlhkmplpIcjR8icWahuiZJM2ILM4aY76zGfQphF8NBD1FlBp9\njLOccZwy4GqpZK7AQxNiCoRWM/qkXmSRbTYPy38WG4x2rFeA15n24hsHuC09yMx7\n+HUSioFipe1c4iDQf14HBuZ5cDfUequKjN49e/wDwN2aTB4hmlPrZVpUbYnrXu80\nlPclYi8idU89n2QposSgKSjloGKKOZjGqOjeeKzK01RDoU3cqVRdP3DBMwjTtTXe\nJordf+Z0HZ/1EqLeYXoR3O2L1ybElQrq50rEyFAVs6gLKT1PTece/m9DpvYZBau1\nDQIDAQAB\n-----END PUBLIC KEY-----\n";

    assert!(verify_http_signature(&Some(context), public_key_pem));
  }
}
