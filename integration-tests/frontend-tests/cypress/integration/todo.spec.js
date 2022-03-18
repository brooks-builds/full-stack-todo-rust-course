const faker = require('faker');

describe("todo app", () => {
  describe("navigation", () => {
    it("can navigate home using the 'logo'", () => {
      cy
        .visit("/create-account")
        .dget("logo")
        .click()
        .url()
        .should("eq", "http://localhost:8080/");
    })
  })

  describe("creating an account", () => {
    it("should be able to create an account", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .dget("create-account")
        .should("not.exist")
        .dget("login")
        .should("not.exist")
        .dget("welcome")
        .should("contain", `Welcome, ${username}`)
        .url()
        .should("not.contain", "/create-account")
    })
  });

  describe("logging into an account", () => {
    it("should be able to log into an existing account", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .visit("/")
        .dget("login")
        .click()
        .url()
        .should("contain", "/login")
        .dget("username")
        .type(username)
        .dget("password")
        .type(password)
        .dget("submit")
        .click()
        .url()
        .should("not.contain", "/login")
        .dget("welcome")
        .should("contain", `Welcome, ${username}`)
    })
  })

  describe("default todo items", () => {
    it("should exist on newly created accounts", () => {
      const username = faker.internet.userName()
      const password = faker.internet.password()

      cy
        .createAccount(username, password)
        .dget("tasklink")
        .should("have.length", 2)
        .dget("tasklink")
        .should("contain", "I am a task, you can complete me by checking the box")
        .dget("tasklink")
        .should("contain", "See my details for by clicking me")
    })
  })

  describe("task details", () => {
    it("should load the details for a single task", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .dget("tasklink")
        .first()
        .click()
        .url()
        .should("contain", "/tasks/")
        .dget("title")
        .should("contain", "I am a task, you can complete me by checking the box")
        .dget("completed")
        .should("contain", "X")
        .dget("priority")
        .should("contain", "A")
        .dget("description")
        .should("contain", "This is my description")
    })

    it("should be editable", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .dget("tasklink")
        .first()
        .click()
        .dget("edit")
        .click()
        .dget("editing-title")
        .type("!!!")
        .dget("editing-description")
        .type("!!!")
        .dget("editing-priority")
        .select("B")
        .dget("completed")
        .click({force: true})
        .dget("submit")
        .click()
        .dget("editing-title")
        .should("not.be", "visible")
        .dget("title")
        .should("contain", "I am a task, you can complete me by checking the box!!!")
        .dget("description")
        .should("contain", "This is my description!!!")
        .visit("/login")
        .dget("username")
        .type(username)
        .dget("password")
        .type(password)
        .dget("submit")
        .click()
        .dget("tasklink")
        .first()
        .click()
        .dget("title")
        .should("contain", "I am a task, you can complete me by checking the box!!!")
        .dget("description")
        .should("contain", "This is my description!!!")
        .dget("priority")
        .should("contain", "B")
        .dget('completed')
        .should("contain", "✓")
        .dget('edit')
        .click()
        .dget("completed")
        .click({force: true})
        .dget("submit")
        .click()
        .login(username, password)
        .dget("tasklink")
        .first()
        .click()
        .dget('completed')
        .should("contain", "X")
    })

    it("should be deletable", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy.createAccount(username, password)
        .dget("tasklink")
        .should("have.length", 2)
        .dget("tasklink")
        .first()
        .click()
        .dget("delete")
        .click()
        .dget("tasklink")
        .should("have.length", 1)
    })

    it("should be able to cancel editing without saving", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .dget("tasklink")
        .first()
        .click()
        .dget("edit")
        .click()
        .dget("editing-title")
        .type("!!!")
        .dget("cancel")
        .click()
        .dget("title")
        .should("not.contain", "!!!")
    })
  })

  describe("creating a task", () => {
    it("should be able to create a new task", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();
      const title = faker.lorem.sentence();
      const description = faker.lorem.sentences(3);
      const priority = 'B'

      cy
        .createAccount(username, password)
        .dget("add-task")
        .click()
        .dget("title")
        .type(title)
        .dget("description")
        .type(description)
        .dget("priority")
        .select(priority)
        .dget("submit")
        .click()
        .dget("tasklink")
        .last()
        .should("contain", title)
        .dget("priority")
        .last()
        .should("contain", priority)
        .dget("tasklink")
        .last()
        .click()
        .dget("title")
        .should("contain", title)
        .dget("priority")
        .should("contain", priority)
        .dget('completed')
        .should("not.be.checked")
        .dget("description")
        .should("contain", description)
    })

    it("should be able to cancel while creating a task", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .dget("add-task")
        .click()
        .dget("title")
        .type("ZZZZZZ")
        .dget("cancel")
        .click()
        .dget("tasklink")
        .should("not.contain", "ZZZZZZ")
    })
  })

  describe("marking task complete", () => {
    it("can mark the task as complete", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();
      cy
        .server()
        .route("/api/v1/tasks").as("getTasks")
        .createAccount(username, password)
        .wait("@getTasks")
        .dget("completed")
        .first()
        .should("not.be.checked")
        .dget("completed")
        .first()
        .click({ force: true })
        .login(username, password)
        .dget("completed")
        .first()
        .should("be.checked")
    })
  })

  describe("logged out", () => {
    it("I should not be able to see any tasks", () => {
      cy
        .visit("/")
        .dget("tasklink")
        .should("have.length", 0)
    })
  })

  describe("home page", () => {
    it("should allow the user to sort the tasks", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .server()
        .route("post", "http://localhost:3000/api/v1/users").as("createAccount")
        .createAccount(username, password)
        .wait("@createAccount")
        .createTask({priority: "A", title: "ZZZZZZZZZZZZZZ"})
        .dget("priority")
        .eq(1)
        .should("contain", "B")
        .dget("sort")
        .select("priority")
        .dget("priority")
        .eq(1)
        .should("contain", "A")
        .dget("sort")
        .select("Name")
        .dget("tasklink")
        .eq(2)
        .should("contain", "See my details for by clicking me")
        .dget("sort")
        .select("priority")
        .dget("sort")
        .select("Created Order")
        .dget("tasklink")
        .eq(1)
        .should("contain", "ZZZZZZZZZZZZZZ")
    })

    it("should allow users to filter the tasks", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .createAccount(username, password)
        .dget("completed")
        .first()
        .click({force: true})
        .dget("filter")
        .select("Completed")
        .dget("tasklink")
        .should("have.length", 1)
        .dget("tasklink")
        .should("contain", "I am a task, you can complete me by checking the box")
        .dget("filter")
        .select("Uncompleted")
        .dget("tasklink")
        .should("have.length", 1)
        .dget("tasklink")
        .should("contain", "See my details for by clicking me")
        .dget("filter")
        .select("Priority A")
        .dget("tasklink")
        .should("have.length", 1)
        .dget("tasklink")
        .should("contain", "I am a task, you can complete me by checking the box")
        .dget("filter")
        .select("Priority B")
        .dget("tasklink")
        .should("have.length", 1)
        .dget("tasklink")
        .should("contain", "See my details for by clicking me")
        .dget("filter")
        .select("Priority C")
        .dget("tasklink")
        .should("have.length", 0)
        .dget("filter")
        .select("None")
        .dget("tasklink")
        .should("have.length", 2)
    })
  })

  describe("error messages", () => {
    it.only("should display when I navigate to a single task while logged out", () => {
      cy
        .visit("/tasks/1")
        .dget("error")
        .should("be.visible")
        .and("contain", "You must be logged in to view tasks")
        .wait(31000)
        .dget("error")
        .should("not.be.visible");
    })
  })
})