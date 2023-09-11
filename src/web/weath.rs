use crate::web::proxy::free_proxy;
// use log::warn;
use serde::Deserialize;
use rand::Rng;
use reqwest::Error;
use std::time::Instant;
use fake_useragent::UserAgents;
use reqwest::ClientBuilder;

const URL: &str = 
"https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&hourly=temperature_2m,
relativehumidity_2m,dewpoint_2m,apparent_temperature,precipitation_probability,precipitation,
rain,showers,snowfall,snow_depth,weathercode,pressure_msl,surface_pressure,cloudcover,
cloudcover_low,cloudcover_mid,cloudcover_high,visibility,evapotranspiration,
et0_fao_evapotranspiration,vapor_pressure_deficit,windspeed_10m,windspeed_80m,
windspeed_120m,windspeed_180m,winddirection_10m,winddirection_80m,winddirection_120m,
winddirection_180m,windgusts_10m,temperature_80m,temperature_120m,temperature_180m,
soil_temperature_0cm,soil_temperature_6cm,soil_temperature_18cm,soil_temperature_54cm,
soil_moisture_0_1cm,soil_moisture_1_3cm,soil_moisture_3_9cm,soil_moisture_9_27cm,
soil_moisture_27_81cm&daily=weathercode,temperature_2m_max,temperature_2m_min,
apparent_temperature_max,apparent_temperature_min,sunrise,sunset,uv_index_max,
uv_index_clear_sky_max,precipitation_sum,rain_sum,showers_sum,snowfall_sum,
precipitation_hours,precipitation_probability_max,windspeed_10m_max,
windgusts_10m_max,winddirection_10m_dominant,shortwave_radiation_sum,
et0_fao_evapotranspiration&timezone=auto&forecast_days=3";

#[derive(Debug)]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Weather{
    error: Option<bool>,
    latitude: Option<String>,
    longitude: Option<String>
}

pub async fn get_weath(ok_ls: &mut Vec<String>, dl_ls: &mut Vec<String>, time_ls: &mut Vec<String>) 
-> Result<Option<Weather>, Error>{
    let start_time = Instant::now();
    let proxy = & match !ok_ls.is_empty() {
        true => {
            ok_ls.first()
            .unwrap().clone()
        }
        false => {
            loop {
                let proxies = free_proxy().await;
                println!("Len: {}", proxies.len());
                let proxy = proxies
                .get(rand::thread_rng().gen_range(0..proxies.len()))
                .unwrap();
                if dl_ls.contains(proxy){
                    continue;
                } else {
                    break proxy.clone();
                }
            }
        }
    };

    match reqwest::Proxy::all(proxy) {
        Ok(build_proxy) => {
            match ClientBuilder::new()
                .proxy(build_proxy)
                .user_agent(
                UserAgents::new().random()
                ).build() {
                    Ok(check_client) => {
                        match check_client
                            .get(URL)
                            .send()
                            .await {
                                Ok(rsp) => {
                                    match rsp.json::<Weather>().await{
                                        Ok(resp) => {
                                            if resp.error.is_some(){
                                                time_ls.push(proxy.to_string());
                                                ok_ls.clear();
                                                println!("So much responses from 1 ip\nProxy: {}", proxy);
                                                println!("Get weather time: {:?}", start_time.elapsed());
                                                println!("Ok_ls: {:?}, Dl_ls: {:?}, Time_ls: {:?}", ok_ls, dl_ls, time_ls);
                                                return Ok(None);
                                            } else if !ok_ls.contains(proxy) {
                                                ok_ls.push(proxy.to_string());
                                                println!("Add proxy to ok_ls, successful to get resp\nProxy: {}", proxy);
                                                println!("Get weather time: {:?}", start_time.elapsed());
                                                println!("Ok_ls: {:?}, Dl_ls: {:?}, Time_ls: {:?}", ok_ls, dl_ls, time_ls);
                                                return Ok(Some(resp))
                                            } else {
                                                println!("Successful to get resp\nProxy: {}", proxy);
                                                println!("Get weather time: {:?}", start_time.elapsed());
                                                println!("Ok_ls: {:?}, Dl_ls: {:?}, Time_ls: {:?}", ok_ls, dl_ls, time_ls);
                                                return Ok(Some(resp))
                                            }
                                        },
                                        Err(e) => {
                                            dl_ls.push(proxy.to_string());
                                            println!("Failed to get resp\nProxy: {}", proxy);
                                            println!("Get weather time: {:?}", start_time.elapsed());
                                            println!("Ok_ls: {:?}, Dl_ls: {:?}, Time_ls: {:?}", ok_ls, dl_ls, time_ls);
                                            return Err(e);
                                        }
                                    }
                                },
                                Err(e) => {
                                    dl_ls.push(proxy.to_string());
                                    println!("Invalid proxy\nProxy: {}", proxy);
                                    println!("Get weather time: {:?}", start_time.elapsed());
                                    println!("Ok_ls: {:?}, Dl_ls: {:?}, Time_ls: {:?}", ok_ls, dl_ls, time_ls);
                                    return Err(e);
                                }
                            }
                        }
                    Err(e) => {
                        dl_ls.push(proxy.to_string());
                        println!("Failed to build client\nProxy: {}", proxy);
                        println!("Get weather time: {:?}", start_time.elapsed());
                        println!("Ok_ls: {:?}, Dl_ls: {:?}, Time_ls: {:?}", ok_ls, dl_ls, time_ls);
                        return Err(e);
                    }
                }
            },
            Err(e) => return Err(e)
        }
}
