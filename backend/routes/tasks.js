const express = require('express');

const router = express.Router();

router.route("/")
  .post((req, res) => {
    console.log("creating a task");
    return res.json({
      message: "Now I'm in a router"
    })
  })

module.exports = router;
