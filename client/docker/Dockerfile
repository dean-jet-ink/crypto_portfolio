FROM node:22-bullseye-slim

WORKDIR /workspace

COPY package.json package-lock.json ./

RUN npm install

COPY . .

CMD ["npm", "run", "dev"]