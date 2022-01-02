const db = require('./');
const bcrypt = require('bcrypt');

const saltRounds = Number(process.env.SALT_ROUNDS || 31);

async function createUser(username, password) {
  try {
    const hash = await hashPassword(password);
    return db("users").insert({username, password: hash}).returning("id");
  } catch (error) {
    console.error("error hashing password", error);
    throw error
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