
# WAFマネージドルールをグループとしてまとめる

resource "aws_wafv2_rule_group" "waf_managed_rule_group" {
  name        = "${var.project_name}-${var.project_stage}-manged-waf-rule-group"
  scope       = var.scope
  description = "${var.project_name} ${var.project_stage} Maneged WAF Rule Group"
  capacity    = 20

  # マネージドルールを追加する
  rule {
    name     = "aws_managed_rules_common_rule_set"
    priority = 2
    action {
      block {}
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
  # AWSManagedRulesAmazonIpReputationList
  rule {
    name     = "aws_managed_rules_amazon_ip_reputation_list"
    priority = 3
    action {
      block {}
    }
    statement {
      # マネージドルールを参照する
      managed_rule_group_statement {
        name        = "AWSManagedRulesAmazonIpReputationList"
        vendor_name = "AWS"
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_amazon_ip_reputation_list"
      sampled_requests_enabled   = false
    }
  }
  # AWSManagedRulesAnonymousIpList
  rule {
    name     = "aws_managed_rules_anonymous_ip_list"
    priority = 4
    action {
      block {}
    }
    statement {
      # マネージドルールを参照する
      managed_rule_group_statement {
        name        = "AWSManagedRulesAnonymousIpList"
        vendor_name = "AWS"
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_anonymous_ip_list"
      sampled_requests_enabled   = false
    }
  }
  # AWSManagedRulesKnownBadInputsRuleSet
  rule {
    name     = "aws_managed_rules_known_bad_inputs_rule_set"
    priority = 5
    action {
      block {}
    }
    statement {
      # マネージドルールを参照する
      managed_rule_group_statement {
        name        = "AWSManagedRulesKnownBadInputsRuleSet"
        vendor_name = "AWS"
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_known_bad_inputs_rule_set"
      sampled_requests_enabled   = false
    }
  }

  # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "${var.project_name}-${var.project_stage}-managed-waf_rule_group"
    sampled_requests_enabled   = false
  }
  tags = {
    Name = "${var.project_name}-${var.project_stage}-managed-waf-rule-group"
  }


}

