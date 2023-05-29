


# パラメータ

variable "project_name" {
    type        = string
    description = "プロジェクト名"
}

variable "project_stage" {
    type        = string
    description = "プロジェクトのステージ"
}

variable "scope" {
    type        = string
    description = "WAFのスコープ"
    default = "CLOUDFRONT"
}

variable "allow_country_codes" {
    type = list(string)
    default = ["JP"]
    description = "国別にアクセスを許可する"
}

variable "access_limit_by_country" {
    type = number
    default = 100
    description = "アクセス数上限を設定する"
}

