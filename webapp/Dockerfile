FROM node:13.7.0-slim AS builder

WORKDIR /universe

# Make use of Docker Layer Caching for dependencies
COPY ./package.json ./tsconfig.json ./yarn.lock /universe/
RUN yarn install

COPY ./src /universe/src
COPY ./public /universe/public
RUN yarn build

FROM nginx:1.17.8-alpine
COPY --from=builder /universe/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
