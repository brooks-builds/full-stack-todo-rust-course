require("dotenv").config();
const { default: axios } = require("axios");
const jwt = require('jsonwebtoken');
const knex = require('knex');
const dbConfiguration = require("./knexfile");
const bcrypt = require("bcrypt");

const apiPort = process.env.API_PORT;
const baseUrl = `${process.env.API_URI}:${apiPort}/api/v1`;
const JWT_SECRET = process.env.JWT_SECRET;
const db = knex(dbConfiguration);

describe("todo api", () => {
  describe("User CRUD", () => {
    const userToCreate = {
      username: `testuser${Date.now()}`,
      password: `${Date.now()}`,
    }

    test("sign up", async () => {
      const {data: response} = await axios.post(`${baseUrl}/users`, userToCreate);
      expect(response).toHaveProperty("data");
      const {data} = response;
      checkLoggedInUser(data, userToCreate);
      const createdUser = await db.table("users").where("username", userToCreate.username).first();
      expect(createdUser.id).toBe(data.id);
      expect(createdUser.username).toBe(userToCreate.username);
      expect(createdUser.password).not.toBe(userToCreate.password);
      expect(createdUser.token).toBe(data.token);
      const passwordMatched = bcrypt.compareSync(userToCreate.password, createdUser.password);
      expect(passwordMatched).toBe(true);
    });
  
    test("cannot create multiple users with the same user name", async () => {
      const newUser = Object.assign({}, userToCreate);
      newUser.username += "!";
      await axios.post(`${baseUrl}/users`, newUser);
      let gotError = false;
      try {
        await axios.post(`${baseUrl}/users`, newUser);
      } catch (error) {
        expect(error.response.status).toBe(400);
        expect(error.response.data.error).toBe("Username already taken, try again with a different user name");
        gotError = true;
      }

      expect(gotError).toBe(true);
    });

    test("sign in", async () => {
      const loggedInUser = await axios.post(`${baseUrl}/users/login`, userToCreate);
      checkLoggedInUser(loggedInUser.data.data, userToCreate);
    });

    test("log out", async () => {
      const userToCreate = {
        username: "userjusttologout" + Date.now(),
        password: "test1234"
      }
      const createdUser = await axios.post(`${baseUrl}/users`, userToCreate);
      await axios({
        method: "POST",
        url: `${baseUrl}/users/logout`,
        headers: {
          "x-auth-token": createdUser.data.data.token
        }
      });
      const dbUser = await db.table("users").where("username", userToCreate.username).first();
      expect(dbUser.token).toBe(null);
    });
  })
})

function checkLoggedInUser(userFromApi, testUser) {
  expect(typeof userFromApi.id).toBe("number");
  expect(userFromApi.username).toBe(testUser.username);
  expect(typeof userFromApi.token).toBe("string");
  const token = jwt.verify(userFromApi.token, JWT_SECRET);
  expect(token.username).toBe(testUser.username);
}