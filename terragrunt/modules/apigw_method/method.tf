# ApiGwから呼び出されるには権限を設定する必要がある
resource "aws_lambda_permission" "apigw_lambda_permission" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = var.lambda_function_name
  principal     = "apigateway.amazonaws.com"
  # /* 部分は、API Gateway内の任意のステージ、メソッド、リソースパスから呼び出すことができます。
  source_arn = "${var.rest_api_execution_arn}${var.lambda_permission_source_arn_with_path}"
  # CloudFront用に用意したパス: "${aws_api_gateway_rest_api.api_gw_rest_api.execution_arn}/*/*/*"
}

# API Gatewayに新しいエンドポイントを追加
resource "aws_api_gateway_resource" "api_gw_resource" {
  rest_api_id = var.api_gw_rest_api_id
  # (必須) API リソースの親リソースの ID です。
  parent_id = var.api_gw_root_resource_id
  # (必須) この API リソースのラストパスセグメントです。
  path_part = var.resource_last_path_part
  # "{proxy+}"
}

resource "aws_api_gateway_method" "api_gw_method" {
  rest_api_id = var.api_gw_rest_api_id
  # (必須) API メソッドのリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource.id
  # (必須) API メソッドの HTTP メソッドです。
  http_method = var.http_method
  # CloudFrontで認証を行うため、NONEに設定
  authorization = "NONE"
}

# リクエストとリソースの統合
resource "aws_api_gateway_integration" "api_gw_integration" {
  rest_api_id = var.api_gw_rest_api_id
  # (必須) API 統合のリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource.id
  # (必須) API 統合の HTTP メソッドです。
  http_method = aws_api_gateway_method.api_gw_method.http_method
  # Lambda関数との統合中に使用されるHTTPメソッド。API Gatewayがバックエンドとどのように対話するかを指定する
  integration_http_method = "POST"
  # (必須) API 統合のタイプです。
  # 現在のサポートされている値は、AWS、AWS_PROXY、HTTP、HTTP_PROXY、MOCK です。
  type = "AWS_PROXY"
  # API 統合の URI です。
  uri = var.lambda_function_invoke_arn
  # これ以降の設定はprotobufの場合では必要
  passthrough_behavior = "WHEN_NO_MATCH"
  # リクエストペイロードのコンテンツタイプコンバージョンの処理方法
  # CONVERT_TO_BINARY: バイナリペイロードをBase64エンコードされた文字列に変換します。
  # CONVERT_TO_TEXT: Base64エンコードされた文字列をバイナリペイロードに変換します。
  content_handling = var.api_gw_integration_content_handling
  request_templates = {
    "application/octet-stream" = "$input.body"
    "application/x-protobuf"   = "$input.body"
    "application/json" = "{ \"body\": $input.json('$') }"
  }
  depends_on = [
    aws_api_gateway_method.api_gw_method,
  ]
}

resource "aws_api_gateway_method_response" "api_gw_method_response" {
  rest_api_id = var.api_gw_rest_api_id
  # (必須) API メソッドレスポンスのリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource.id
  # (必須) API メソッドレスポンスの HTTP メソッドです。
  http_method = aws_api_gateway_method.api_gw_method.http_method
  # (必須) API メソッドレスポンスのステータスコードです。
  status_code = "200"
  # HTTPメソッドのレスポンスはHTTPメソッドの後に作る必要があるため、明示的な依存を宣言
  depends_on = [
    aws_api_gateway_method.api_gw_method,
    aws_api_gateway_integration.api_gw_integration
  ]
}

# 統合レスポンス
resource "aws_api_gateway_integration_response" "api_gw_integration_response" {
  rest_api_id = var.api_gw_rest_api_id
  # (必須) API 統合レスポンスのリソース ID です。
  resource_id = aws_api_gateway_resource.api_gw_resource.id
  # (必須) API 統合レスポンスの HTTP メソッドです。
  http_method = aws_api_gateway_method.api_gw_method.http_method
  # (必須) API 統合レスポンスのステータスコードです。
  status_code = aws_api_gateway_method_response.api_gw_method_response.status_code

  # protobufの場合では必要
  # バイナリデータをパススルーするように設定
  response_templates = {
    "application/octet-stream" = "$input.body"
    "application/x-protobuf"   = "$input.body"
    "application/json" = "{ \"body\": $input.json('$') }"
  }

  depends_on = [
    aws_api_gateway_method_response.api_gw_method_response,
    aws_api_gateway_integration.api_gw_integration
  ]
}



