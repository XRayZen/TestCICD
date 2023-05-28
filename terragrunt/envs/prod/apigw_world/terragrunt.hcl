
include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/apigw_method"
}

dependency "lambda_world" {
    config_path = "../lambda_world"
}

dependency "apigw_rest"{
    config_path = "../apigw_rest"
}

inputs ={
    # 依存先からパラメーターを引き継ぐ
    # apigw
    api_gw_rest_api_id = dependency.apigw_rest.outputs.api_gw_rest_api_id
    api_gw_root_resource_id = dependency.apigw_rest.outputs.api_gw_root_resource_id
    rest_api_execution_arn = dependency.apigw_rest.outputs.rest_api_execution_arn
    # lambda function
    lambda_function_invoke_arn= dependency.lambda_world.outputs.lambda_function_invoke_arn
    lambda_function_name= dependency.lambda_world.outputs.lambda_function_name
    # パラメーター
    lambda_permission_source_arn_with_path = "/*" # CloudFrontからなら/*/*/* とりあえず今はこれで
    resource_last_path_part = "world" # ここにはパスの最後の部分に関数名を入れる
    http_method = "POST"
    api_gw_integration_content_handling = "CONVERT_TO_TEXT" # jsonの場合はCONVERT_TO_TEXT
}


