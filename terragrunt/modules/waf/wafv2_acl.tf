
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
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_custom_rule_set"
      sampled_requests_enabled   = false
    }
  }
  # マネージドルールグループを参照する
  # managed_rule_group_statementがACLにしか無いから見づらくなるが、ここに書く必要がある
  rule {
    name     = "aws_managed_rules_common_rule_set"
    priority = 20
    override_action {
      none {}
    }
    statement {
      # マネージドルールを参照する
      managed_rule_group_statement {
        name        = "AWSManagedRulesCommonRuleSet"
        vendor_name = "AWS"
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_common_rule_set"
      sampled_requests_enabled   = false
    }
  }
  rule {
    name     = "aws_managed_rules_amazon_ip_reputation_list"
    priority = 30
    override_action {
      none {}
    }
    statement {
      managed_rule_group_statement {
        name        = "AWSManagedRulesAmazonIpReputationList"
        vendor_name = "AWS"
      }
    }
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_amazon_ip_reputation_list"
      sampled_requests_enabled   = false
    }
  }
  rule {
    name     = "aws_managed_rules_anonymous_ip_list"
    priority = 40
    override_action {
      none {}
    }
    statement {
      managed_rule_group_statement {
        name        = "AWSManagedRulesAnonymousIpList"
        vendor_name = "AWS"
      }
    }
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_anonymous_ip_list"
      sampled_requests_enabled   = false
    }
  }

  rule {
    name     = "aws_managed_rules_known_bad_inputs_rule_set"
    priority = 50
    override_action {
      none {}
    }
    statement {
      managed_rule_group_statement {
        name        = "AWSManagedRulesKnownBadInputsRuleSet"
        vendor_name = "AWS"
      }
    }
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_known_bad_inputs_rule_set"
      sampled_requests_enabled   = false
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "${var.project_name}-${var.project_stage}-waf"
    sampled_requests_enabled   = false
  }
  tags = {
    Name       = "${var.project_name}-${var.project_stage}-waf"
    Maneged_by = "Terraform"
  }

  depends_on = [
    aws_wafv2_rule_group.waf_rule_group
  ]
}


