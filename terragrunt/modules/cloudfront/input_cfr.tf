# 他のリソースに依存する変数

variable "rest_api_invoke_url" {
  type        = string
  description = "API GatewayのURL"
}

variable "waf_arn" {
  type        = string
  description = "WAFのARN"
}

# パラメータ
variable "project_stage" {
  type        = string
  description = "プロジェクトのステージ"
}

variable "project_name" {
  type        = string
  description = "プロジェクト名"

}
















