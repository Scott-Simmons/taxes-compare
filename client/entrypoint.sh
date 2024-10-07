#!/bin/sh

set -e

envsubst '${NGINX_PORT},${REACT_APP_BACKEND_PORT}' < /etc/nginx/nginx.conf.template > /etc/nginx/nginx.conf

sed -i "s|localhost:3001|${REACT_APP_BACKEND_HOST}|g" /usr/share/nginx/html/config.js
sed -i "s|http|${REACT_APP_BACKEND_PROTOCOL}|g" /usr/share/nginx/html/config.js

nginx -g 'daemon off;'
