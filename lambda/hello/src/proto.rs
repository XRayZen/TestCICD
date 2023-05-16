use protobuf::{EnumOrUnknown, Message};

use crate::{
    gen_proto::{self, hello_req::HelloRequest},
    repo_trait::{ RepoTrait},
    usecase::Usecase,
};
use lambda_http::{Body, Response};

pub async fn proto_process<'a, T: RepoTrait>(
    data: &[u8],
    repo: &'a T,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let request: HelloRequest = protobuf::Message::parse_from_bytes(data)?;
    let usecase = Usecase::new(repo);
    match request.api_req_type.enum_value() {
        Ok(i) => match i {
            gen_proto::hello_req::ApiReqType::API_REQ_TYPE_UNSPECIFIED => todo!(),
            gen_proto::hello_req::ApiReqType::API_REQ_TYPE_GET_USER => {
                return get_user_process(request, usecase).await;
            }
            gen_proto::hello_req::ApiReqType::API_REQ_TYPE_ADD_USER => {
                return add_user_process(request, usecase).await;
            }
        },
        Err(err) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from(format!("Failed to parse request: {:?}", err)))?);
        }
    }
}

async fn get_user_process<'a, T: RepoTrait>(
    request: HelloRequest,
    usecase: Usecase<'a, T>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut hello_res = gen_proto::hello_res::HelloResponse::new();
    let mut get_user_response = gen_proto::hello_res::GetUserResponse::new();
    get_user_response.message = usecase
        .get(&request.get_user_name_request().user_id)
        .await?;
    hello_res.set_get_user_response(get_user_response);
    hello_res.api_res_type =
        EnumOrUnknown::new(gen_proto::hello_res::ApiResType::API_RES_TYPE_SUCCESS);
    let response_data = hello_res.write_to_bytes()?;
    return Ok(Response::builder()
        .status(200)
        .header("content-type", "application/x-protobuf")
        .body(response_data.into())
        .map_err(Box::new)?);
}

async fn add_user_process<'a, T: RepoTrait>(
    request: HelloRequest,
    usecase: Usecase<'a, T>,
) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut hello_res = gen_proto::hello_res::HelloResponse::new();
    let mut add_user_response = gen_proto::hello_res::AddUserResponse::new();
    add_user_response.message = usecase
        .add_user(
            &request.add_user_request().user_id,
            &request.add_user_request().user_name,
        )
        .await?;
    hello_res.set_add_user_response(add_user_response);
    hello_res.api_res_type =
        EnumOrUnknown::new(gen_proto::hello_res::ApiResType::API_RES_TYPE_SUCCESS);
    let response_data = hello_res.write_to_bytes()?;
    return Ok(Response::builder()
        .status(200)
        .header("content-type", "application/x-protobuf")
        .body(response_data.into())
        .map_err(Box::new)?);
}
