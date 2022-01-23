const { getByToken } = require("../database/userQueries");

async function authenticate(req, res, next) {
  const token = req.headers["x-auth-token"];
  if(!token) {
    const error = new Error("not authenticated!");
    error.code = 401;
    return next(error);
  }
  
  try {
    const user = await getByToken(token);
    if(!user) {
      const error = new Error("not authenticated!");
      error.code = 401;
      return next(error);
    }
    req.user = user;
    return next();
  } catch (error) {
    return next(error);
  }
}

module.exports = {
  authenticate,
}