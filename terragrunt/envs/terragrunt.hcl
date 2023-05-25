remote_state {
  backend = "s3"
  config = {
    bucket = "terraform-state-remote"
    key    = "test-cicd/${path_relative_to_include()}.tfstate"
    region = "us-east-1"
  }
  generate = {
    path      = "backend.tf"
    if_exists = "overwrite"
  }
}

generate "provider" {
  path      = "_provider.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}
provider "aws" {
        alias = "virginia"
        # CloudFrontを使う場合はAWSリソースのリージョンをus-east-1にする必要がある
        region = "us-east-1"
        default_tags {
                tags = {
                        env = "dev"
                        ManagedBy = "Terraform"
                }
        }
    }
EOF
}
