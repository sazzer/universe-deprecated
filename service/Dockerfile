FROM node:13.8.0-slim AS builder

WORKDIR /universe

# Make use of Docker Layer Caching for dependencies
COPY ./package.json ./tsconfig.json ./yarn.lock /universe/
RUN yarn install

COPY ./src /universe/src
COPY ./config /universe/config
RUN yarn build

CMD ["node", "build/index.js"]
