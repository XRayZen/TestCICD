
output "api_gw_rest_api_id" {
  value = aws_api_gateway_rest_api.api_gw_rest_api.id
}

output "api_gw_root_resource_id" {
  value = aws_api_gateway_rest_api.api_gw_rest_api.root_resource_id
}

output "rest_api_execution_arn" {
  value = aws_api_gateway_rest_api.api_gw_rest_api.execution_arn
}


