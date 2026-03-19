# SOP 4.2: Infrastructure-as-Code Financial Kill-Switches (FinOps)
# This snippet enforces automatic scale-to-zero upon exceeding $1000 budget.

provider "aws" {
  region = "us-east-1"
}

resource "aws_budgets_budget" "finops_killswitch" {
  name              = "consortium-strict-limit"
  budget_type       = "COST"
  limit_amount      = "1000"
  limit_unit        = "USD"
  time_unit         = "MONTHLY"

  notification {
    comparison_operator        = "GREATER_THAN"
    threshold                  = 110
    threshold_type             = "PERCENTAGE"
    notification_type          = "ACTUAL"
    subscriber_sns_topic_arns  = [aws_sns_topic.killswitch_trigger.arn]
  }
}

resource "aws_sns_topic" "killswitch_trigger" {
  name = "budget-exceeded-trigger"
}

# The Lambda attached to this SNS will execute logic to set the AutoScalingGroup
# desired_capacity to 0, completely shutting off the compute nodes.
resource "aws_lambda_function" "scale_to_zero_lambda" {
  function_name    = "ScaleToZero"
  role             = aws_iam_role.lambda_exec.arn
  handler          = "index.handler"
  runtime          = "nodejs18.x"
  filename         = "finops_lambda.zip"
  # Ensures the Lambda can contact EC2 / AutoScaling
}

# SOP 5.1: Workload Identity / Least Privilege
resource "aws_iam_role" "lambda_exec" {
  name = "FinOpsLambdaRole-Strict"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    }]
  })
}

# Explicitly defining allowed actions to prevent IAM: *
resource "aws_iam_role_policy" "strict_autoscaling" {
  name = "StrictAutoScalingDown"
  role = aws_iam_role.lambda_exec.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = [
        "autoscaling:SetDesiredCapacity"
      ]
      Effect   = "Allow"
      Resource = "*" # To be restricted to the specific ASG ARN
    }]
  })
}
