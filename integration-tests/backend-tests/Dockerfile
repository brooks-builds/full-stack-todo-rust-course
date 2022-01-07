FROM node:14
VOLUME /code
WORKDIR /code
RUN npm i
ENV API_PORT=3000
ENV API_URI=http://localhost
CMD npm run test:watch