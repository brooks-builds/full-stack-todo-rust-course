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
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
Cypress.Commands.add("createAccount", (username, password) => {
    cy.visit('/create-account')
        .get("[data-test-username]")
        .type(username)
        .get("[data-test-password]")
        .type(password)
        .get("[data-test-submit]")
        .click()
})

Cypress.Commands.add("dataGet", (selector) => {
    cy
        .get(`[data-test-${selector}]`)
})

Cypress.Commands.add("login", (username, password) => {
    cy
        .visit("/login")
        .dataGet("username")
        .type(username)
        .dataGet("password")
        .type(password)
        .dataGet("submit")
        .click()
})