#!/bin/bash
set -euxo pipefail

echo "Setting up COMMIT_SHA..."
echo "COMMIT_SHA=$(git rev-parse HEAD)" > .env

echo "Running orderbook prep-base..."
(cd lib/rain.orderbook && ./prep-base.sh)

echo "Setup complete!"
