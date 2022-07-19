// use std::{sync::Arc, io::{Error, ErrorKind}};

// use rustls::{ClientConfig, RootCertStore, OwnedTrustAnchor};
// use awc::{http::header, Client, Connector};

// static HUE_SWVER_URL: &str = "https://www.philips-hue.com/en-us/support/release-notes/bridge";

// pub async fn get_latest_swversion() -> Option<String> {
//     // TODO: Use actixweb::client
//     //#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i

//     let client_tls_config = Arc::new(rustls_config());

//     let client = Client::builder()
//             .add_default_header((header::USER_AGENT, "Mozilla/5.0 (iPhone13,2; U; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/15E148 Safari/602.1"))
//             .connector(Connector::new().rustls(Arc::clone(&client_tls_config)))
//             .finish();

//     let mut response = client.get(HUE_SWVER_URL).send().await.unwrap();

//     if !response.status().is_success() {
//         log::error!("Wikipedia did not return expected image");
//         return None
//     }


//     let payload = response
//         .body()
//         .limit(20_000_000) // 20MB
//         .await
//         .unwrap();
//         //.map_err(|e| Error::new(ErrorKind::InvalidData, format!("Payload Error: {}", e)));


//     let html_doc = scraper::Html::parse_document(std::str::from_utf8(&payload).unwrap());
//     let selector = scraper::Selector::parse("#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i").unwrap();
//     let mut res = html_doc.select(&selector);
//     let swversion = res.next().unwrap().text().next().unwrap().split(':').nth(1).unwrap().trim();
//     Some(swversion.to_string())
// }

// /// Create simple rustls client config from root certificates.
// fn rustls_config() -> ClientConfig {
//     let mut root_store = RootCertStore::empty();
//     root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
//         OwnedTrustAnchor::from_subject_spki_name_constraints(
//             ta.subject,
//             ta.spki,
//             ta.name_constraints,
//         )
//     }));

//     rustls::ClientConfig::builder()
//         .with_safe_defaults()
//         .with_root_certificates(root_store)
//         .with_no_client_auth()
// }