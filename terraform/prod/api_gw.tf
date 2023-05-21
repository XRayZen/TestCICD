

resource "aws_api_gateway_rest_api" "api_gw_rest_api" {
  name        = "api_gw_rest"
  description = "Example API Gateway with gRPC and Lambda"
  # APIエンドポイント設定を定義する
  endpoint_configuration {
    #  CloudFrontを使用する場合は、REGIONALを指定する
    types = ["REGIONAL"]
  }
  # REST APIがサポートするバイナリーメディアタイプ
  # API Gatewayがprotobufをサポートするように設定
  binary_media_types = [
    "application/octet-stream",
    "application/x-protobuf",
    # OpenAPIもサポートするように設定
    "application/json",
  ]
}

resource "aws_api_gateway_stage" "api_stage_prod" {
  stage_name    = "${var.project_stage}"  
  rest_api_id   = aws_api_gateway_rest_api.api_gw_rest_api.id
  deployment_id = aws_api_gateway_deployment.api_gw_deploy.id
  # CloudWatch Logs用
  depends_on = [
    # TODO:クラウドウォッチを追加する
  ]
}

resource "aws_api_gateway_deployment" "api_gw_deploy" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  depends_on = [
    aws_api_gateway_integration.api_gw_hello_integration,
    aws_api_gateway_integration.api_gw_world_integration,
  ]
  # これ以降の紐付けを維持する為の設定
  lifecycle {
    create_before_destroy = true
  }
  stage_description = "timestamp = ${timestamp()}"
}

resource "aws_api_gateway_usage_plan" "api_usage_plan" {
    name = "api_usage_plan"
    description = "api_usage_plan"
    api_stages {
        api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
        stage = aws_api_gateway_stage.api_stage_prod.stage_name
    }
    product_code = "APIGW-TEST-CICD"
    # 月額課金の場合は固定値
    quota_settings {
        # 時間内に行うことができる最大リクエスト数
        limit = 1000
        # 初期時間帯に与えられた制限から減算されるリクエスト数
        offset = 0
        # 制限が適用される時間帯。有効な値は"DAY"、 "WEEK"、または"MONTH"
        period = "MONTH"
    }
    throttle_settings {
        # APIリクエストバースト制限
        burst_limit = 100
        # APIリクエストのレート制限
        rate_limit = 50
    }
}