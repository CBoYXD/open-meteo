mod  web;
use web::weath::get_weath;

#[tokio::main]
async fn main(){
    let mut ok_ls: Vec<String> = Vec::new();
    let mut time_ls: Vec<String> = Vec::new();
    let mut dl_ls: Vec<String> = Vec::new();
    for i in 0..=100{
        match get_weath(&mut ok_ls, &mut dl_ls, &mut time_ls).await{
            Ok(resp) => {
                if resp.is_some(){
                    println!("{:#?}", resp.unwrap())
                }
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
        println!("Req №{}", i);
    }
}