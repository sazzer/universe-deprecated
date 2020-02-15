FROM node:13.8.0-slim AS builder

WORKDIR /universe

# Make use of Docker Layer Caching for dependencies
COPY ./package.json ./tsconfig.json ./yarn.lock /universe/
RUN yarn install

COPY ./src /universe/src
COPY ./public /universe/public
RUN yarn build

FROM nginx:1.17.8-alpine
RUN set -x && \
    apk add --update libintl && \
    apk add --virtual build_deps gettext &&  \
    cp /usr/bin/envsubst /usr/local/bin/envsubst && \
    apk del build_deps

COPY --from=builder /universe/build /usr/share/nginx/html
COPY docker/start.sh /start.sh

EXPOSE 80
CMD ["/start.sh"]
