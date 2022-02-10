const faker = require('faker');

describe("todo app", () => {
  describe("navigation", () => {
    it("can navigate home using the 'logo'", () => {
      cy
        .visit("/create-account")
        .get("[data-test-logo]")
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
        .visit("/")
        .get("[data-test-create-account]")
        .click()
        .url()
        .should("contain", "/create-account")
        .get("[data-test-username]")
        .type(username)
        .get("[data-test-password")
        .type(password)
        .get("[data-test-submit")
        .click()
        .get("[data-test-create-account]")
        .should("not.exist")
        .get("[data-test-login]")
        .should("not.exist")
        .get("[data-test-welcome]")
        .should("contain", `Welcome, ${username}`)
        .url()
        .should("not.contain", "/create-account")
    })
  });

  describe("logging into an account", () => {
    const username = faker.internet.userName();
    const password = faker.internet.password();

    before(() => {
      cy
        .visit("/create-account")
        .get("[data-test-username]")
        .type(username)
        .get("[data-test-password")
        .type(password)
        .get("[data-test-submit")
        .click()
    })

    it("should be able to log into an existing account", () => {
      cy
        .visit("/")
        .get("[data-test-login]")
        .click()
        .url()
        .should("contain", "/login")
        .get("[data-test-username]")
        .type(username)
        .get("[data-test-password]")
        .type(password)
        .get("[data-test-submit]")
        .click()
        .url()
        .should("not.contain", "/login")
        .get("[data-test-welcome]")
        .should("contain", `Welcome, ${username}`)
    })
  })

  describe("default todo items", () => {
    const username = faker.internet.userName()
    const password = faker.internet.password()

    before(() => {
      cy
        .visit("/create-account")
        .get("[data-test-username]")
        .type(username)
        .get("[data-test-password]")
        .type(password)
        .get("[data-test-submit]")
        .click()
    })

    it("should exist on newly created accounts", () => {
      cy
        .get("[data-test-task]")
        .should("have.length", 2)
        .get("[data-test-task]")
        .should("contain", "I am a task, you can complete me by checking the box")
        .get("[data-test-task]")
        .should("contain", "See my details for by clicking me")
    })
  })

  describe.only("task details", () => {
    it("should load the details for a single task", () => {
      const username = faker.internet.userName();
      const password = faker.internet.password();

      cy
        .visit("/create-account")
        .get("[data-test-username]")
        .type(username)
        .get("[data-test-password]")
        .type(password)
        .get("[data-test-submit]")
        .click()
        .get("[data-test-tasklink]")
        .first()
        .click()
        .url()
        .should("contain", "/tasks/")
        .get("[data-test-title]")
        .should("contain", "I am a task, you can complete me by checking the box")
        .get("[data-test=completed]")
        .should("not.be.checked")
        .get("[data-test-priority]")
        .should("contain", "A")
        .get("[data-test-description]")
        .should("contain", "This is my description")
    })
  })
})