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
        rest_api_invoke_url = "rest_api_invoke_url"
        api_gw_stage_name = "api_gw_stage_name"
    }
}

dependency "waf"{
    config_path = "../waf"

    mock_outputs = {
        waf_arn = "waf_arn"
    }
}

inputs = {
    rest_api_invoke_url = dependency.apigw_deploy.outputs.rest_api_invoke_url
    waf_arn= dependency.waf.outputs.waf_arn
    project_stage = local.env.locals.env
    project_name = local.env.locals.project_name
    origin_name = "api_gw"
    price_class = "PriceClass_200"
    compress= false
}

