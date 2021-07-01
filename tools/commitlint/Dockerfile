FROM node:16.3.0-alpine3.13

# Needed for getting the commit messages
RUN apk add --no-cache git

WORKDIR /app

RUN chown -R node:node /app

USER node:node

COPY package.json package.json
COPY package-lock.json package-lock.json

RUN npm ci

COPY commitlint.config.js commitlint.config.js

CMD [ "npm", "run", "commitlint" ]
