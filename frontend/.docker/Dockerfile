FROM node:16 AS builder

WORKDIR /app

COPY --chown=node:node package*.json .
RUN npm install

COPY --chown=node:node . .
USER node

CMD npm install; npm start
