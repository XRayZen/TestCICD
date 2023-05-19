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
  region = "us-east-1"
}

resource "aws_s3_bucket" "terraform_state_bucket" {
  bucket = "terraform-state-test-cicd"
  tags = {
    Name = "terraform-state-test-cicd"
  }
}

resource "aws_s3_bucket_versioning" "terraform_state-bucket-versioning" {
  bucket = aws_s3_bucket.terraform_state_bucket.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_dynamodb_table" "terraform_state_lock" {
  name           = "terraform_state_lock"
  read_capacity  = 1
  write_capacity = 1
  hash_key       = "LockID"
  attribute {
    name = "LockID"
    type = "S"
  }
}



