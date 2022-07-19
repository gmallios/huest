pub fn get_latest_swversion() -> Result<String, ureq::Error> {
    // TODO: Use actixweb::client
    //#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i

    let body = ureq::get("https://www.philips-hue.com/en-us/support/release-notes/bridge")
        .set("User-Agent", "Mozilla/5.0 (iPhone13,2; U; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/15E148 Safari/602.1")
        .call()?
        .into_string()?;

    let html_doc = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("#experience-fragment-component-b9c8b63f00 > div > div > div > div > p:nth-child(1) > i").unwrap();
    let mut res = html_doc.select(&selector);
    let swversion = res.next().unwrap().text().next().unwrap().split(':').nth(1).unwrap().trim();
    Ok(swversion.to_string())
}