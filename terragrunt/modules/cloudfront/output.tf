
# output "waf_arn" {
#     value = aws_wafv2_web_acl.waf.arn
# }

output "cloudfront_domain_name" {
    value = aws_cloudfront_distribution.cf_dist.domain_name  
}

