use lambda_runtime::{run, service_fn, LambdaEvent};
use static_init::dynamic;
use shared::queue;

mod export;


#[dynamic]
static QUEUE_URL: String = std::env::var("QUEUE_URL").expect("QUEUE_URL must be set");


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let handler = service_fn(handler);
    Ok(run(handler).await?)
}



async fn handler(event: LambdaEvent<String>) -> Result<()> {
    let dataset = event.payload.parse()?;
    let ids = export::harvest_export(dataset).await?;
    let url = queue_url();
    queue::process_ids(dataset, ids.into_iter(), &url).await?;
    Ok(())
}


fn queue_url() -> &'static str {
    QUEUE_URL.as_str()
}