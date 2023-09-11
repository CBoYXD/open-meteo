use serde::Deserialize;
use std::time::Instant;
use scraper::{Html, Selector};
use fake_useragent::UserAgents;
use reqwest::ClientBuilder;

#[derive(Deserialize)]
struct Data{
    ip: Option<String>,
    port: Option<String>
}

#[derive(Deserialize)]
struct Json{
    data: Vec<Data>
}

pub async fn free_proxy() -> Vec<String> {
    let start_time = Instant::now();
    let mut proxies: Vec<String> = Vec::new();
    let client: reqwest::Client = ClientBuilder::new()
    .user_agent(
        UserAgents::new().random()
    ).build()
    .unwrap();
    
    
    
    let resp_1 = client
        .get("https://free-proxy-list.net/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let fragment = Html::parse_document(&resp_1);

    for table in fragment.select(
        &Selector::parse("table.table.table-striped.table-bordered").unwrap()
    )  {
        for row in table.select(
            &Selector::parse("tbody tr").unwrap()
        ).skip(1) {
            let tds = row.select(
                &Selector::parse("td").unwrap()
            ).collect::<Vec<_>>();
            
            if let Some(ip) = tds.get(0){
                        if let Some(port) = tds.get(1) {
                            let ip_text = ip.text().collect::<String>().trim().to_string();
                            let port_text = port.text().collect::<String>().trim().to_string();
                            let host = format!("https://{}:{}", ip_text, port_text);
                            proxies.push(host);
                        }
                    }
                }
        }
    
    /*
    let resp_2 = Document::from(
        client
        .get("https://advanced.name/freeproxy")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .as_str()
    );
    
    let resp_3 = Document::from(
        client
        .get("http://free-proxy.cz/en/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .as_str()
    );

    let resp_4 = Document::from(
        client
        .get("https://www.freeproxy.world/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .as_str()
    );
    */
    for proxy in (
        client.get(
        "https://proxylist.geonode.com/api/proxy-list?limit=500&page=1&sort_by=lastChecked&sort_type=desc"
    ).send()
        .await
        .unwrap()
        .json::<Json>()
        .await
        .unwrap()
    ).data {
        proxies.push(format!("https://{}:{}", 
        proxy.ip.unwrap(),
        proxy.port.unwrap()
    ).to_string())
    }
    println!("Parsing time: {:?}", start_time.elapsed());
    return proxies
}