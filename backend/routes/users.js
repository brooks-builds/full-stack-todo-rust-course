const express = require('express');
const userQueries = require('../database/userQueries');

const router = express.Router();

router
  .route("/")
  .post(async (req, res) => {
    const {username, password} = req.body;
    const userId = await userQueries.createUser(username, password);
    res.json({
      data: {
        userId
      },
      message: "user created successfully"
    })
  })

router.route('/:id')
  .get((req, res) => {
    // get one user
  })

router.route('/login')
  .post((req, res) => {
    // log in the user
  })

module.exports = router;