pub mod db;
mod tests;
pub mod usecase;
pub mod proto;

use aws_config::meta::region::RegionProviderChain;
use db::db_repo::{DbRepoTrait, ImplDbRepo};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use usecase::{ImplRepos, Usecase};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // let data = String::from_utf8(&).unwrap();
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let mut message = format!("Hello {who}, this is an AWS Lambda HTTP request");
    let region = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    message.push_str(&format!(
        " from {}!",
        region.region().await.unwrap().as_ref()
    ));
    // dynamodbからデータを挿入する
    let config = aws_config::from_env().load().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    //eventからバイナリデータを取得する
    let bytes = event.body().as_ref().to_vec();
    // このデータをprotobufでデコードする

    // protobufを使う関数をローカルだけでテストするには難しいから一旦デプロイしてデコードができていたらレスポンスにOKと加えて返してクライアントが確認してテスト完了
    // この関数はprotobufをデコードしてデータをdynamodbに挿入する
    //usecaseをインスタンス化してImplDbRepoをDiする
    let db_repo = ImplDbRepo::new("user".to_string());
    let implrepos = ImplRepos::new(db_repo);
    let usecase = Usecase::new(&implrepos);
    // usecase.add(user_id, user_name);

    // IntoResponse を実装したものを返す。
    // ランタイムによって自動的に正しいレスポンスイベントにシリアライズされます。
    let resp = Response::builder()
        .status(200)
        // プロトバフの場合、content-typeはapplication/x-protobufになる
        .header("content-type", "application/x-protobuf")
        // ボディにprotobufにシリアライズしたデータを入れる
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
