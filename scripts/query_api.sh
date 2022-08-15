#!/bin/bash

api_url=`aws cloudformation describe-stacks --stack-name event-processing-demo | jq -r '.Stacks[0].Outputs[0].OutputValue'`

query_url="${api_url}?user_id=1234"

curl $query_url
