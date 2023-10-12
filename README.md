
# サーバーレスなWebAPIをIaCで構築・運用するCI/CDを構築（AWS・CI/CDなどのDevOps練習）
- AWS・CI/CDなどのDevOps勉強・練習する為にバックエンド(WebAPI)をサーバーレスアーキテクチャで構築した

# AWS構成
- WAF->CloudFront(CDN)->API Gateway->Lambda->DynamoDB
  - CloudFront->S3にアクセスログを保存
  - APIGWはクラウドウォッチにログを保存
- セキュリティ対策として、CloudFrontの前段にWAFを設定する
  - 無料枠の対象外なのが痛いところ
    - 設定するだけで課金されていくので注意する
# 使用技術一覧やコンセプト
- Lambda動作言語：Rust（とりあえず書き慣れていたから）
  - DockerコンテナにしてAWS ECRにデプロイ
    - コードをDockerコンテナにしてデプロイすることで、開発環境・テスト・デプロイの統一・容易性を実現
- CI/CDパイプライン:Github Actions
- クラウドインフラ構築:IaCであるTerraform+Terragruntを使用
    - 素のテラフォームだとステージごとにコードをコピペする必要があり（Not DRY・IaCコードの保守性が悪化）、実行順序にも気をつけなければならないなどの問題がある
    - そこでIaCのDRYが容易に行え、実行順序も調整してくれるラッパーであるTerragruntを採用
- APIGWの前段にCloudFront+WAF
  - APIのレスポンスタイムを大幅に短縮することができる
    - また、CloudFrontは世界各地にあるエッジロケーションを使用してリクエストをキャッシュし、リージョン間で転送するため、APIのレスポンスタイムを大幅に短縮することができる
    - これにより、エンドユーザーはより高速にWeb APIにアクセスできる
  - WAFを入れる事で、DDoS攻撃やSQLインジェクションなどの攻撃を防ぐことができる
    - また、CloudFrontにはIPアドレス制限機能があるので、特定のIPアドレスからのアクセスのみを許可することもできる

# 工夫したところ/苦労したところ
- AWSとGithubとのOIDC認証をする事で、Github ActionsでインフラのCI/CDが実行出来るようにした
- `AWS`+`OIDC認証`+`IaC`+`CI/CD(Github Actions)`+`Terraform`+`Terragrunt`+`Docker`など、全てが未挑戦の技術が目白押しで、かなり苦労した
  - また、AWSのサービスが多すぎて、どのサービスを使えばいいのか・どう設定すれば良いのかわからなかった
    - 例えば、LambdaのログをCloudWatchに保存するのかS3に保存するのか、CloudFrontのログをS3に保存するのかCloudWatchに保存するのかなど
    - ベストプラクティスがわからないので、ググりまくった
  - ググりまくり、プログラミングノートに情報・知見を溜めつつ、PDCAサイクルを回していった
  - 最終的には、コミットするとサービスのCI/CDが自動化する所まで実装できた
    - ただ、まだまだ改善の余地があるので、今後も改善していく

## 備忘録:GHAにて
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
