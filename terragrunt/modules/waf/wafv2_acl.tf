
# WAF ACL
resource "aws_wafv2_web_acl" "waf" {
  name        = "${var.project_name}-${var.project_stage}-waf"
  scope       = var.scope
  description = "${var.project_name} ${var.project_stage} WAF ACL"

  # 許可されたルール以外のリクエストをブロックする
  # サービス運用だと許可をして個別にブロックする
  default_action {
    block {}
  }
  rule {
    # カスタマイズしたルールグループを参照する
    name     = aws_wafv2_rule_group.waf_rule_group.name
    priority = 1
    # Webリクエストを上書きするかどうか
    # ルールグループを参照する時に必要
    override_action {
      # 上書きしない
      none {}
    }
    statement {
      # ルールグループを参照する
      rule_group_reference_statement {
        arn = aws_wafv2_rule_group.waf_rule_group.arn
      }
    }
    # visibility_config はルールグループに定義済み
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    # visibility_config {
    #   cloudwatch_metrics_enabled = true
    #   metric_name                = "waf_rg_reference_rg"
    #   sampled_requests_enabled   = false
    # }
  }
  # マネージドルールグループを参照する
  rule {
    name     = aws_wafv2_rule_group.waf_managed_rule_group.name
    priority = 2
    # Webリクエストを上書きするかどうか
    # マネージドルールを参照する時に必要
    override_action {
      # 上書きしない
      none {}
    }
    statement {
      # マネージドルールを参照する
      managed_rule_group_statement {
        name        = aws_wafv2_rule_group.waf_managed_rule_group.name
        vendor_name = "AWS"
      }
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "${var.project_name}-${var.project_stage}-waf"
    sampled_requests_enabled   = false
  }
  tags = {
    Name = "${var.project_name}-${var.project_stage}-waf"
    Maneged_by = "Terraform"
  }

  depends_on = [
    aws_wafv2_rule_group.waf_rule_group,
    aws_wafv2_rule_group.waf_managed_rule_group
  ]
}


