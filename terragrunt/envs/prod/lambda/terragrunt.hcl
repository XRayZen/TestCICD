include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "${dirname(find_in_parent_folders())}//modules/lambda"
}

dependency "ecr" {
    config_path = "${get_env("env")}/lambda.hcl"
}

