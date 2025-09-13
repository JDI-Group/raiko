#! /bin/bash

# Determine image type from first parameter
if [ "_$1" == "_pccs" ]; then
    SOURCE_IMAGE_FULLNAME=us-docker.pkg.dev/moonchain-com/images/pccs:latest
    IMAGE_NAME=us-docker.pkg.dev/moonchain-com/images/pccs
    shift # Remove first parameter
elif [ "_$1" == "_raiko" ]; then
    SOURCE_IMAGE_FULLNAME=us-docker.pkg.dev/moonchain-com/images/raiko:latest
    IMAGE_NAME=us-docker.pkg.dev/moonchain-com/images/raiko
    shift # Remove first parameter
else
    # Default to raiko for backward compatibility
    SOURCE_IMAGE_FULLNAME=us-docker.pkg.dev/moonchain-com/images/raiko:latest
    IMAGE_NAME=us-docker.pkg.dev/moonchain-com/images/raiko
fi

# Set image tag from remaining parameters
if [ "_$1" == "_" ]; then
    IMAGE_TAG="latest"
elif [ "_$1" == "_moonchain" ]; then
    IMAGE_TAG="latest"
elif [ "_$1" == "_main" ]; then
    IMAGE_TAG="latest"
else
    IMAGE_TAG=`echo "$1" | sed 's/\//-/g'`
fi
IMAGE_FULLNAME="${IMAGE_NAME}:${IMAGE_TAG}"

if [ "${SOURCE_IMAGE_FULLNAME}" != "${IMAGE_FULLNAME}" ]; then
    docker image tag ${SOURCE_IMAGE_FULLNAME} ${IMAGE_FULLNAME}
fi

docker push ${IMAGE_FULLNAME}