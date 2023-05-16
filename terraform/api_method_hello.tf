# API内のエンドポイントを定義する
resource "aws_api_gateway_resource" "api_gw_resource_hello" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  # (必須) API リソースの親リソースの ID です。
  parent_id = aws_api_gateway_rest_api.api_gw_rest_api.root_resource_id
  # (必須) この API リソースのラストパスセグメントです。
  path_part = "{proxy+}"
}

resource "aws_api_gateway_method" "api_gw_method_hello" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  # (必須) API メソッドのリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource_hello.id
  # (必須) API メソッドの HTTP メソッドです。
  http_method = "POST"
  # 今はまだ認証は設けない prodステージで実装する
  authorization = "NONE"
}

# リクエストとリソースの統合
resource "aws_api_gateway_integration" "api_gw_hello_integration" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  # (必須) API 統合のリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource_hello.id
  # (必須) API 統合の HTTP メソッドです。
  http_method = aws_api_gateway_method.api_gw_method_hello.http_method
  # Lambda関数との統合中に使用されるHTTPメソッド。
  integration_http_method = "POST"
  # (必須) API 統合のタイプです。
  # 現在のサポートされている値は、AWS、AWS_PROXY、HTTP、HTTP_PROXY、MOCK です。
  type = "AWS_PROXY"
  # API 統合の URI です。
  uri = aws_lambda_function.lambda_function_hello.invoke_arn
  # これ以降の設定はprotobufの場合では必要
  passthrough_behavior = "WHEN_NO_MATCH"
  request_templates = {
    "application/octet-stream" = "$input.body"
    "application/x-protobuf"   = "$input.body"
  }
  content_handling = "CONVERT_TO_BINARY"

}

resource "aws_api_gateway_method_response" "api_gw_method_response_hello" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  # (必須) API メソッドレスポンスのリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource_hello.id
  # (必須) API メソッドレスポンスの HTTP メソッドです。
  http_method = aws_api_gateway_method.api_gw_method_hello.http_method
  # (必須) API メソッドレスポンスのステータスコードです。
  status_code = "200"
  # HTTPメソッドのレスポンスはHTTPメソッドの後に作る必要があるため、明示的な依存を宣言
  depends_on = [
    aws_api_gateway_method.api_gw_method_hello
  ]
}

# 統合レスポンス
resource "aws_api_gateway_integration_response" "api_gw_integration_response_hello" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  # (必須) API 統合レスポンスのリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource_hello.id
  # (必須) API 統合レスポンスの HTTP メソッドです。
  http_method = aws_api_gateway_method.api_gw_method_hello.http_method
  # (必須) API 統合レスポンスのステータスコードです。
  status_code = aws_api_gateway_method_response.api_gw_method_response_hello.status_code

  # protobufの場合では必要
  # バイナリデータをパススルーするように設定
  response_templates = {
    "application/octet-stream" = "$input.body"
    "application/x-protobuf"   = "$input.body"
  }
}
