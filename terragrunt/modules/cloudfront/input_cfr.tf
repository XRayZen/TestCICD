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

variable "origin_name" {
  type        = string
  description = "CloudFrontのオリジン名"
}

variable "price_class" {
  type        = string
  description = "CloudFront価格クラス"
  default = "PriceClass_200"
}

variable "compress" {
  type        = bool
  description = "コンテンツ圧縮(gzip)を許可するかどうか"
  default = false
}












