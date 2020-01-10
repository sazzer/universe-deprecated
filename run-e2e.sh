#!/bin/sh

docker-compose -f docker-compose.e2e.yml down
docker-compose -f docker-compose.e2e.yml up --exit-code-from universe-e2e-tests --build
