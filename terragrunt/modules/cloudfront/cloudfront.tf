# WebApi用のCloudFront
resource "aws_cloudfront_distribution" "cf_dist" {
  enabled    = true
  web_acl_id = aws_wafv2_web_acl.waf.arn
  origin {
    domain_name = replace(var.rest_api_invoke_url, "/^https?://([^/]*).*/", "$1")
    origin_id   = "${var.origin_name}_root}"

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
    target_origin_id = "${var.origin_name}_root}"
    compress         = var.compress # 高速化のためコンテンツ圧縮(gzip)

    forwarded_values {
      query_string = false

      cookies {
        forward = "all" # 全てのCookieを転送する。
      }
    }
    # httpsにリダイレクトしてWebApiなのでキャッシュはしない
    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 0
    max_ttl                = 0

  }

  ordered_cache_behavior {
    path_pattern     = "/*"
    allowed_methods  = ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "${var.origin_name}_root}"
    compress         = var.compress # 高速化のためコンテンツ圧縮(gzip)

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
    Name      = "${var.project_name}-${var.project_stage}-cloudfront"
    ManagedBy = "Terraform"
  }

  price_class = var.price_class # すべての価格クラスのエッジロケーションを使用する

  # アクセスログをS3に保存する
  logging_config {
    bucket          = aws_s3_bucket.cloudfront_logging.bucket_domain_name
    include_cookies = false
    prefix          = "${var.project_name}-cloudFront/${var.project_stage}/"
  }

  retain_on_delete = false # TerraformでCloudFrontを削除したいため無効。

  # ディストリビューションのステータスが「InProgress」→「Deployed」に変わることを待つかどうかの設定。
  wait_for_deployment = true
}
