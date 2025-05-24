use lambda_runtime::{run, service_fn, LambdaEvent};
use shared::{Dataset, queue};
use static_init::dynamic;


#[dynamic]
static API_KEY: String = std::env::var("API_KEY").expect("API_KEY must be set");
#[dynamic]
static QUEUE_URL: String = std::env::var("QUEUE_URL").expect("QUEUE_URL must be set");
#[dynamic]
static INTERVAL: u8 = read_set_interval();

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

mod syncer;

#[tokio::main]
async fn main() -> Result<()> {
    let handler = service_fn(handler);
    Ok(run(handler).await?)
}


async fn handler(event: LambdaEvent<String>) -> Result<()> {
    let dataset = event.payload.parse()?;
    let days_interval = interval();
    let api_key = api_key();
    let ids = syncer::harvest_sync(dataset, days_interval, api_key).await?;
    let url = queue_url();
    queue::process_ids(dataset, ids.into_iter(), &url).await?;
    Ok(())
}


fn api_key() -> &'static str {
    API_KEY.as_str()
}

fn queue_url() -> &'static str {
    QUEUE_URL.as_str()
}

fn read_set_interval() -> u8 {
    match std::env::var("SET_INTERVAL") {
        Ok(interval) => interval.parse::<u8>().unwrap_or(1),
        Err(_) => 1,
    }
}

fn interval() -> u8 {
    *INTERVAL
}