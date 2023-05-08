
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "4.0.1"

  name = "${var.project_name}-vpc"
  cidr = "10.0.0.0/16"

  azs             = var.availability_zones
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]
  # LAMBDAがネットにアクセスするからNATゲートウェイが必要
  # 単一のNATゲートウェイを経由してインターネットトラフィックをルーティング
  enable_nat_gateway     = true
  single_nat_gateway     = true
  one_nat_gateway_per_az = false

}




