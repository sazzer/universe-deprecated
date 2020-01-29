FROM node:13.6.0-alpine3.11

RUN apk add --no-cache openssl

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /universe

COPY package.json yarn.lock tsconfig.json docker/start.sh /universe/
RUN yarn install

COPY src /universe/src/
COPY features /universe/features/
COPY format.ts /universe/

CMD ["./start.sh"]
