
module "ecr-lambda" {
  source                          = "terraform-aws-modules/ecr/aws"
  version                         = "1.6.0"
  repository_name                 = "${var.project_name}-repo"
  repository_image_tag_mutability = "MUTABLE"
  # ECRリポジトリにアクセスするLambdaのARNを指定する
  # repository_lambda_read_access_arns = [
  #   aws_lambda_function.lambda_function_hello.arn,
  # aws_lambda_function.lambda_function_world.arn, ]
  # repository_read_write_access_arns = [
  #   aws_iam_role.lambda_execution_role.arn
  # ]
  repository_lifecycle_policy = jsonencode({
    rules = [
      {
        rulePriority = 1,
        description  = "Keep last 5 images",
        selection = {
          tagStatus     = "tagged",
          tagPrefixList = ["v"],
          countType     = "imageCountMoreThan",
          countNumber   = 5
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


