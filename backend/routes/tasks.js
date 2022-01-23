const express = require('express');
const {authenticate} = require('./utilities');
const {insertTask, getAllUsersTasks, getOneUsersTask} = require("../database/taskQueries");

const router = express.Router();

router.route("/")
  .all(authenticate)
  .post(async (req, res, next) => {
    const {
      priority = null,
      title,
      completed_at = null,
      description,
    } = req.body;

    if(!title) return res.status(400).json({error: "missing task title"});

    try {
      const createdTask = await insertTask(priority, title, completed_at, description, req.user.id);
      return res.json({
        data: createdTask
      })
    } catch (error) {
      return next(error);
    }
  })
  .get(async (req, res, next) => {
    try {
      const tasks = await getAllUsersTasks(req.user.id);
      res.json({data: tasks});
    } catch(error) {
      return next(error);
    }
  });

router.route("/:taskId")
  .all(authenticate)
  .get(async (req, res, next) => {
    try {
      const task = await getOneUsersTask(req.user.id, req.params.taskId);
      if(!task) return res.status(404).json({error: "not found"});
      return res.json({data: task});
    } catch (error) {
      return next(error);
    }
  })

module.exports = router;
