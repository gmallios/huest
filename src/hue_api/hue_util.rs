use awc::{http::header, Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};

static HUE_SWVER_URL: &str = "https://www.philips-hue.com/en-us/support/release-notes/bridge";

pub fn xy_to_rgb(x: f32, y: f32, _bri: u8) -> (u8, u8, u8)  {
    let Z = 1.0 - x - y;

    let R = x * 3.2406 + y * -1.5372 + Z * -0.4986;
    let G = x * -0.9689 + y * 1.8758 + Z * 0.0415;
    let B = x * 0.0557 + y * -0.2040 + Z * 1.0570;

    let R = if R <= 0.0031308 {
        12.92 * R
    } else {
        (1.0 + 0.055) * R.powf(1.0 / 2.4) - 0.055
    };

    let G = if G <= 0.0031308 {
        12.92 * G
    } else {
        (1.0 + 0.055) * G.powf(1.0 / 2.4) - 0.055
    };

    let B = if B <= 0.0031308 {
        12.92 * B
    } else {
        (1.0 + 0.055) * B.powf(1.0 / 2.4) - 0.055
    };

    let R = (R * 255.0) as u8;
    let G = (G * 255.0) as u8;
    let B = (B * 255.0) as u8;

    (R, G, B)
}

pub fn rgb_to_xy(r: u8, g: u8, b: u8) -> (f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let r = if r > 0.04045 {
        ((r + 0.055) / 1.055).powf(2.4)
    } else {
        r / 12.92
    };
    let g = if g > 0.04045 {
        ((g + 0.055) / 1.055).powf(2.4)
    } else {
        g / 12.92
    };
    let b = if b > 0.04045 {
        ((b + 0.055) / 1.055).powf(2.4)
    } else {
        b / 12.92
    };

    let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
    let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
    let z = r * 0.0193 + g * 0.1192 + b * 0.9505;

    let x = x / (x + y + z);
    let y = y / (x + y + z);

    (x, y)
}

pub async fn get_latest_swversion() -> Option<String> {
    let client = Client::builder()
            .add_default_header((header::USER_AGENT, "Mozilla/5.0 (iPhone13,2; U; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/15E148 Safari/602.1"))
            .connector(Connector::new().openssl(openssl_config()))
            //.connector(Connector::new().rustls(Arc::clone(&client_tls_config)))
            .finish();

    let mut response = client.get(HUE_SWVER_URL).send().await.unwrap();

    if !response.status().is_success() {
        log::error!("Wikipedia did not return expected image");
        return None;
    }

    let payload = response
        .body()
        .limit(20_000_000) // 20MB
        .await
        .unwrap();
    //.map_err(|e| Error::new(ErrorKind::InvalidData, format!("Payload Error: {}", e)));

    //#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i
    let html_doc = scraper::Html::parse_document(std::str::from_utf8(&payload).unwrap());
    let selector = scraper::Selector::parse(
        "#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i",
    )
    .unwrap();
    let mut res = html_doc.select(&selector);
    let swversion = res
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim();
    Some(swversion.to_string())
}

fn openssl_config() -> SslConnector {
    //TODO: Migrate to this https://github.com/alexcrichton/openssl-probe
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
