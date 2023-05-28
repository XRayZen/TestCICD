include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/apigw_deploy"
}

# deployはrestとmethod二つのリソースに依存する
dependency "rest" {
    config_path = "../apigw_rest"
}

dependency "method_world" {
    config_path = "../apigw_world"
}

inputs ={
    api_gw_rest_api_id = dependency.rest.outputs.api_gw_rest_api_id
    api_gw_resource_path = dependency.rest.outputs.api_gw_resource_path
    # パラメータ
    project_name = local.env.locals.project_name
    project_stage = local.env.locals.env
    quota_limit = 1000
    quota_offset = 0
    quota_period = "WEEK"
    throttle_burst_limit = 1000
    throttle_rate_limit = 1000
}