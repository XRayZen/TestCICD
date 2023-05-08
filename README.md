# RustでAWS Lambdaを作成しCI/CDパイプラインを構築する

```bash
aws cloudformation create-stack --stack-name TestCICDFix1 --template-body file://iam_role/iam-role.yaml --profile terraform-user --capabilities CAPABILITY_NAMED_IAM
```
- show stack arn
```bash
aws cloudformation describe-stacks --stack-name TestCICDFix1 --query 'Stacks[0].Outputs[0].OutputValue' --output text --profile terraform-user
```

- delete stack
```bash
aws cloudformation delete-stack --stack-name プロジェクト名-stack --profile AWSプロファイル名
```



