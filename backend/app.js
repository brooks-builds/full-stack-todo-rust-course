require('dotenv').config()

const bodyParser = require('body-parser');
const express = require('express');
const {tasksRouter, usersRouter} = require('./routes');

const app = express();
const port = process.env.PORT || 3000;

app.use(bodyParser.json());

app.use("/api/v1/tasks", tasksRouter);
app.use("/api/v1/users", usersRouter);

app.use((error, req, res, next) => {
  res.status(error.code || 500).json({error: error.message});
})

app.listen(port, () => {
  console.log(`backend listening on port ${port}`);
})