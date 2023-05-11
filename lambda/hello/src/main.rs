pub mod app;
pub mod db;
pub mod gen_proto;
pub mod proto;
pub mod repo_trait;
mod tests;
pub mod usecase;

use app::repo_trait::DbRepoTrait;
use db::impl_db_repo::ImplDbRepo;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use repo_trait::ImplRepoTrait;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    //eventからバイナリデータを取得する
    let bytes = event.body().as_ref();
    // protobufを使う関数をローカルだけでテストするには難しいから一旦デプロイしてデコードができていたらレスポンスにOKと加えて返してクライアントが確認してテスト完了
    // この関数はprotobufをデコードしてデータをdynamodbに挿入する
    //usecaseをインスタンス化してImplDbRepoをDiする
    let db_repo = ImplDbRepo::new("user".to_string());
    let implrepos = ImplRepoTrait::new(db_repo);
    // IntoResponse を実装したものを返す。
    // ランタイムによって自動的に正しいレスポンスイベントにシリアライズされます
    match proto::proto_process(&bytes, &implrepos).await {
        Ok(txt) => return Ok(txt),
        Err(err) => {
            eprintln!("Failed to parse request: {:?}", err);
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Bad request"))
                .unwrap());
        }
    }
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
