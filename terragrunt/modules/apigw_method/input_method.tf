## モジュールから変数を受け取る
## 他のモジュールから変数を受け取る
variable "api_gw_rest_api_id" {
  type        = string
  description = "API GatewayのREST API ID"
}

variable "api_gw_root_resource_id" {
  type        = string
  description = "API GatewayのルートリソースID"
}

variable "rest_api_execution_arn" {
  type        = string
  description = "API GatewayのREST APIの実行ARN"
  default = "arn:aws:execute-api:ap-northeast-1:123456789012:abcdefghij/*/GET/hello"
}

variable "lambda_function_invoke_arn" {
  type        = string
  description = "Lambda関数のInvoke ARN"  
  default = "arn:aws:lambda:ap-northeast-1:123456789012:function:hello"
}

variable "lambda_function_name" {
  type        = string
  description = "紐づけるLambda関数名"
}

## パラメーター
variable "lambda_permission_source_arn_with_path" {
  type        = string
  description = "この部分でのパスは、API Gateway内の任意のステージ、メソッド、リソースパスから呼び出すことができます。"
  default     = "/*"
}

variable "resource_last_path_part" {
  type        = string
  description = "APIリソースのラストパスパート"
}

variable "http_method" {
  type        = string
  description = "APIリソースのHTTPメソッド"
}

variable "api_gw_integration_content_handling" {
  type        = string
  description = "API Gatewayの統合のリクエストペイロードのコンテンツタイプコンバージョンの処理方法 (CONVERT_TO_BINARY(gRpc) or CONVERT_TO_TEXT(json))"
  default = "CONVERT_TO_BINARY"
}
