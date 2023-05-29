include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/waf"
}

inputs ={
    project_name = local.env.locals.project_name
    project_stage = local.env.locals.env
    scope = "CLOUDFRONT"
    allow_country_codes = ["JP"]
    access_limit_by_country = 10000
}
