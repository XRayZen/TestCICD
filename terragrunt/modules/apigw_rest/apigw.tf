
resource "aws_api_gateway_rest_api" "api_gw_rest_api" {
  name        = "${var.project_name}_api_gw_rest_api"
  description = var.api_gw_description
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
    # jsonもサポートするように設定
    "application/json",
  ]
}
