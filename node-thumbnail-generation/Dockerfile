FROM node:18-alpine

WORKDIR /app

RUN apk add --no-cache vips-dev build-base libheif

COPY package*.json ./

RUN npm install

COPY . .

CMD [ "node", "index.js" ]