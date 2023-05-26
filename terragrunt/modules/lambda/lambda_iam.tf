
resource "aws_iam_role" "lambda_role" {
  name                = "${var.lambda_function_name}-role"
  assume_role_policy  = data.aws_iam_policy_document.lambda_role_policy.json
  managed_policy_arns = var.managed_policy_arns
}

data "aws_iam_policy_document" "lambda_role_policy" {
  statement {
    sid     = ""
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}






