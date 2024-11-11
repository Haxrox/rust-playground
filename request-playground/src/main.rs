use::reqwest;
use::serde::Deserialize;
use::chrono::DateTime;

// #[tokio::main]
// async fn main() -> Result<()> {
#[derive(Deserialize, Debug)]
struct TimeResp {
    utc_offset : String,
    timezone : String,
    day_of_week : u32,
    day_of_year : u32,
    datetime: String,
    utc_datetime : String,
    unixtime : u32,
    raw_offset : i32,
    week_number: u32,
    dst: bool,
    abbreviation: String,
    dst_offset: i32,
    dst_from: Option<i32>,
    dst_until: Option<i32>,
    client_ip: String,
}

fn main() {
    println!("Hello, world!");
    
    let result = reqwest::blocking::get("http://worldtimeapi.org/api/timezone/America/Vancouver");
    
    if result.is_err() {
        println!("Error: {:?}", result.err());
        return;
    }

    let res = result.unwrap();
    println!("Response: {:?}", res);

    // let mut body = String::new();
    // res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{:#?}", res.text());

    let json_opt = res.json::<TimeResp>();
    println!("Body:\n{:#?}", json_opt);

    if json_opt.is_err() {
        println!("Error: {:?}", json_opt.err());
        return;
    }

    let time_resp : TimeResp = json_opt.unwrap();
    println!("The current date is: {}", DateTime::parse_from_rfc3339(&time_resp.datetime).unwrap().format("%Y-%m-%d at %H:%M:%S"));

    // match result {
        //     Ok(response) => {
            //         println!("Response: {:?}", response);
    //     },
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //     }
    // }
}