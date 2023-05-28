
# API Gatewayのログ出力設定
# https://docs.aws.amazon.com/ja_jp/apigateway/latest/developerguide/set-up-logging.html
# TF公式: https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/api_gateway_stage
resource "aws_api_gateway_method_settings" "api_gw_method_settings" {
  rest_api_id = var.api_gw_rest_api_id
  stage_name  = aws_api_gateway_stage.api_stage.stage_name
  method_path = "*/*"

  settings {
    metrics_enabled = true
    logging_level   = "INFO"
  }
  # このリソースはステージ後に実行する必要がある
  depends_on = [
    aws_api_gateway_stage.api_stage,
  ]
}
# ApiGatewayにクラウドウォッチのロールARNを追加する
resource "aws_api_gateway_account" "api_gw_account" {
  cloudwatch_role_arn = aws_iam_role.cloudwatch.arn
}

data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["apigateway.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "cloudwatch" {
  name               = "api_gateway_cloudwatch_global"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

data "aws_iam_policy_document" "cloudwatch" {
  statement {
    effect = "Allow"

    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:DescribeLogGroups",
      "logs:DescribeLogStreams",
      "logs:PutLogEvents",
      "logs:GetLogEvents",
      "logs:FilterLogEvents",
    ]

    resources = ["*"]
  }
}
resource "aws_iam_role_policy" "cloudwatch" {
  name   = "default-API-Gateway-Execution-Logs"
  role   = aws_iam_role.cloudwatch.id
  policy = data.aws_iam_policy_document.cloudwatch.json
}





