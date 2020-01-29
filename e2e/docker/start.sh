#!/bin/sh

dockerize -timeout 30s $DOCKERIZE_PARAMS yarn start
