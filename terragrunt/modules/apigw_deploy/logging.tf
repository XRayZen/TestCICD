
resource "aws_api_gateway_method_settings" "api_gw_method_settings" {
  rest_api_id = var.api_gw_rest_api_id
  stage_name  = var.project_stage
  method_path = var.api_gw_resource_path

  settings {
    metrics_enabled = true
    logging_level   = "INFO"
  }
}







