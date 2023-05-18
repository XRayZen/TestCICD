
module "ecr-lambda" {
  source                          = "terraform-aws-modules/ecr/aws"
  version                         = "1.6.0"
  repository_name                 = "${var.project_name}-repo"
  repository_image_tag_mutability = "MUTABLE"
  repository_lifecycle_policy = jsonencode({
    rules = [
      {
        rulePriority = 1,
        description  = "Keep last 30 images",
        selection = {
          tagStatus     = "tagged",
          tagPrefixList = ["v"],
          countType     = "imageCountMoreThan",
          countNumber   = 30
        },
        action = {
          type = "expire"
        }
      }
    ]
  })
  # タグは、AWS環境の全体像を把握しやすくなる
  tags = {
    Name        = "test-cicd-repo"
    Project     = "test-cicd"
    Environment = var.project_stage
    Owner       = "Matagi"
  }
}


