FROM node:16
RUN apt-get update && apt-get install -y xvfb libnss3 libatk1.0-0 libatk-bridge2.0-0 libgtk-3-0 libgbm1 libasound2
RUN npm i -g cypress
VOLUME /code
WORKDIR /code
CMD cypress run --no-exit