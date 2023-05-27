use lambda_http::{
    aws_lambda_events::serde_json::ser, run, service_fn, Body, Error, Request, RequestExt, Response,
};

/// 機能の本体となります。
/// その中にコードを書く。
/// 以下のURLにコード例があります：
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let message = format!("Hello this is an AWS Lambda HTTP request");
    // let ddd= ser::to_string(&message).unwrap();
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
