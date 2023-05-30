

## AWSとOIDCするためにIAMロールを作成する
```bash
aws cloudformation create-stack --stack-name TestCICDFix1 --template-body file://iam_role/iam-role.yaml --profile terraform-user --capabilities CAPABILITY_NAMED_IAM
```
- プロファイルを指定しない場合は`--profile terraform-user`を削除する

- show stack arn
```bash
aws cloudformation describe-stacks --stack-name TestCICDFix1 --query 'Stacks[0].Outputs[0].OutputValue' --output text --profile terraform-user
```

- delete stack
```bash
aws cloudformation delete-stack --stack-name TestCICDFix1 --profile AWSプロファイル名
```







