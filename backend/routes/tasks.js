const express = require('express');
const {authenticate} = require('./utilities');
const {insertTask} = require("../database/taskQueries");

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

module.exports = router;
