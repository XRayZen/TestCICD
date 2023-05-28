
output "api_gw_stage_name" {
  value = aws_api_gateway_stage.api_stage.stage_name
}

output "rest_api_invoke_url" {
  value = aws_api_gateway_deployment.api_gw_deploy.invoke_url
}
