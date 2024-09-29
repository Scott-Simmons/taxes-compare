FROM node:20.10.0-alpine AS build

WORKDIR /app

ENV PATH /app/node_modules/.bin:$PATH

# Defaults, overwritten in kube manifests
ENV HOST="localhost"
ENV PROTOCOL="http"
ENV PORT="8080"

COPY package.json package-lock.json ./
RUN NODE_ENV=production npm ci

COPY src/ src/
COPY public/ public/
COPY tsconfig.json tsconfig.json

RUN npm run build

FROM nginx:1.22.0-alpine
COPY ./etc/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/build/ /usr/share/nginx/html
CMD ["nginx", "-g", "daemon off;"]
