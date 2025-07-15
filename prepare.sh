#!/bin/sh

source ./env.sh
docker build -t ${SEACAN_DEV_CONTAINER_NAME} build/dev-container
