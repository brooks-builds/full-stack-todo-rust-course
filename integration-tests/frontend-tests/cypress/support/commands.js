// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --

const faker = require("faker");

// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
Cypress.Commands.add("createAccount", (username, password) => {
  cy.visit("/create-account")
    .dget("username")
    .type(username)
    .dget("password")
    .type(password)
    .dget("submit")
    .click();
});

Cypress.Commands.add("dget", (selector) => {
  cy.get(`[data-test="${selector}"]`);
});

Cypress.Commands.add("login", (username, password) => {
  cy.visit("/login")
    .dget("username")
    .type(username)
    .dget("password")
    .type(password)
    .dget("submit")
    .click();
});

Cypress.Commands.add(
  "createTask",
  ({
    title = faker.lorem.sentence(),
    description = faker.lorem.sentences(3),
    priority = "B",
  }) => {
    cy.dget("add-task")
      .click()
      .dget("title")
      .type(title)
      .dget("description")
      .type(description)
      .dget("priority")
      .select(priority)
      .dget("submit")
      .click();
  }
);
