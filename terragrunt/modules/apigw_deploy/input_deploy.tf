
## 他のモジュールに依存する変数
variable "api_gw_rest_api_id" {
  type        = string
  description = "API GatewayのREST API ID"
}

variable "api_gw_resource_path" {
  type        = string
  description = "API Gatewayのリソースパス"
}

# パラメーター
variable "project_name" {
  type        = string
  description = "プロジェクト名"
}

variable "project_stage" {
  type        = string
  description = "ステージ名"
}

variable "quota_limit" {
  type        = number
  description = "時間内に行うことができる最大リクエスト数"
  default     = 1000
}

variable "quota_offset" {
  type        = number
  description = "初期時間帯に与えられた制限から減算されるリクエスト数"
  default     = 0
}

variable "quota_period" {
  type        = string
  description = "制限が適用される時間帯。有効な値はDAY WEEK MONTH"
  default     = "MONTH"
}

variable "throttle_burst_limit" {
  type        = number
  description = "API呼び出しの最大数"
  default     = 1000
}

variable "throttle_rate_limit" {
  type        = number
  description = "API呼び出しのレート制限"
  default     = 1000
}