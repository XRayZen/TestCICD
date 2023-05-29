# RustでAWS Lambdaを使ったWebAPIバックエンドを構築して関数コードコンテナ・IaCで運用するCI/CDパイプラインを構築する（練習）
- 組み合わせたAWSリソース
    - CloudFront->API Gateway->Lambda->DynamoDB
        - CloudFront->S3にアクセスログを保存
        - APIGWはクラウドウォッチにログを保存
- CI/CDパイプラインにはGithub Actionsを使用する
- クラウド構築にはIaCであるTerraformを使用する
    - 素のテラフォームだとステージごとにコードをコピペする必要があり（Not DRY・IaCコードの保守性が悪化）、実行順序にも気をつけなければならないなどの問題がある
    - そこでIaCのDRYが容易に行え、実行順序も調整してくれるラッパーであるTerragruntを使う

## CloudFrontとAPIGWを組み合わせるメリット
CloudFrontとAPI Gatewayを組み合わせることには、次のようなメリットがあります。

1. 高速化されたWeb API：CloudFrontは世界各地にあるエッジロケーションを使用してリクエストをキャッシュし、リージョン間で転送するため、APIのレスポンスタイムを大幅に短縮することができます。これにより、エンドユーザーはより高速にWeb APIにアクセスできます。

2. コスト削減：CloudFrontは、デフォルトでAmazon S3バケット、EC2インスタンス、Elastic Load Balancer、API GatewayなどのAWSサービスに対応しています。これらのサービスは、リクエスト料金やデータ転送料金を支払う必要がある場合がありますが、CloudFrontを使用することでこれらの費用を削減することができます。

3. セキュリティ：CloudFrontは、DDoS攻撃やマルウェア攻撃に対して強力な保護策を提供します。CloudFrontを使用することで、API Gatewayに到達する前に、悪意のあるリクエストへのアクセスが制限されます。

4. カスタムドメインサポート：CloudFrontを使用すると、また別のコストがかかるカスタムドメイン名を簡単に設定できます。これにより、API Gatewayのエンドポイントを使用せずに、特定のドメインに関連付けられたエッジロケーションの配布を作成できます。

5. スケーラビリティ：CloudFrontは、世界各地にあるエッジロケーションを使用して、Web APIのレスポンスを処理し、分散させることができます。これにより、エンドユーザーの数が増えても、APIのパフォーマンスが維持されます。

以上のようなメリットがあり、API GatewayとCloudFrontを組み合わせることで、高速で安全かつスケーラブルなWeb APIを実現できます。

## GHAにて
>[参考元](https://zenn.dev/snowcait/scraps/9d9c47dc4d0414)
- いちいちコミットするたびにActionを動かすとテスト・デプロイに時間がかかりすぎるのでコミットメッセージで実行するジョブを制御する
  - `lambda`を入れるとLambdaのCICDジョブが実行される
  - `infra`を入れるとクラウドインフラのCICDジョブが実行される
```yaml
  deploy_infrastructure:
        # コミットメッセージにinfraが含まれている場合は実行
    if: ${{ contains(github.event.head_commit.message, 'infra') }}
    runs-on: ubuntu-latest
```
- 最初は変更が加えられたフォルダにしようとしたがそれ用の機能が削除されていたのか動かなかった

##


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

## cargo-aws-lambda で　 Dockerfile を作成する
`cargo-aws-lambda`は、AWS Lambda関数をビルドするためのRustツールチェーンです。このツールチェーンを使用すると、AWS Lambda関数をビルドするために必要なすべての依存関係を含む単一のバイナリをビルドできます。このバイナリは、AWS Lambdaのランタイムに直接アップロードできます。

`cargo-aws-lambda`を使用する場合、Dockerコンテナにカスタムランタイムを含める必要はありません。AWS Lambdaのランタイムは、AWSが提供するものを使用するため、ランタイムの準備は不要です。`cargo-aws-lambda`は、AWS Lambda関数をビルドするために必要なすべての依存関係を含む単一のバイナリをビルドするため、Dockerコンテナを使用する必要もありません。

以下は、`cargo-aws-lambda`を使用してAWS Lambda関数をビルドするためのDockerfileの例です。

```Dockerfile
# Dockerfile for building Rust AWS Lambda function using cargo-aws-lambda

# Use the official Rust image as the base image
FROM rust:1.55 as builder

# Set the working directory
WORKDIR /usr/src/my-lambda-function

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the Lambda function
RUN cargo install cargo-aws-lambda
RUN cargo lambda build --release --bin my-lambda-function

# Use the official Amazon Linux image as the base image
FROM amazonlinux:2

# Copy the Lambda function binary to the container
COPY --from=builder /usr/src/my-lambda-function/target/lambda/release/my-lambda-function /var/task/

# Set the CMD to the Lambda function handler
CMD [ "my-lambda-function" ]
```

このDockerfileは、2つのステージで構成されています。最初のステージでは、Rustのビルド環境を構築し、AWS Lambda関数をビルドします。2番目のステージでは、AWS Lambda関数を実行するための実行環境を構築します。

最初のステージでは、`rust:1.55`イメージをベースイメージとして使用しています。`WORKDIR`ディレクティブを使用して、作業ディレクトリを`/usr/src/my-lambda-function`に設定します。`COPY`ディレクティブを使用して、`Cargo.toml`と`Cargo.lock`ファイルをコンテナにコピーします。`RUN`ディレクティブを使用して、`cargo-aws-lambda`をインストールします。`cargo lambda build`コマンドを使用して、AWS Lambda関数をビルドします。`--release`フラグを使用して、リリースビルドを実行します。これにより、最適化されたバイナリが生成されます。`--bin`フラグを使用して、ビルドするバイナリを指定します。

2番目のステージでは、`amazonlinux:2`イメージをベースイメージとして使用しています。`COPY`ディレクティブを使用して、最初のステージでビルドしたAWS Lambda関数のバイナリをコンテナにコピーします。`CMD`ディレクティブを使用して、AWS Lambda関数のハンドラを指定します。この例では、`my-lambda-function`という名前のバイナリを指定しています。

このDockerfileを使用して、AWS Lambda関数をビルドするには、以下のコマンドを実行します。

```
docker build -t my-lambda-function .
```

このコマンドは、カレントディレクトリにあるDockerfileを使用して、`my-lambda-function`という名前のDockerイメージをビルドします。ビルドが完了すると、AWS Lambda関数を実行するためのDockerイメージが作成されます。
