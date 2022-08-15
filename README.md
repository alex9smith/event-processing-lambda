# Event processing demo in AWS Lambda

This repo demos a system which listens to events emitted from an OIDC Identity Provider (IDp) as users log in to different Relying Parties (RPs) and stores a record of which RPs a user has logged in to.

Events come into the system on an SQS queue and are processed by an event handler Lanbda function. This function reads the event and updates the datastore (DynamoDB). A second Lambda function acts as a query API where another application can provide a user ID and get back the list of RPs for that user.

## Data structure

Incoming event body:

```json
{
  "user_id": "123456789",
  "service_id": "abcdef",
  "service_name": "A service name",
  "timestamp": "2022-08-04T08:50:39Z"
}
```

Transformed by the event handler Lambda and stored in DynamoDB as:

```json
{
  "user_id": "123456789",
  "services": [
    {
      "service_id": "abcdef",
      "service_name": "A service name",
      "last_accessed": "2022-08-04T08:50:39Z"
    },
    {
      "service_id": "ghijkl",
      "service_name": "Another service name",
      "last_accessed": "2022-07-03T10:23:45Z"
    }
  ]
}
```

This stored data structure means we have a slower transformation and load step, but a fast query by `user_id` which is what we need for this use case as the query will be blocking a user-facing HTTP response.

## Running the demo

1. Make sure you have the AWS CLI installed and configured with credentials
2. Install the [AWS SAM CLI](https://aws.amazon.com/serverless/sam/)
3. From the root of the project run the deploy script

```bash
./scripts/deploy.sh
```

4. Put a sample event on the SQS queue by running

```bash
./scripts/write_sample_event_to_queue.sh
```

5. Look at the SQS, Lambda and DynamoDB monitoring in AWS to see the event travel through the system

6. Run the query script to call the query Lambda via API Gateway

```bash
./scripts/query_api.sh
```

7. Remove all resources used in this demo by deleting the Cloudformation stack

```bash
aws cloudformation delete-stack --stack-name event-processing-demo
```

## Deploying to production

This system is for demonstration purposes only and isn't ready to be deployed to a production environment.
This is a non-exhaustive, non-ordered list of what would need to happen before that:

- Add authentication to API Gateway
- Set up route mapping in API Gateway, rather than forwarding all requests to the query Lambda
- Add error handling and recovery to both Lambdas
- Add a dead-letter queue to store events that couldn't be parsed by the event handler
- Add alerting for the Lambdas, API Gateway and dead-letter queue
- Set retention periods and better names for Cloudwatch logs
