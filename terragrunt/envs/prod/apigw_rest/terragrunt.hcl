include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/apigw_rest"
}

inputs ={
    project_name = local.env.locals.project_name
    project_stage = local.env.locals.env
    api_gw_description = "API Gateway for ${local.env.locals.project_name} stage:${local.env.locals.env}"
}