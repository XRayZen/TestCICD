
# ApiGwから呼び出されるには権限を設定する必要がある
resource "aws_lambda_permission" "apigw_lambda_permission_hello" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_function_hello.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_api_gateway_rest_api.api_gw_rest_api.execution_arn}/*/*/*"
}

resource "aws_lambda_function" "lambda_function_hello" {
  function_name = "hello"
  description   = "hello function"
  # 関数の要件に応じた権限が付与されたIAMロールARNを指定する
  role = aws_iam_role.lambda_execution_role.arn
  # ファイル名とエントリーポイントを指定するが、イメージを使うので不要
  # handler = "lambda-handler"
  # runtime = "provided.al2"
  # メモリサイズの大きさに応じて割り当てられるCPUリソースが変わる
  # 最小値は128MB、最大値は3008MB
  memory_size = 128
  timeout     = 5

  package_type = "Image"
  image_uri = "${module.ecr-lambda.repository_url}:hello"

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
    # module.ecr-lambda,
    module.vpc
  ]
}
# 外部ソース（EventBridge Rule、SNS、S3など）にLambda関数へのアクセス権限を付与する。
resource "aws_lambda_permission" "apigw_lambda_permission_world" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda_function_world.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_api_gateway_rest_api.api_gw_rest_api.execution_arn}/*"
  # "${aws_api_gateway_rest_api.api_gw_rest_api.execution_arn}/*/*/*"
}

resource "aws_lambda_function" "lambda_function_world" {
  function_name = "world"
  description   = "world function 02"
  # 関数の要件に応じた権限が付与されたIAMロールARNを指定する
  role = aws_iam_role.lambda_execution_role.arn
  # ファイル名とエントリーポイントを指定するが、イメージを使うので不要
  # メモリサイズの大きさに応じて割り当てられるCPUリソースが変わる
  # 最小値は128MB、最大値は3008MB
  memory_size = 128
  timeout     = 5

  package_type = "Image"
  image_uri = "${module.ecr-lambda.repository_url}:world"

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
    # module.ecr-lambda,
    module.vpc
  ]
}
