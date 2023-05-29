include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/cloudfront"
}

# deployはrestとmethod二つのリソースに依存する

dependency "apigw_deploy"{
    config_path = "../apigw_deploy"

    mock_outputs = {
        rest_id = "rest_id"
        method_id = "method_id"
    }
}

inputs = {
    rest_api_invoke_url = dependency.apigw_deploy.outputs.rest_api_invoke_url
    project_stage = local.env.env
    project_name = local.env.project_name
    origin_name = "api_gw"
    price_class = "PriceClass_200"
    compress= false
}

