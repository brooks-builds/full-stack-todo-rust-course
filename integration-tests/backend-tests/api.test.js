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

jest.setTimeout(10000)

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

    test("should get an error when using an authenticated route without a token", async () => {
      let receivedError = false;
      try {
        await axios.post(`${baseUrl}/users/logout`, {});
      } catch (error) {
        receivedError = true;
        expect(error.response.status).toBe(401);
        expect(error.response.data.error).toBe("not authenticated!");
      }
      expect(receivedError).toBe(true);
    });
    
    test("should get an error when using an authenticated route with a fake token", async () => {
      let receivedError = false;
      try {
        await axios.post(`${baseUrl}/users/logout`, {}, {headers: {"x-auth-token": "fake token"}});
      } catch (error) {
        receivedError = true;
        expect(error.response.status).toBe(401);
        expect(error.response.data.error).toBe("not authenticated!");
      }
      expect(receivedError).toBe(true);
    });
  });

  describe("Task CRUD", () => {
    let user;
    let headers;
    
    beforeAll(async () => {
      [user, headers] = await createUser();
    });

    describe("create a task", () => {
      test("should be able to create a task", async () => {
        const newTask = {
          priority: "A",
          title: `this is a test task - ${Date.now()}`,
          description: "This task is the best task"
        };
        const result = await createTask(headers, newTask);
        const createdTask = result.data.data;
        expect(typeof createdTask.id).toBe("number");
        expect(createdTask.priority).toBe("A");
        expect(createdTask.title).toBe(newTask.title);
        expect(createdTask.completed_at).toBe(null);
        expect(createdTask.description).toBe(newTask.description);
        expect(createdTask).not.toHaveProperty("user_id");
        expect(createdTask).not.toHaveProperty("is_default");
      });
      test("should not be able to create a task when not logged in", async () => {
        let gotError = false;
        try {
          await axios.post(`${baseUrl}/tasks`);
        } catch (error) {
          gotError = true;
          expect(error.response.data.error).toBe("not authenticated!");
          expect(error.response.status).toBe(401);
        }

        expect(gotError).toBe(true);
      });
      test("cannot create a task without all required data", async () => {
        let gotError = false;
        try {
          await axios.post(`${baseUrl}/tasks`, {}, {headers});
        } catch(error) {
          gotError = true;
          expect(error.response.data.error).toEqual("missing task title");
          expect(error.response.status).toBe(400);
        };

        expect(gotError).toBe(true);
      });
    })

    describe("get all tasks", () => {
      let getAllTasksUser;
      let getAllTasksHeaders;

      beforeAll(async () => {
        [getAllTasksUser, getAllTasksHeaders] = await createUser();

        await createTask(getAllTasksHeaders, {title: "task 1"});
        await createTask(getAllTasksHeaders, {title: "task 2"});
        await createTask(getAllTasksHeaders, {title: "task 3"});
      });

      test("should be able to get all my tasks", async () => {
        const createdTasks = await axios.get(`${baseUrl}/tasks`, {headers: getAllTasksHeaders});
        expect(createdTasks.data.data.length).toBe(3);
        expect(createdTasks.data.data[0].title).toBe("task 1");
        expect(createdTasks.data.data[1].title).toBe("task 2");
        expect(createdTasks.data.data[2].title).toBe("task 3");
      });
      test.todo("should not be able to get any tasks when logged out");
      test.todo("should not be able to get other users tasks");
      test.todo("should not be able to get deleted tasks");
    });

    describe("get one task", () => {
      test.todo("should be able to get my task");
      test.todo("should not be able to get task when logged out");
      test.todo("should not be able to get another users task");
    });

    describe("update task", () => {
      test.todo("should be able to mark a task as completed");
      test.todo("should be able to mark a test as not completed");
      test.todo("should be able to update all fields in the task");
      test.todo("should not be able to mark other users tasks as completed");
      test.todo("should not be able to update other users tasks");
    });

    describe("soft delete a task", () => {
      test.todo("should be able to soft delete a task");
      test.todo("should not be able to soft delete another users task");
    })
  });

  describe("Creating an account", () => {
    test.todo("new users should get default tasks");
  })
})

function checkLoggedInUser(userFromApi, testUser) {
  expect(typeof userFromApi.id).toBe("number");
  expect(userFromApi.username).toBe(testUser.username);
  expect(typeof userFromApi.token).toBe("string");
  const token = jwt.verify(userFromApi.token, JWT_SECRET);
  expect(token.username).toBe(testUser.username);
}

async function createUser() {
  const result = await axios.post(`${baseUrl}/users`, {username: `${Math.random()}`, password: "password"});
  expect(typeof result.data.data.token).toBe("string");
  const user = result.data.data;
  const headers = {"x-auth-token": user.token};
  return [user, headers];
}

async function createTask(headers, newTask) {
  return await axios.post(`${baseUrl}/tasks`, newTask, {headers});
}

function sleep(time = 1000) {
  return new Promise((resolve, reject) => {
    setTimeout(() => resolve(), time);
  });
}