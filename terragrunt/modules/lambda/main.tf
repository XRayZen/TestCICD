
resource "aws_lambda_function" "lambda_func_" {
  function_name = var.lambda_function_name
  description   = var.lambda_function_description
  memory_size   = var.memory_size
  timeout       = var.timeout
  role          = aws_iam_role.lambda_role.arn

  package_type = "Image"
  image_uri    = var.image_url

  vpc_config {
    subnet_ids         = var.subnet_ids
    security_group_ids = var.security_group_ids
  }
  environment {
    variables = var.environment_variables
  }
  lifecycle {
    ignore_changes = [
      "environment_variables"
    ]
  }
}






