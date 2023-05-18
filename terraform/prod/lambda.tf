
# ApiGwから呼び出されるには権限を設定する必要がある
resource "aws_lambda_permission" "apigw_lambda_permission_hello" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_function_hello.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_api_gateway_rest_api.api_gw_rest_api.execution_arn}/*/*/*"
}

data "aws_ecr_image" "hello-ecr-image" {
  repository_name = "${var.project_name}-repo"
  image_tag       = "hello"
  depends_on      = [module.ecr-lambda]
}

resource "aws_lambda_function" "lambda_function_hello" {
  function_name = "hello"
  description   = "hello function"
  # 関数の要件に応じた権限が付与されたIAMロールARNを指定する
  role = aws_iam_role.lambda_execution_role.arn
  # ファイル名とエントリーポイントを指定する
  handler = "hello.main"
  runtime = "provided.al2"

  image_uri = "${module.ecr-lambda.repository_url}:hello"

  # メモリサイズの大きさに応じて割り当てられるCPUリソースが変わる
  # 最小値は128MB、最大値は3008MB
  memory_size = 128
  timeout     = 5
  environment {
    variables = {
      TABLE_NAME = aws_dynamodb_table.prime_ministers_table.name
    }
  }
  # AWSのリソースにアクセスするためにVPCを指定する
  vpc_config {
    #両方とも指定しないとVPC設定はされていないと判断される
    subnet_ids         = [data.aws_subnet.public[0].id]
    security_group_ids = [aws_security_group.lambda-sg.id]
  }
  # 依存関係のあるリソースを指定する
  depends_on = [
    module.ecr-lambda,
    module.vpc
  ]
}

data "aws_ecr_image" "world-ecr-image" {
  repository_name = "${var.project_name}-repo"
  image_tag       = "world"
  depends_on      = [module.ecr-lambda]
}

resource "aws_lambda_function" "lambda_function_world" {
  function_name = "world"
  description   = "world function"
  # 関数の要件に応じた権限が付与されたIAMロールARNを指定する
  role = aws_iam_role.lambda_execution_role.arn
  # ファイル名とエントリーポイントを指定する
  handler = "world.main"
  runtime = "provided.al2"

  image_uri = "${module.ecr-lambda.repository_url}:world"
  # メモリサイズの大きさに応じて割り当てられるCPUリソースが変わる
  # 最小値は128MB、最大値は3008MB
  memory_size = 128
  timeout     = 5

  environment {
    variables = {
      TABLE_NAME = aws_dynamodb_table.prime_ministers_table.name
    }
  }
  # AWSのリソースにアクセスするためにVPCを指定する
  vpc_config {
    #両方とも指定しないとVPC設定はされていないと判断される
    subnet_ids         = [data.aws_subnet.public[0].id]
    security_group_ids = [aws_security_group.lambda-sg.id]
  }
  # 依存関係のあるリソースを指定する
  depends_on = [
    module.ecr-lambda,
    module.vpc
  ]
}
