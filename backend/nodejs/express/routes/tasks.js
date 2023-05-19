const express = require('express');
const {authenticate} = require('./utilities');
const {insertTask, getAllUsersTasks, getOneUsersTask, markTaskAsCompleted, markTaskAsUncompleted, updateTask, softDeleteTask} = require("../database/taskQueries");

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
        data: createdTask[0]
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
  .patch(async (req, res, next) => {
    try {
      const {priority, title, description, completed_at} = req.body;
      const count = await updateTask(req.user.id, req.params.taskId, {priority, title, description, completed_at});
      if(count) {
        return res.sendStatus(200);
      }

      const error = new Error("Not found");
      error.code = 404;
      throw error;
    } catch (error) {
      return next(error);
    }
  })
  .delete(async (req, res, next) => {
    try {
      const count = await softDeleteTask(req.user.id, req.params.taskId);
      if(count) {
        return res.sendStatus(200);
      }

      const error = new Error("Not found");
      error.code = 404;
      throw error;
    } catch(error) {
      return next(error);
    }
  })

router.route("/:taskId/completed")
  .all(authenticate)
  .put(async (req, res, next) => {
    const {taskId} = req.params;
    const userId = req.user.id;
    try {
      const completedCount = await markTaskAsCompleted(userId, taskId);
      if(completedCount) {
        return res.sendStatus(200);
      }

      const error = new Error("not found");
      error.code = 404;
      throw error;
    } catch (error) {
      return next(error);
    }
  })

router.route("/:taskId/uncompleted")
  .all(authenticate)
  .put(async (req, res, next) => {
    const {taskId} = req.params;
    const userId = req.user.id;
    try {
      const resultCount = await markTaskAsUncompleted(userId, taskId);
      if(resultCount) {
        return res.sendStatus(200);
      }

      const error = new Error("not found");
      error.code = 404;
      throw error;
    } catch (error) {
      return next(error);
    }
  })

module.exports = router;
