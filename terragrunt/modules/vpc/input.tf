
variable "project_name" {
  type        = string
  description = "Project name"
}

variable "cidr" {
  type        = string
  description = "CIDR"
  default     = "10.0.0.0/16"
}

variable "availability_zones" {
  type        = list(string)
  description = "Availability zones"
}

variable "private_subnets" {
  type        = list(string)
  description = "Private subnets"
  default     = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
}

variable "public_subnets" {
  type        = list(string)
  description = "Public subnets"
  default     = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]
}

variable "enable_nat_gateway" {
  type        = bool
  description = "Enable NAT gateway"
}

variable "single_nat_gateway" {
  type        = bool
  description = "Single NAT gateway"
}

variable "one_nat_gateway_per_az" {
  type        = bool
  description = "One NAT gateway per AZ"
}

