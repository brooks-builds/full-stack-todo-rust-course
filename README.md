# full-stack-todo-rust-course

wip - we are building this out now in prep for the real course

## Plan

- [x] Come up with the requirements
- [x] Create user stories
- [ ] Design the todo app
- [ ] Create the database schema
- [ ] Create init.sql file
- [ ] Dockerize database
- [ ] Create the backend
- [ ] Create the frontend
- [ ] Set up docker compose for all of the pieces

## Requirements

- CRUD tasks
  - Create the task
  - Update the task wording
  - mark task as complete
  - list all tasks
  - display one task with description
  - delete a task
- Basic auth (username + password)
- assign a task a priority
- filter by priority
- sort by priority

## user stories

- [ ] As a doer, I want to create an account
- [ ] As a doer, I want to log in to my existing account
- [ ] As a doer logging in for the first time, I want my account to see default tasks
- [ ] As a logged in doer, I want to see a list of my tasks
- [ ] As a logged in doer, I want to click on a task and see it's details
- [ ] As a logged in doer viewing task details, I want to edit the task name and description
- [ ] As a logged in doer, I want to create a task
- [ ] As a logged in doer, I want to mark my tasks as complete
- [ ] As a logged out doer, I don't want to be able to see my tasks
- [ ] As a logged in doer, I want to delete a task
- [ ] As a logged in doer, I want to be able to log out
- [ ] As a logged in doer, I want to assign a task a priority
- [ ] As a logged in doer, I want to sort my tasks by priority
- [ ] As a logged in doer, I want to filter my tasks by priority