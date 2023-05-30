
variable "budget_name" {
  type        = string
  default     = "budget"
  description = "Budget name"
}

variable "limit_amount" {
  type        = string
  default     = "10"
  description = "Budget limit"
}

variable "time_unit" {
  type        = string
  default     = "MONTHLY"
  description = "Budget time unit"
}

variable "threshold" {
  type        = string
  default     = "20"
  description = "Budget threshold"
}



variable "notification_emails" {
  type        = list(string)
  default     = [""]
  description = "Email addresses to notify when budget is exceeded"
}

