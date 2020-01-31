#!/bin/sh

echo '=== Starting Universe UI ==='
cat /usr/share/nginx/html/config.js.template | envsubst > /usr/share/nginx/html/config.js

echo Configuration:
cat /usr/share/nginx/html/config.js

nginx -g 'daemon off;'
