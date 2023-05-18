
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

# Provider configuration
provider "aws" {
  alias = "virginia"
  # CloudFrontを使う場合はAWSリソースのリージョンをus-east-1にする必要がある
  region  = var.aws_region
}
