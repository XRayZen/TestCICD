
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







