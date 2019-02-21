#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
//extern crate simple_logger;
extern crate simple_error;

use simple_error::bail;
use lambda::error::HandlerError;
use serde_json::{Map, Value};


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
    resources: Vec<String>, 
    detail: Map<String, Value>,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() {
    //simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
}

fn my_handler(e: CloudWatchEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.version == "" {
        bail!("Empty version number");
    }
    Ok(CustomOutput {
        message: format!("Detail-type, {}!", e.detail_type),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_de() {
        //Test deserialization works
        let cwe = r#"
             {
            "version": "0",
            "id": "6a7e8feb-b491-4cf7-a9f1-bf3703467718",
            "detail-type": "EC2 Instance State-change Notification",
            "source": "aws.ec2",
            "account": "111122223333",
            "time": "2017-12-22T18:43:48Z",
            "region": "us-west-1",
            "resources": [
                "arn:aws:ec2:us-west-1:123456789012:instance/ i-1234567890abcdef0"
            ],
            "detail": {
                "instance-id": " i-1234567890abcdef0",
                "state": "terminated"
            }
            } "#;
        let event: CloudWatchEvent = serde_json::from_str(cwe).unwrap();
        println!("detail is: {:?}", event.detail);
        //Ok(());
    }
}
