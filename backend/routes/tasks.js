const express = require('express');

const router = express.Router();

router.route("/")
  .get((req, res) => {
    return res.json({
      message: "Now I'm in a router"
    })
  })

module.exports = router;
