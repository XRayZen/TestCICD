module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "4.0.1"

  name = "${var.project_name}-vpc"
  cidr = var.cidr

  azs             = var.availability_zones
  private_subnets = var.private_subnets
  public_subnets  = var.public_subnets
  # NATゲートウェイを使う場合はtrueにする
  enable_nat_gateway     = var.enable_nat_gateway
  single_nat_gateway     = var.single_nat_gateway
  one_nat_gateway_per_az = var.one_nat_gateway_per_az
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