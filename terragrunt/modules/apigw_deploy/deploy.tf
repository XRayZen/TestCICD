
resource "aws_api_gateway_deployment" "api_gw_deploy" {
  rest_api_id = var.api_gw_rest_api_id
  # テラグラントが実行順序を保証してくれるので、不要
  # depends_on = [
  #   aws_api_gateway_integration.api_gw_integration,
  #   aws_api_gateway_method.api_gw_method,
  # ]
  # これ以降の紐付けを維持する為の設定
  lifecycle {
    create_before_destroy = true
  }
  # ステージを最新状態に保つために、毎回デプロイする
  stage_description = "timestamp = ${timestamp()}"
}

resource "aws_api_gateway_stage" "api_stage" {
  stage_name    = var.project_stage
  rest_api_id   = var.api_gw_rest_api_id
  deployment_id = aws_api_gateway_deployment.api_gw_deploy.id
  # CloudWatch Logs用
  depends_on = [
    # TODO:クラウドウォッチを追加する
  ]
}

resource "aws_api_gateway_usage_plan" "api_usage_plan" {
  name        = "${var.project_name}_usage_plan"
  description = "api_usage_plan"
  api_stages {
    api_id = var.api_gw_rest_api_id
    stage  = aws_api_gateway_stage.api_stage.stage_name
  }
  product_code = var.project_name
  # 月額課金の場合は固定値
  quota_settings {
    # 時間内に行うことができる最大リクエスト数
    limit = var.quota_limit
    # 初期時間帯に与えられた制限から減算されるリクエスト数
    offset = var.quota_offset
    # 制限が適用される時間帯。有効な値は"DAY"、 "WEEK"、または"MONTH"
    period = var.quota_period
  }
  throttle_settings {
    # APIリクエストバースト制限
    burst_limit = var.throttle_burst_limit
    # APIリクエストのレート制限
    rate_limit = var.throttle_rate_limit
  }
}
