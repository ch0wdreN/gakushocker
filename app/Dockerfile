FROM node:19.6-alpine
WORKDIR /usr/src/app

COPY ./app/package.json ./
RUN apk add --update python3 make g++ && rm -rf /var/cache/apk/*
RUN yarn install
COPY . .
EXPOSE 3000
CMD ["yarn", "run", "dev"]
