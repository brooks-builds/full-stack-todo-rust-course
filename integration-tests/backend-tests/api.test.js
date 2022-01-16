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
      expect(typeof data.id).toBe("number");
      expect(data.username).toBe(userToCreate.username);
      expect(typeof data.token).toBe("string");
      const token = jwt.verify(data.token, JWT_SECRET);
      expect(token.username).toBe(userToCreate.username);
      expect(typeof token.id).toBe('number');
      expect(token.id).toBe(data.id);
      const createdUser = await db.table("users").where("username", userToCreate.username).first();
      expect(createdUser.id).toBe(data.id);
      expect(createdUser.username).toBe(userToCreate.username);
      expect(createdUser.password).not.toBe(userToCreate.password);
      const passwordMatched = bcrypt.compareSync(userToCreate.password, createdUser.password);
      expect(passwordMatched).toBe(true);
    });
  
    test.only("cannot create multiple users with the same user name", async () => {
      const newUser = Object.assign({}, userToCreate);
      newUser.username += "!";
      const {data: response} = await axios.post(`${baseUrl}/users`, userToCreate);
      let gotError = false;
      try {
        const response = await axios.post(`${baseUrl}/users`, userToCreate);
      } catch (error) {
        expect(error.response.status).toBe(400);
        expect(error.response.data.error).toBe("Username already taken, try again with a different user name");
        gotError = true;
      }

      expect(gotError).toBe(true);
    });

    test.todo("sign in");
    test.todo("log out");
  })
})