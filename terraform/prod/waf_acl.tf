
# WAF ACL
resource "aws_wafv2_web_acl" "waf" {
  name        = "waf"
  scope       = "CLOUDFRONT"
  description = "waf"
  # CloudFrontを使う場合はAWSリソースのリージョンをus-east-1(Region:Virginia)にする
  provider = aws.virginia

  # 許可されたルール以外のリクエストをブロックする
  # CICDポートフォリオだから大胆にブロックできるがサービス運用だとできないだろう
  default_action {
    block {}
  }
  rule {
    # ルールグループを参照する
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
      metric_name                = "waf_rg_reference_rg"
      sampled_requests_enabled   = false
    }
  }
  # マネージドルールを追加する
  rule {
    name     = "aws_managed_rules_common_rule_set"
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
    # Webリクエストを上書きするかどうか
    # マネージドルールを参照する時に必要
    override_action {
      # 上書きしない
      none {}
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
    # Webリクエストを上書きするかどうか
    # マネージドルールを参照する時に必要
    override_action {
      # 上書きしない
      none {}
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
    # Webリクエストを上書きするかどうか
    # マネージドルールを参照する時に必要
    override_action {
      # 上書きしない
      none {}
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

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "waf"
    sampled_requests_enabled   = false
  }
  tags = {
    Name = "waf"
  }
}

# ルールをそこに直接書くと助長になるのでWAFルールグループを作成する
resource "aws_wafv2_rule_group" "waf_rule_group" {
  name        = "waf_rule_group"
  scope       = "CLOUDFRONT"
  description = "waf_rule_group"
  capacity    = 20
  # 日本からのアクセスを許可する
  rule {
    name     = "allow_japan"
    priority = 1
    action {
      allow {}
    }
    statement {
      geo_match_statement {
        country_codes = ["JP"]
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_allow_japan"
      sampled_requests_enabled   = false
    }
  }
  # 日本からのアクセス数上限を設定する
  rule {
    name     = "limit_japan"
    priority = 2
    action {
      block {}
    }
    statement {
      rate_based_statement {
        limit              = 100
        aggregate_key_type = "IP"
        scope_down_statement {
          geo_match_statement {
            country_codes = ["JP"]
          }
        }
      }
    }
    # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "waf_rg_limit_japan"
      sampled_requests_enabled   = false
    }
  }

  # Amazon CloudWatchのメトリクスとWebリクエストのサンプル収集を定義
  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "waf_rule_group"
    sampled_requests_enabled   = false
  }
  tags = {
    Name = "waf_rule_group"
  }
}























