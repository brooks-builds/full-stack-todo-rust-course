require("dotenv").config();
const { default: axios } = require("axios");
const jwt = require("jsonwebtoken");
const knex = require("knex");
const dbConfiguration = require("./knexfile");
const bcrypt = require("bcrypt");
const dns = require('node:dns');

const apiPort = process.env.API_PORT;
const baseUrl = `${process.env.API_URI}:${apiPort}/api/v1`;
const JWT_SECRET = process.env.JWT_SECRET;
const db = knex(dbConfiguration);

console.log(apiPort, baseUrl)

dns.setDefaultResultOrder('ipv4first');
jest.setTimeout(10000);

describe("todo api", () => {
  describe("User CRUD", () => {
    const userToCreate = {
      username: `testuser${Date.now()}`,
      password: `${Date.now()}`,
    };
    
    test("sign up", async () => {
      const { data: response } = await axios.post(
        `${baseUrl}/users`,
        userToCreate
        );
        expect(response).toHaveProperty("data");
        const { data } = response;
        checkLoggedInUser(data, userToCreate);
        const createdUser = await db
        .table("users")
        .where("username", userToCreate.username)
        .first();
        expect(createdUser.id).toBe(data.id);
        expect(createdUser.username).toBe(userToCreate.username);
        expect(createdUser.password).not.toBe(userToCreate.password);
        expect(createdUser.token).toBe(data.token);
        const passwordMatched = bcrypt.compareSync(
          userToCreate.password,
          createdUser.password
          );
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
        expect(error.response.data.error).toBe(
          "Username already taken, try again with a different user name"
        );
        gotError = true;
      }

      expect(gotError).toBe(true);
    });

    test("sign in", async () => {
      const loggedInUser = await axios.post(
        `${baseUrl}/users/login`,
        userToCreate
      );
      checkLoggedInUser(loggedInUser.data.data, userToCreate);
    });

    test("gets a good error message when attempting to log in with bad username and/or password", async () => {
      let gotError = false;
      try {
        await axios.post(`${baseUrl}/users/login`, {
          username: "329q845gljwenpdhrefpdhury",
          password: "q9384gfuenh",
        });
      } catch (error) {
        gotError = true;
        expect(error.response.status).toBe(400);
        expect(error.response.data.error).toBe(
          "incorrect username and/or password"
        );
      }

      expect(gotError).toBe(true);
    });

    test("log out", async () => {
      const userToCreate = {
        username: "userjusttologout" + Date.now(),
        password: "test1234",
      };
      const createdUser = await axios.post(`${baseUrl}/users`, userToCreate);
      await axios({
        method: "POST",
        url: `${baseUrl}/users/logout`,
        headers: {
          "x-auth-token": createdUser.data.data.token,
        },
      });
      const dbUser = await db
        .table("users")
        .where("username", userToCreate.username)
        .first();
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
        await axios.post(
          `${baseUrl}/users/logout`,
          {},
          { headers: { "x-auth-token": "fake token" } }
        );
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
          description: "This task is the best task",
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
          await axios.post(`${baseUrl}/tasks`, {}, { headers });
        } catch (error) {
          gotError = true;
          expect(error.response.data.error).toEqual("missing task title");
          expect(error.response.status).toBe(400);
        }

        expect(gotError).toBe(true);
      });
    });

    describe("get all tasks", () => {
      let getAllTasksUser;
      let getAllTasksHeaders;

      beforeAll(async () => {
        [getAllTasksUser, getAllTasksHeaders] = await createUser();

        await createTask(getAllTasksHeaders, { title: "task 1" });
        await createTask(getAllTasksHeaders, { title: "task 2" });
        await createTask(getAllTasksHeaders, { title: "task 3" });
      });

      test("should be able to get all my tasks", async () => {
        const createdTasks = await axios.get(`${baseUrl}/tasks`, {
          headers: getAllTasksHeaders,
        });
        expect(createdTasks.data.data.length).toBe(5);
        expect(createdTasks.data.data[0]).not.toHaveProperty("deleted_at");
        expect(createdTasks.data.data[0]).not.toHaveProperty("is_default");
        expect(createdTasks.data.data[0]).not.toHaveProperty("user_id");
      });

      test("should not be able to get any tasks when logged out", async () => {
        let gotError = false;

        try {
          await axios.get(`${baseUrl}/tasks`);
        } catch (error) {
          gotError = true;
          expect(error.response.status).toBe(401);
          expect(error.response.data.error).toBe("not authenticated!");
        }
      });

      test("should not be able to get other users tasks", async () => {
        const [user, headers] = await createUser();

        await createTask(headers, { title: "my task 10" });
        await createTask(headers, { title: "my task 20" });
        await createTask(headers, { title: "my task 30" });

        const response = await axios.get(`${baseUrl}/tasks`, { headers });

        expect(response.status).toBe(200);
        expect(response.data.data.length).toBe(5);
      });

      test("should not be able to get deleted tasks", async () => {
        const deletedUser = await axios.post(`${baseUrl}/users/login`, {
          username: "deleteduser",
          password: "password",
        });
        const headers = {
          "x-auth-token": deletedUser.data.data.token,
        };
        const response = await axios.get(`${baseUrl}/tasks`, { headers });

        expect(response.data.data.length).toBe(0);
      });
    });

    describe("get one task", () => {
      let users;
      let headers;
      let task;
      beforeAll(async () => {
        [users, headers] = await createUser();
        const firstTaskResponse = await createTask(headers, {
          title: "my one task 10",
        });
        task = firstTaskResponse.data.data;
        await createTask(headers, { title: "my one task 20" });
      });

      test("should be able to get my task", async () => {
        const result = await axios.get(`${baseUrl}/tasks/${task.id}`, {
          headers,
        });
        expect(result.data.data.id).toBe(task.id);

        expect(result.data.data.title).toBe("my one task 10");
      });

      test("should not be able to get task when logged out", async () => {
        let gotError = false;
        try {
          await axios.get(`${baseUrl}/tasks/${task.id}`);
        } catch (error) {
          gotError = true;
          expect(error.response.status).toBe(401);
          expect(error.response.data.error).toBe("not authenticated!");
        }

        expect(gotError).toBe(true);
      });

      test("should not be able to get another users task", async () => {
        const [newUser, newHeaders] = await createUser();
        let gotError = false;
        try {
          await axios.get(`${baseUrl}/tasks/${task.id}`, {
            headers: newHeaders,
          });
        } catch (error) {
          gotError = true;
          expect(error.response.status).toBe(404);
          expect(error.response.data.error).toBe("not found");
        }

        expect(gotError).toBe(true);
      });
    });

    describe("update task", () => {
      test("should be able to mark a task as completed", async () => {
        const [user, headers] = await createUser();
        const taskResponse = await createTask(headers, { title: "new task" });
        const task = taskResponse.data.data;
        expect(task.completed_at).toBe(null);
        let completedUri = `${baseUrl}/tasks/${task.id}/completed`;
        await axios.put(completedUri, {}, { headers });
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: task.id })
          .first();
        expect(dbTask.completed_at).not.toBe(null);
      });

      test("should be able to mark a task as not completed", async () => {
        const [user, headers] = await createUser();
        const taskResponse = await createTask(headers, { title: "new task" });
        const task = taskResponse.data.data;
        expect(task.completed_at).toBe(null);
        let completedUri = `${baseUrl}/tasks/${task.id}/completed`;
        await axios.put(completedUri, {}, { headers });

        let uncompletedUri = `${baseUrl}/tasks/${task.id}/uncompleted`;
        await axios.put(uncompletedUri, {}, { headers });
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: task.id })
          .first();
        expect(dbTask.completed_at).toBe(null);
      });

      test("should be able to update all fields in the task", async () => {
        const [user, headers] = await createUser();
        const initialTask = {
          priority: "A",
          title: "Stream",
          description: "Stream something on Twitch",
        };
        const createdTaskResponse = await axios.post(
          `${baseUrl}/tasks`,
          initialTask,
          { headers }
        );
        const createdTask = createdTaskResponse.data.data;
        const now = new Date();
        const updateTask = {
          priority: "B",
          title: "Create YouTube Video",
          description: "Meant to create a YouTube video instead",
          completed_at: now.toISOString(),
        };
        await axios.patch(`${baseUrl}/tasks/${createdTask.id}`, updateTask, {
          headers,
        });
        const dbTask = await db
        .select()
        .from("tasks")
        .where({ id: createdTask.id })
        .first();
        expect(dbTask.priority).toBe(updateTask.priority);
        expect(dbTask.title).toBe(updateTask.title);
        expect(dbTask.description).toBe(updateTask.description);
        let completed_at = new Date(dbTask.completed_at);
        expect(completed_at.toUTCString()).toBe(now.toUTCString());
      });

      test("can_update_some_of_the_task_without_losing_data", async () => {
        const [user, headers] = await createUser();
        const initialTask = {
          priority: "A",
          title: "Stream",
          description: "Stream something on Twitch",
        };
        const createdTaskResponse = await axios.post(
          `${baseUrl}/tasks`,
          initialTask,
          { headers }
          );
          const createdTask = createdTaskResponse.data.data;
          let completedUri = `${baseUrl}/tasks/${createdTask.id}/completed`;
          await axios.put(completedUri, {}, { headers });
        const updateTask = {
          description: "Meant to create a LinkedIn video instead",
        };
        await axios.patch(`${baseUrl}/tasks/${createdTask.id}`, updateTask, {
          headers,
        });
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: createdTask.id })
          .first();
        expect(dbTask.priority).toBe(initialTask.priority);
        expect(dbTask.title).toBe(initialTask.title);
        expect(dbTask.description).toBe(updateTask.description);
        expect(dbTask.completed_at).not.toBe(null);
      });

      test("can uncomplete a task with an update", async () => {
        const [user, headers] = await createUser();
        const initialTask = {
          priority: "A",
          title: "Stream",
          description: "Stream something on Twitch",
        };
        const createdTaskResponse = await axios.post(
          `${baseUrl}/tasks`,
          initialTask,
          { headers }
        );
        const createdTask = createdTaskResponse.data.data;
        let completedUri = `${baseUrl}/tasks/${createdTask.id}/completed`;
          await axios.put(completedUri, {}, { headers });
        const updateTask = {
          description: "Meant to create a LinkedIn video instead",
          completed_at: null,
        };
        await axios.patch(`${baseUrl}/tasks/${createdTask.id}`, updateTask, {
          headers,
        });
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: createdTask.id })
          .first();
        expect(dbTask.priority).toBe(initialTask.priority);
        expect(dbTask.title).toBe(initialTask.title);
        expect(dbTask.description).toBe(updateTask.description);
        expect(dbTask.completed_at).toBe(null);
      });

      test("should not be able to mark other users tasks as completed", async () => {
        const [user1, headers1] = await createUser();
        const [user2, headers2] = await createUser();
        const createdTaskResponse = await createTask(headers1, {
          title: "user 1 task",
        });
        let gotError = false;
        try {
           await axios.put(
            `${baseUrl}/tasks/${createdTaskResponse.data.data.id}/completed`,
            {},
            { headers: headers2 }
            );
          } catch(error) {
            expect(error.response.status).toBe(404);
            gotError = true;
        }
        expect(gotError).toBe(true);
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: createdTaskResponse.data.data.id })
          .first();
        expect(dbTask.completed_at).toBe(null);
      });

      test("should not be able to mark other users tasks as not completed", async () => {
        const [user1, headers1] = await createUser();
        const [user2, headers2] = await createUser();
        const createdTaskResponse = await createTask(headers1, {
          title: "user 1 task",
        });
        let gotError = false;
        try {
           await axios.put(
            `${baseUrl}/tasks/${createdTaskResponse.data.data.id}/completed`,
            {},
            { headers: headers1 }
            );

            await axios.put(
            `${baseUrl}/tasks/${createdTaskResponse.data.data.id}/uncompleted`,
            {},
            { headers: headers2 }
            );
          } catch(error) {
            expect(error.response.status).toBe(404);
            gotError = true;
        }
        expect(gotError).toBe(true);
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: createdTaskResponse.data.data.id })
          .first();
        expect(dbTask.completed_at).not.toBe(null);
      });

      test("should not be able to update other users tasks", async () => {
        const [user1, headers1] = await createUser();
        const [user2, headers2] = await createUser();
        const createdTaskResponse = await createTask(headers1, {
          title: "user 1 task",
        });
        let gotError = false;
        try {
          await axios.patch(
            `${baseUrl}/tasks/${createdTaskResponse.data.data.id}`,
            { title: "user 2 task" },
            { headers: headers2 }
          );
        } catch(error) {
          expect(error.response.status).toBe(404);
          gotError = true;
        }
        expect(gotError).toBe(true);
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: createdTaskResponse.data.data.id })
          .first();
        expect(dbTask.title).toBe("user 1 task");
      });
    });

    describe("soft delete a task", () => {
      test("should be able to soft delete a task", async () => {
        const [user, headers] = await createUser();
        const newTaskResponse = await createTask(headers, {
          title: "am I deleted?",
        });
        await axios.delete(`${baseUrl}/tasks/${newTaskResponse.data.data.id}`, {
          headers,
        });
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: newTaskResponse.data.data.id })
          .first();
        expect(dbTask.deleted_at).not.toBe(null);
      });

      test("should not be able to soft delete another users task", async () => {
        const [user, headers] = await createUser();
        const [user2, headers2] = await createUser();
        const newTaskResponse = await createTask(headers, {
          title: "am I deleted?",
        });
        let gotError = false;
        try {
          await axios.delete(`${baseUrl}/tasks/${newTaskResponse.data.data.id}`, {
            headers: headers2,
          });
        } catch (error) {
          gotError = true;
          expect(error.response.status).toBe(404);
        }
        expect(gotError).toBe(true);
        const dbTask = await db
          .select()
          .from("tasks")
          .where({ id: newTaskResponse.data.data.id })
          .first();
        expect(dbTask.deleted_at).toBe(null);
      });
    });
  });

  describe("Creating an account", () => {
    test("new users should get default tasks", async () => {
      const [user, headers] = await createUser();
      const tasksResponse = await axios.get(`${baseUrl}/tasks`, { headers });
      const tasks = tasksResponse.data.data;
      expect(tasks.length).toBe(2);
      const aTask = tasks.find((task) => task.priority == "A");
      expect(aTask.title).toBe(
        "I am a task, you can complete me by checking the box"
      );
      const bTask = tasks.find((task) => task.priority == "B");
      expect(bTask.title).toBe("See my details for by clicking me");
    });
  });

  afterAll(async () => {
    await db.destroy();
  });
});

function checkLoggedInUser(userFromApi, testUser) {
  expect(typeof userFromApi.id).toBe("number");
  expect(userFromApi.username).toBe(testUser.username);
  expect(typeof userFromApi.token).toBe("string");
  const token = jwt.verify(userFromApi.token, JWT_SECRET);
  expect(token.username).toBe(testUser.username);
  expect(userFromApi).not.toHaveProperty("deleted_at");
  expect(userFromApi).not.toHaveProperty("password");
}

async function createUser(username = Math.random()) {
  const result = await axios.post(`${baseUrl}/users`, {
    username: `${username}`,
    password: "password",
  });
  expect(typeof result.data.data.token).toBe("string");
  const user = result.data.data;
  const headers = { "x-auth-token": user.token };
  return [user, headers];
}

async function createTask(headers, newTask) {
  return await axios.post(`${baseUrl}/tasks`, newTask, { headers });
}

function sleep(time = 1000) {
  return new Promise((resolve, reject) => {
    setTimeout(() => resolve(), time);
  });
}
