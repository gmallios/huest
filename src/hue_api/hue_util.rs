use std::{sync::Arc, io::{Error, ErrorKind}};

use awc::{http::header, Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};

static HUE_SWVER_URL: &str = "https://www.philips-hue.com/en-us/support/release-notes/bridge";

pub async fn get_latest_swversion() -> Option<String> {
    
    let client = Client::builder()
            .add_default_header((header::USER_AGENT, "Mozilla/5.0 (iPhone13,2; U; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/15E148 Safari/602.1"))
            .connector(Connector::new().openssl(openssl_config()))
            //.connector(Connector::new().rustls(Arc::clone(&client_tls_config)))
            .finish();

    let mut response = client.get(HUE_SWVER_URL).send().await.unwrap();

    if !response.status().is_success() {
        log::error!("Wikipedia did not return expected image");
        return None
    }


    let payload = response
        .body()
        .limit(20_000_000) // 20MB
        .await
        .unwrap();
        //.map_err(|e| Error::new(ErrorKind::InvalidData, format!("Payload Error: {}", e)));

    //#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i
    let html_doc = scraper::Html::parse_document(std::str::from_utf8(&payload).unwrap());
    let selector = scraper::Selector::parse("#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i").unwrap();
    let mut res = html_doc.select(&selector);
    let swversion = res.next().unwrap().text().next().unwrap().split(':').nth(1).unwrap().trim();
    Some(swversion.to_string())
}

fn openssl_config() -> SslConnector {
    //https://stackoverflow.com/a/62729715
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file("./ssl/cacert.pem").unwrap();
    builder.build()
}


#[cfg(test)]
mod hue_util {


    #[actix_rt::test]
    async fn swversion_fetch() {
        let swver = crate::hue_api::hue_util::get_latest_swversion().await;
        assert!(swver.is_some());
    }
}