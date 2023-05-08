# テーブルを定義する
# 歴代の内閣総理大臣の名前と就任と辞任の日時データを格納するテーブルを作成する
resource "aws_dynamodb_table" "prime_ministers_table" {
  name           = "prime_ministers"
  # 読み込み/書き込みキャパシティーユニットの最小値を設定する
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "name"
  attribute {
    name = "name"
    type = "S"
  }
  attribute {
    name = "start_date"
    type = "S"
  }
  attribute {
    name = "end_date"
    type = "S"
  }
}



