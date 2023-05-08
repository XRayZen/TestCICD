API GatewayをCloudfrontに紐付けるには、以下の手順が必要です。

1. CloudFrontディストリビューションの作成

まず最初に、API GatewayとCloudFrontを紐付けるために、CloudFrontディストリビューションを作成する必要があります。これは、AWSのマネジメントコンソールまたはTerraformで行うことができます。以下は、Terraformでのディストリビューション作成の例です。

```bash
resource "aws_cloudfront_distribution" "example" {
  origin {
    domain_name = aws_api_gateway_deployment.example.execution_arn
    origin_id   = aws_api_gateway_deployment.example.id
  }

  # その他の必要な設定（省略可能）
}
```

2. API Gatewayの設定

API Gatewayでは、カスタムドメイン名が必要になるため、事前に設定が必要です。Terraformでやる場合は以下のようにします。

```bash
resource "aws_api_gateway_domain_name" "example" {
  domain_name              = "api.example.com"
  certificate_arn          = "arn:aws:acm:us-east-1:11111111111:certificate/abcd1234-ab12-34cd-abcd-1234abcd5678"
  security_policy          = "TLS_1_2"
  endpoint_configuration   = {
    types = ["REGIONAL"]
  }
}

resource "aws_api_gateway_base_path_mapping" "example" {
  rest_api_id   = aws_api_gateway_rest_api.example.id
  stage_name    = aws_api_gateway_deployment.example.stage_name
  domain_name   = aws_api_gateway_domain_name.example.id
}
```

以上の設定で、API Gatewayのカスタムドメイン名を作成し、デプロイされたAPIに関連付けます。

3. CloudFrontとAPI Gatewayの紐付け

CloudFrontディストリビューションとAPI Gatewayを紐付けるには、AWS Lambda@Edgeを使用する必要があります。TerraformでLambda@Edgeを設定する場合、以下のようにします。

```terraform
resource "aws_lambda_function" "example" {
  filename         = "lambda_function_payload.zip"
  function_name    = "example_lambda_function"
  role             = aws_iam_role.lambda.arn
  handler          = "lambda_function.handler"
  source_code_hash = filebase64sha256("lambda_function_payload.zip")
  runtime          = "nodejs14.x"
}

resource "aws_cloudfront_origin_request_policy" "example" {
  origin_request_policy_config = jsonencode({
    # 許可ポリシーの設定
  })
}

resource "aws_cloudfront_function" "example_edge_function" {
  name         = "example_edge_function"
  function_code = aws_lambda_function.example.code_sha256
}

resource "aws_cloudfront_function_association" "example" {
  event_type    = "origin-request"
  function_arn  = aws_lambda_function.example.arn
  include_body  = true
  distribution_id = aws_cloudfront_distribution.example.id
}

resource "aws_cloudfront_cache_behavior" "example" {
  path_pattern           = "/api/*"
  allowed_methods        = ["GET", "HEAD", "OPTIONS"]
  cached_methods         = ["GET", "HEAD", "OPTIONS"]
  target_origin_id       = aws_cloudfront_distribution.example.origin[0].origin_id
  forwarded_values {
    query_string = false
  }
  viewer_protocol_policy = "https-only"
  origin_request_policy_id = aws_cloudfront_origin_request_policy.example.id
  function_association {
    event_type    = "origin-request"
    function_arn  = aws_cloudfront_function.example_edge_function.qualified_arn
    include_body  = true
  }
}
```

以上で、CloudFrontとAPI Gatewayの紐付けが完了しました。この設定により、API Gatewayのエンドポイントを直接公開することなく、CloudFrontから保護された形で利用することができます。