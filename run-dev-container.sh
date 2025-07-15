#!/bin/sh

source ./env.sh
docker run \
    -v $(pwd):/root/ \
    -v $(pwd)/bin/config:/etc/seacan \
    -v $(pwd)/bin/log:/var/log/seacan \
    -it ${SEACAN_DEV_CONTAINER_NAME} \
    /bin/sh
