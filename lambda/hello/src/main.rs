pub mod db;
pub mod gen_proto;
pub mod repo_trait;
mod tests;
pub mod usecase;

use db::db_repo::{DbRepoTrait, ImplDbRepo};
use gen_proto::hello_req::HelloRequest;
use lambda_http::{run, service_fn, Body, Error, Request,  Response};
use protobuf::{Message, EnumOrUnknown};
use repo_trait::ImplRepos;
use usecase::Usecase;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // dynamodbからデータを挿入する
    let config = aws_config::from_env().load().await;
    //eventからバイナリデータを取得する
    let bytes = event.body().as_ref();
    // このデータをprotobufでデコードする
    let request: HelloRequest = match protobuf::Message::parse_from_bytes(bytes) {
        Ok(request) => request,
        Err(e) => {
            eprintln!("Failed to parse request: {:?}", e);
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("Bad request"))
                .unwrap());
        }
    };
    // protobufを使う関数をローカルだけでテストするには難しいから一旦デプロイしてデコードができていたらレスポンスにOKと加えて返してクライアントが確認してテスト完了
    // この関数はprotobufをデコードしてデータをdynamodbに挿入する
    //usecaseをインスタンス化してImplDbRepoをDiする
    let db_repo = ImplDbRepo::new("user".to_string());
    let implrepos = ImplRepos::new(db_repo);
    let usecase = Usecase::new(&implrepos);
    match request.api_req_type.enum_value() {
        Ok(i) => match i {
            gen_proto::hello_req::ApiReqType::API_REQ_TYPE_UNSPECIFIED => todo!(),
            gen_proto::hello_req::ApiReqType::API_REQ_TYPE_GET_USER => {
                let mut helloRes = gen_proto::hello_res::HelloResponse::new();
                let mut get_user_response = gen_proto::hello_res::GetUserResponse::new();
                get_user_response.message = usecase.get(&request.get_user_name_request().user_id).await?;
                helloRes.set_get_user_response(get_user_response);
                helloRes.api_res_type = EnumOrUnknown::new(gen_proto::hello_res::ApiResType::API_RES_TYPE_SUCCESS);
                let response_data= vec![];
                match helloRes.write_to_bytes() {
                    Ok(data) => {
                        response_data = data;
                    },
                    Err(err) => {
                        println!("err: {:?}", err);
                    }  
                }
                return Ok(Response::builder()
                    .status(200)
                    .header("content-type", "application/x-protobuf")
                    .body(response_data.into())
                    .map_err(Box::new)?);
            },
            gen_proto::hello_req::ApiReqType::API_REQ_TYPE_ADD_USER => todo!(),
        },
        Err(err) => todo!(),
    }
    //  usecase.add(user_id, user_name);

    // IntoResponse を実装したものを返す。
    // ランタイムによって自動的に正しいレスポンスイベントにシリアライズされます。
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
