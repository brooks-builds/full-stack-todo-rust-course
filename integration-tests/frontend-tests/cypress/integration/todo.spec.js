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
    it.only("should be able to create an account", () => {
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
    })
  })
})