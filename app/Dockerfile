FROM node:16-alpine
WORKDIR /usr/src/app

COPY ./app/package.json ./
RUN yarn install
COPY . .
EXPOSE 3000
CMD ["yarn", "run", "dev"]


