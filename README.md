# Rustで動くLambdaでサーバーレスなWebAPIをCICD構築して関数コードコンテナ・IaCで運用するCI/CDパイプラインを構築（DevOps練習）
- バックエンド・WebAPIはサーバーレスアーキテクチャで構築
- 組み合わせたAWSリソース
    - WAF->CloudFront(CDN)->API Gateway->Lambda->DynamoDB
        - CloudFront->S3にアクセスログを保存
        - APIGWはクラウドウォッチにログを保存
- CI/CDパイプラインにはGithub Actionsを使用する
- クラウドインフラ構築にはIaCであるTerraform+Terragruntを使用する
    - 素のテラフォームだとステージごとにコードをコピペする必要があり（Not DRY・IaCコードの保守性が悪化）、実行順序にも気をつけなければならないなどの問題がある
    - そこでIaCのDRYが容易に行え、実行順序も調整してくれるラッパーであるTerragruntを使う
- セキュリティ対策として、CloudFrontの前段にWAFを設定する
  - 無料枠の対象外なのが痛いところ
    - 設定するだけで課金されていくので注意する
  - 設定するだけでDDoS攻撃やSQLインジェクションなどの攻撃を防ぐことができる
  - また、CloudFrontにはIPアドレス制限機能があるので、特定のIPアドレスからのアクセスのみを許可することもできる

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


