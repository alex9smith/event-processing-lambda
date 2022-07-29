# Event processing demo in AWS Lambda

This repo demos a system which listens to events emitted from an OIDC Identity Provider (IDp) as users log in to different Relying Parties (RPs) and stores a record of which RPs a user has logged in to.

Events come into the system on an SQS queue and are processed by an event handler Lanbda function. This function reads the event and updates the datastore (DynamoDB). A second Lambda function acts as a query API where another application can provide a user ID and get back the list of RPs for that user.

