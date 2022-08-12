#!/bin/bash

cd event-handler
cargo lambda build --release --arm64

cd ../infrastructure
sam deploy
