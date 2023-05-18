
module "ecr-lambda" {
  source                          = "terraform-aws-modules/ecr/aws"
  version                         = "1.6.0"
  repository_name                 = "${var.project_name}-repo"
  repository_image_tag_mutability = "MUTABLE"
  # テラフォーム内でコンテナをプッシュするのであればIAMロールを指定する必要がある
  
  # タグは、AWS環境の全体像を把握しやすくなる
  tags = {
    Name        = "test-cicd-repo"
    Project     = "test-cicd"
    Environment = "dev"
    Owner       = "Matagi"
  }
}


