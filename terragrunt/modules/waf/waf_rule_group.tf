

# カスタマイズしたWAFルールグループを作成する
resource "aws_wafv2_rule_group" "waf_rule_group" {
  name        = "${var.project_name}-${var.project_stage}-waf-rule-group"
  scope       = var.scope
  description = "${var.project_name} ${var.project_stage} WAF Rule Group"
  capacity    = 20
  # 国ごとにアクセスを許可する
  rule {
    name     = "allow_country"
    priority = 1
    action {
      allow {}
    }
    statement {
      geo_match_statement {
        country_codes = var.allow_country_codes
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_allow_country"
      sampled_requests_enabled   = false
    }
  }
  # 国ごとにアクセス数上限を設定する
  rule {
    name     = "access_limit_by_country"
    priority = 2
    action {
      block {}
    }
    statement {
      rate_based_statement {
        limit              = var.access_limit_by_country
        aggregate_key_type = "IP"
        scope_down_statement {
          geo_match_statement {
            country_codes = var.allow_country_codes
          }
        }
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_access_limit_by_country"
      sampled_requests_enabled   = false
    }
  }

  # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "${var.project_name}-${var.project_stage}-custom-waf_rule_group"
    sampled_requests_enabled   = false
  }
  tags = {
    Name = "waf_rule_group"
  }
}

