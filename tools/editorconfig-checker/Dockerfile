FROM node:16.4.0-alpine3.13

# Needed for ignoring git-related files
RUN apk add --no-cache git

WORKDIR /app

RUN chown -R node:node /app

USER node:node

COPY package.json package.json
COPY package-lock.json package-lock.json

RUN npm ci

CMD [ "npm", "run", "editorconfig-checker" ]
