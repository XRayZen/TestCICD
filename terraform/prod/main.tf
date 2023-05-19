
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
  backend "s3" {
    bucket         = "terraform-state-test-cicd"
    key            = "terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    dynamodb_table = "terraform_state_lock"
  }
}

# Provider configuration
provider "aws" {
  alias = "virginia"
  # CloudFrontを使う場合はAWSリソースのリージョンをus-east-1にする必要がある
  region = var.aws_region
}