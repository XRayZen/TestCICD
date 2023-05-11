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


GitHub Copilot: `cargo-aws-lambda`は、AWS Lambda関数をビルドするためのRustツールチェーンです。このツールチェーンを使用すると、AWS Lambda関数をビルドするために必要なすべての依存関係を含む単一のバイナリをビルドできます。このバイナリは、AWS Lambdaのランタイムに直接アップロードできます。

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
