const knex = require("knex");
const configuration = require("../knexfile");

const db = knex(configuration);

module.exports = db;
