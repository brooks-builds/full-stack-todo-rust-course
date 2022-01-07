require("dotenv").config();
const { default: axios } = require("axios");

const apiPort = process.env.API_PORT;
const baseUrl = `${process.env.API_URI}:${apiPort}/api/v1`;

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
      // check the token to make sure that it is signed correctly
      // check the database to make sure that our new user is there and the password is hashed correcly
    });

    test.todo("sign in");
    test.todo("log out");
  })
})