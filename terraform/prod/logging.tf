
###############################################
# CloudFrontのアクセスログ格納用バケットポリシー
###############################################
data "aws_iam_policy_document" "cloudfront_logging_bucket" {
  statement {
    sid    = ""
    effect = "Allow"

    principals {
      identifiers = ["*"]
      type        = "*"
    }

    actions = [
      "s3:ListBucket",
      "s3:PutObject",
      "s3:GetObject"
    ]

    resources = [
      "arn:aws:s3:::cloudfront-access-log.mini-schna.com",
      "arn:aws:s3:::cloudfront-access-log.mini-schna.com/*"
    ]
  }
}

###############################################
# CloudFrontのアクセスログ格納用バケット
###############################################
resource "aws_s3_bucket" "cloudfront_logging" {
  bucket = "cloudfront-access-log.mini-schna.com"
  # policy        = data.aws_iam_policy_document.cloudfront_logging_bucket.json
  force_destroy = false
}

# S3 Public Access Block
## パブリックアクセスをすべて許可する
resource "aws_s3_bucket_public_access_block" "cloudfront_logging" {
  bucket                  = aws_s3_bucket.cloudfront_logging.bucket
  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_lifecycle_configuration" "log_bucket_life" {
  bucket = aws_s3_bucket.cloudfront_logging.id
  rule {
    id     = "assets"
    status = "Enabled"
    ## オブジェクトの保存期限。
    expiration {
      days = "365" ## 1年
    }
    ## 現在のオブジェクトの移行設定。
    transition {
      ## オブジェクトが作成されてから移行するまでの日数。
      days          = "93" ## 3ヶ月
      storage_class = "STANDARD_IA"
    }
    ## オブジェクトの以前のバージョンの保存期限。
    noncurrent_version_expiration {
      noncurrent_days = "1095" ## 3年
    }
    ## 古いのオブジェクトの移行設定。
    noncurrent_version_transition {
      ## オブジェクトが古いバージョンになってから移行するまでの日数。
      noncurrent_days = "365" ## 1年
      storage_class   = "GLACIER"
    }
  }
}

resource "aws_s3_bucket_versioning" "log_bucket_versioning" {
  bucket = aws_s3_bucket.cloudfront_logging.id
  versioning_configuration {
    status     = "Enabled"
    mfa_delete = "Disabled"
  }
}

resource "aws_s3_bucket_request_payment_configuration" "log_bucket_request_payment" {
  bucket = aws_s3_bucket.cloudfront_logging.id
  payer  = "BucketOwner"
}
# CloudFrontのアクセスログ格納用バケットポリシー
resource "aws_s3_bucket_policy" "log_bucket_policy" {
  bucket = aws_s3_bucket.cloudfront_logging.id
  # "s3:ListBucket" "s3:PutObject"  "s3:GetObject"
  # "Service": "cloudfront.amazonaws.com"
  policy = <<POLICY
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowCloudFrontToPutLogs",
      "Effect": "Allow",
      "Principal": {
        "*": "*"
      },
      "Action": "s3:PutObject",
      "Resource": "arn:aws:s3:::cloudfront-access-log.mini-schna.com/*"
    },
    {
      "Sid": "AllowCloudFrontToGetLogs",
      "Effect": "Allow",
      "Principal": {
        "*": "*"
      },
      "Action": "s3:GetObject",
      "Resource": "arn:aws:s3:::cloudfront-access-log.mini-schna.com/*"
    },
    {
      "Sid": "AllowCloudFrontToDescribeBucket",
      "Effect": "Allow",
      "Principal": {
        "*": "*"
      },
      "Action": "s3:ListBucket",
      "Resource": "arn:aws:s3:::cloudfront-access-log.mini-schna.com"
    },
    {
      "Sid": "AddPerm",
      "Effect": "Allow",
      "Principal": {
        "Service": "cloudfront.amazonaws.com"
      },
      "Action": "s3:PutObject",
      "Resource": "arn:aws:s3:::mybucket/*",
      "Condition": {
        "StringEquals": {
          "s3:x-amz-acl": "bucket-owner-full-control"
        }
      }
    }
  ]
}
POLICY
}
