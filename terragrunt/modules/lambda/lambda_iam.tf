
resource "aws_iam_role" "lambda_role" {
  name               = "${var.lambda_function_name}-role"
  assume_role_policy = data.aws_iam_policy_document.lambda_role_policy.json
  managed_policy_arns = [
    # Lambda関数がCloudWatch Logsにログを書き込むための最低限の権限を提供します。
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    # Lambda関数がVPC内のリソースにアクセスしながら実行するための最低限の権限（ネットワークインターフェースの作成、記述、削除、CloudWatch Logsへの書き込み権限）を提供します。
    "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole",
    # Amazon ECR に対する読み取り専用アクセスを付与
    "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    # APIGW用
    "arn:aws:iam::aws:policy/AmazonAPIGatewayInvokeFullAccess",
  ]
}

data "aws_iam_policy_document" "lambda_role_policy" {
  statement {
    sid     = ""
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}






