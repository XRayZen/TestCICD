
variable "vpc_cidr_block" {
  description = "CIDR Block"
  default     = "10.10.0.0/16"
}

variable "aws_region" {
  description = "AWS Region"
  default     = "us-east-1"
}

variable "project_name" {
  description = "Project Name"
  # ECRのリポジトリ名に使われるので、小文字のみを使う
  default     = "test-cicd"
}

variable "project_stage" {
  description = "Project Stage"
  default     = "prod"
}

variable "project_tag" {
  description = "Project Tag"
  default     = "latest"
}

variable "project_description" {
  description = "Project Description"
  default     = "Test CICD CloudFront+ApiGateWay+Lamdba+Rust+Docker+ECR+S3 Terraform Project"
}

variable "project_owner" {
  description = "Project Owner"
  default     = "Matagi"
}
# 作成するサブネットに合わせて、アベイラビリティゾーンリストを定義
variable "availability_zones" {
  type = list(string)
  # us-east-1a, us-east-1b, us-east-1c
  default = ["us-east-1a", "us-east-1b", "us-east-1c"]
}

variable "notification_emails" {
  type    = list(string)
  default = ["megaon1fight0123456@gmail.com"]
}
