const db = require("./");
const bcrypt = require("bcrypt");

const saltRounds = Number(process.env.SALT_ROUNDS || 31);

async function createUser(username, password, token) {
  try {
    const hash = await hashPassword(password);
    const [newUser] = await db("users")
      .insert({ username, password: hash, token })
      .returning(["id", "username", "token"]);
    return newUser;
  } catch (error) {
    const errors = {
      23505: "Username already taken, try again with a different user name",
    };

    throw new Error(errors[error.code] || error.message);
  }
}

function hashPassword(password) {
  return new Promise((resolve, reject) => {
    bcrypt.hash(password, saltRounds, (error, hash) => {
      if (error) return reject(error);
      return resolve(hash);
    });
  });
}

function getByUsername(username) {
  return db("users").where("username", username).first();
}

function findAndRemoveToken(token) {
  return db("users").where("token", token).update({ token: null });
}

function getByToken(token) {
  return db("users")
    .select(["id", "username", "token"])
    .where("token", token)
    .andWhere("deleted_at", null)
    .first();
}

function addTokenToUser(token, userId) {
  return db("users").update({ token }).where({ id: userId });
}

module.exports = {
  createUser,
  getByUsername,
  findAndRemoveToken,
  getByToken,
  addTokenToUser,
};
