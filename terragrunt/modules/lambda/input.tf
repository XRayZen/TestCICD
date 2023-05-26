
variable "lambda_function_name" {
    type = string
    description = "lambda function name"
}

variable "lambda_function_description" {
    type = string
    description = "lambda function description"
}

variable "repo_url" {
    type = string
    description = "ecr url"
}

variable "image_tag" {
    type = string
    # タグを指定する
    description = "image tag (e.g. latest)"
}

variable "memory_size" {
    type = number
    description = "memory size"
}

variable "timeout" {
    type = number
    description = "timeout"
}

variable "managed_policy_arns" {
    type = set(string)
    description = "managed policy arns"
}
# VPCを設定するまでは使わない
# variable "environment_variables" {
#     type = map(string)
#     description = "environment variables"
# }

# variable "subnet_ids" {
#     type = list(string)
#     description = "subnet ids"
# }

# variable "security_group_ids" {
#     type = list(string)
#     description = "security group ids"
# }





