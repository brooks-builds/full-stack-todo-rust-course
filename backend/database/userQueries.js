const db = require('./');
const bcrypt = require('bcrypt');

const saltRounds = Number(process.env.SALT_ROUNDS || 31);

async function createUser(username, password) {
  try {
    const hash = await hashPassword(password);
    const [newUser] = await db("users").insert({username, password: hash}).returning(["id", "username"]);
    return newUser
  } catch (error) {
    const errors = {
      "23505": "Username already taken, try again with a different user name"
    }

    throw new Error (errors[error.code] || error.message);
  }
}

function hashPassword(password) {
  return new Promise((resolve, reject) => {
    bcrypt.hash(password, saltRounds, (error, hash) => {
      if (error) return reject(error);
      return resolve(hash);
    });
  })
}

module.exports = {
  createUser
}