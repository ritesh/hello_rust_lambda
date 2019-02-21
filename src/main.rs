#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Deserialize, Clone)]
struct CloudWatchEvent {
    version: String,
    id: String,
    #[serde(rename="detail-type")]
    detail_type: String,
    source: String,
    account: String,
    time: String,
    region: String,
    //TODO:
    //resources: 
    //detail
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CloudWatchEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.version == "" {
        error!("Empty version number in {}", c.aws_request_id);
        return Err(c.new_error("No version number in event"));
    }

    Ok(CustomOutput {
        message: format!("Detail-type, {}!", e.detail_type),
    })
}
