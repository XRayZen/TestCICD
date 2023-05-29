
resource "aws_cloudfront_distribution" "cf_dist" {
  enabled    = true
  web_acl_id = aws_wafv2_web_acl.waf.arn
  origin {
    domain_name = replace(aws_api_gateway_deployment.api_gw_deploy.invoke_url, "/^https?://([^/]*).*/", "$1")
    origin_id   = "apigw_root"

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1", "TLSv1.1"]
    }
  }

  default_cache_behavior {
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "apigw_root"

    forwarded_values {
      query_string = false

      cookies {
        forward = "all"
      }
    }
    # 
    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 0
    max_ttl                = 0

  }

  ordered_cache_behavior {
    path_pattern     = "/${var.project_stage}/*"
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "apigw_root"

    forwarded_values {
      query_string = false

      cookies {
        forward = "all"
      }
    }
    # これを指定するとビューワーからのリクエストをHTTPSにリダイレクトする
    # https://docs.aws.amazon.com/ja_jp/AmazonCloudFront/latest/DeveloperGuide/using-https-viewers-to-cloudfront.html
    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 0
    max_ttl                = 0
  }

  restrictions {
    # https://docs.aws.amazon.com/ja_jp/AmazonCloudFront/latest/DeveloperGuide/georestrictions.html
    geo_restriction {
      restriction_type = "none"
    }
  }
  # 通信経路に使う証明書を指定する
  viewer_certificate {
    # 今はとりあえずデフォルトの証明書を使う
    cloudfront_default_certificate = true
  }

  tags = {
    Name = "api_gw_cloudfront"
  }

  price_class = "PriceClass_All" # すべての価格クラスのエッジロケーションを使用する

  # アクセスログをS3に保存する
  logging_config {
    bucket          = aws_s3_bucket.cloudfront_logging.bucket_domain_name
    include_cookies = false
    prefix          = "cloudfront/"
  }
}

output "cf_dist_endpoint" {
  value = aws_cloudfront_distribution.cf_dist.domain_name
}












