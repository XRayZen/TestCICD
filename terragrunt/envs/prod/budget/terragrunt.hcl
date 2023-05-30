include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/budget"
}

inputs = {
    budget_name = "test-cicd-budget"
    limit_amount = 7
    time_unit = "MONTHLY"
    threshold = 20
    notification_emails = ["megaon1fight0123456@gmail.com"]
}