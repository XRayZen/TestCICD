
resource "aws_api_gateway_resource" "api_gw_resource_world" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  parent_id   = aws_api_gateway_rest_api.api_gw_rest_api.root_resource_id
  path_part   = "world"
}

resource "aws_api_gateway_method" "api_gw_method_world" {
  rest_api_id   = aws_api_gateway_rest_api.api_gw_rest_api.id
  resource_id   = aws_api_gateway_resource.api_gw_resource_world.id
  http_method   = "POST"
  authorization = "NONE"

  request_parameters = {
    "method.request.header.Content-Type" = true
  }

  request_models = {
    "application/json" = "Empty"
  }
}

resource "aws_api_gateway_integration" "api_gw_world_integration" {
  rest_api_id             = aws_api_gateway_rest_api.api_gw_rest_api.id
  resource_id             = aws_api_gateway_resource.api_gw_resource_world.id
  http_method             = aws_api_gateway_method.api_gw_method_world.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.lambda_function_world.invoke_arn
  passthrough_behavior    = "WHEN_NO_MATCH"

  request_templates = {
    "application/json" = "{ \"body\": $input.json('$') }"
  }

  content_handling = "CONVERT_TO_TEXT"
}

resource "aws_api_gateway_method_response" "api_gw_method_response_world" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  resource_id = aws_api_gateway_resource.api_gw_resource_world.id
  http_method = aws_api_gateway_method.api_gw_method_world.http_method
  status_code = "200"

  response_models = {
    "application/json" = "Empty"
  }
}

resource "aws_api_gateway_integration_response" "api_gw_integration_response_world" {
  rest_api_id = aws_api_gateway_rest_api.api_gw_rest_api.id
  resource_id = aws_api_gateway_resource.api_gw_resource_world.id
  http_method = aws_api_gateway_method.api_gw_method_world.http_method
  status_code = "200"

  response_templates = {
    "application/json" = "{ \"body\": $input.json('$') }"
  }
}



