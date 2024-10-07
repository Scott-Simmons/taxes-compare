FROM node:20.10.0-alpine AS build

WORKDIR /app

ENV PATH /app/node_modules/.bin:$PATH

COPY package.json package-lock.json ./
RUN NODE_ENV=production npm ci

COPY src/ src/
COPY public/ public/
COPY tsconfig.json tsconfig.json

RUN npm run build

FROM nginx:1.22.0-alpine

COPY --from=build /app/build/ /usr/share/nginx/html
COPY public/config.js /usr/share/nginx/html/config.js
COPY ./etc/nginx/nginx.conf.template /etc/nginx/nginx.conf.template
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
