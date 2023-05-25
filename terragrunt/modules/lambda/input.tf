
variable "lambda_function_name" {
    type = string
    description = "lambda function name"
}

variable "lambda_runtime" {
    type = string
    description = "lambda runtime"
}

variable "lambda_function_description" {
    type = string
    description = "lambda function description"
}

variable "image_url" {
    type = string
    # タグが付いているイメージURLを指定する
    description = "image url with tag (e.g. xxx.dkr.ecr.region.amazonaws.com/test-cicd-repo:tag)"
}

variable "memory_size" {
    type = number
    description = "memory size"
}

variable "timeout" {
    type = number
    description = "timeout"
}

variable "environment_variables" {
    type = map(string)
    description = "environment variables"
}

variable "subnet_ids" {
    type = list(string)
    description = "subnet ids"
}

variable "security_group_ids" {
    type = list(string)
    description = "security group ids"
}





