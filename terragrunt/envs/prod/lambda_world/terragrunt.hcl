include "root" {
    path= find_in_parent_folders()
}

locals{
    env= read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform{
    source = "../../../modules/lambda"
}

dependency "ecr" {
    config_path = "../ecr"
    
    mock_outputs = {
        ecr_repository_url = "123456789012.dkr.ecr.ap-northeast-1.amazonaws.com/world"
    }
}

inputs = {
    lambda_function_name = "world"
    lambda_function_description = "world lambda function"
    repo_url= dependency.ecr.outputs.ecr_repository_url
    image_tag= "world"
    memory_size = 128
    timeout = 3
    managed_policy_arns = [
    # Lambda関数がCloudWatch Logsにログを書き込むための最低限の権限を提供します。
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    # Lambda関数がVPC内のリソースにアクセスしながら実行するための最低限の権限（ネットワークインターフェースの作成、記述、削除、CloudWatch Logsへの書き込み権限）を提供します。
    "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole",
    # Amazon ECR に対する読み取り専用アクセスを付与
    "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly",
    "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess",
    # APIGW用
    "arn:aws:iam::aws:policy/AmazonAPIGatewayInvokeFullAccess",
    ]
}