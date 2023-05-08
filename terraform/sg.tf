# lamdbaのセキュリティーグループ
resource "aws_security_group" "lambda-sg" {
  name        = "lambda-sg"
  description = "lambda security group"
  vpc_id      = module.vpc.vpc_id
  # API Gatewayへの送受信を許可する
  ingress {
    from_port   = 0
    to_port     = 65535
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port       = 0
    to_port         = 65535
    protocol        = "tcp"
    # Lambdaがインターネット接続不要であれば、下記のコメントアウトを外す
    cidr_blocks     = [module.vpc.vpc_cidr_block]
  }
}

# API Gateway側のセキュリティグループを作成
resource "aws_security_group" "api_gateway_sg" {
  name_prefix = "api_gateway_sg_"
  vpc_id      = module.vpc.vpc_id

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Lambdaへの接続はAPI Gatewayから始まるため、egressは不要
}

# インスタンスがWebスクレイピングとかでネットにアクセスする用のセキュリティーグループ
# 全てのポートを受け入れてるから慎重に使うこと
resource "aws_security_group" "internet_access_sg" {
  name_prefix = "internet_access_sg"
  description = "Free Internet Access Security Group for instances"

  ingress {
    from_port   = 0
    to_port     = 65535
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "internet_access_sg"
  }
}
