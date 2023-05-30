module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "4.0.1"

  name = "${var.project_name}-vpc"
  cidr = var.cidr

  azs             = var.availability_zones
  private_subnets = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  public_subnets  = ["10.0.101.0/24", "10.0.102.0/24", "10.0.103.0/24"]
  # NATゲートウェイを使う場合はtrueにする
  enable_nat_gateway     = false
  single_nat_gateway     = false
  one_nat_gateway_per_az = false
}

# ラムダ関数をVPC内で実行するためには、VPCのサブネットを指定する必要があるので用意する
data "aws_subnet" "public" {
  count      = length(module.vpc.public_subnets)
  id         = element(module.vpc.public_subnets, count.index)
  depends_on = [module.vpc]
}

data "aws_subnet" "private" {
  count      = length(module.vpc.private_subnets)
  id         = element(module.vpc.private_subnets, count.index)
  depends_on = [module.vpc]
}