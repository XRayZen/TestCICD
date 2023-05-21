

resource "aws_iam_role" "lambda_execution_role" {
  name = "lambda_execution_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}
# やっぱりJsonで定義した方がフレキシブルだし、可読性も高い
# VPC用のaws_iam_policy_documentを定義する
data "aws_iam_policy_document" "lambda_access_policy" {
  statement {
    effect = "Allow"
    actions = [
      # VPC用iam_policy
      # AWSLambdaVPCAccessExecutionRole
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
      "ec2:CreateNetworkInterface",
      "ec2:DescribeNetworkInterfaces",
      "ec2:DeleteNetworkInterface",
      "ec2:AssignPrivateIpAddresses",
      "ec2:UnassignPrivateIpAddresses",
      # ついでに
      "ec2:DescribeSubnets",
      "ec2:DescribeSecurityGroups",
      "ec2:DescribeVpcs",
      # ECRを読み込みを許可するポリシー
      # "ecr:GetAuthorizationToken",
      # "ecr:BatchCheckLayerAvailability",
      # "ecr:GetDownloadUrlForLayer",
      # "ecr:BatchGetImage",
      # "ecr:InitiateLayerUpload",
      "ecr:*",
      # DynamoDB用ポリシー
      "dynamodb:BatchGetItem",
      "dynamodb:GetItem",
      "dynamodb:Query",
      "dynamodb:Scan",
      "dynamodb:BatchWriteItem",
      "dynamodb:PutItem",
      "dynamodb:UpdateItem",
      "dynamodb:DeleteItem",
      # API Gatewayへのアクセスを許可する
      "execute-api:Invoke"
    ]

    resources = ["*"]
  }
}

# lambda用のaws_iam_policyを定義する
resource "aws_iam_policy" "lambda_access_iam_policy" {
  name        = "lambda_access_iam_policy"
  description = "lambda_access_iam_policy"
  policy      = data.aws_iam_policy_document.lambda_access_policy.json
}


resource "aws_iam_role_policy_attachment" "lambda_access_policy_attachment" {
  policy_arn = aws_iam_policy.lambda_access_iam_policy.arn
  role       = aws_iam_role.lambda_execution_role.name
}



