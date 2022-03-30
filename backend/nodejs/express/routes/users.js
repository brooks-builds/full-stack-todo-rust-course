const express = require("express");
const userQueries = require("../database/userQueries");
const jwt = require("jsonwebtoken");
const jwtSecret = process.env.JWT_SECRET;
const bcrypt = require("bcrypt");
const { authenticate } = require("./utilities");
const { getDefaultTasks, insertTask } = require("../database/taskQueries");

const router = express.Router();

router.route("/").post(async (req, res) => {
  const { username, password } = req.body;
  try {
    const newUser = await userQueries.createUser(
      username,
      password,
      createToken({ username })
    );
    const defaultTasks = await getDefaultTasks();
    const insertTaskPromises = defaultTasks.map((defaultTask) => {
      return insertTask(
        defaultTask.priority,
        defaultTask.title,
        null,
        defaultTask.description,
        newUser.id
      );
    });
    await Promise.all(insertTaskPromises);
    res.json({
      data: newUser,
    });
  } catch (error) {
    console.error(error.message);
    res.status(400).json({ error: error.message });
  }
});

router
  .route("/logout")
  .all(authenticate)
  .post(async (req, res) => {
    try {
      const token = req.headers["x-auth-token"];
      const result = await userQueries.findAndRemoveToken(token);
      res.json({ message: "user logged out" });
    } catch (error) {
      res.status(500).json({ error: error.message });
    }
  });

router.route("/login").post(async (req, res) => {
  const { username, password } = req.body;
  try {
    const dbUser = await userQueries.getByUsername(username);
    if (!dbUser) {
      const error = new Error("incorrect username and/or password");
      error.code = 400;
      throw error;
    }
    const matchedPassword = await bcrypt.compare(password, dbUser.password);
    if (!matchedPassword) {
      const error = new Error("incorrect username or password");
      error.code = 400;
      throw error;
    }
    dbUser.token = createToken({ username: dbUser.username });
    await userQueries.addTokenToUser(dbUser.token, dbUser.id);
    delete dbUser.deleted_at;
    delete dbUser.password;
    res.json({ data: dbUser });
  } catch (error) {
    res.status(error.code || 500).json({ error: error.message });
  }
});

function createToken(data) {
  return jwt.sign(data, jwtSecret);
}

module.exports = router;
