#!/bin/bash

queue_url=`aws cloudformation describe-stacks --stack-name event-processing-demo | jq -r '.Stacks[0].Outputs[1].OutputValue'`

aws sqs send-message --queue-url \
  $queue_url \
  --message-body \
  '{"user_id": "1234", "service_id": "service_id", "service_name": "A service", "timestamp": "now"}'
