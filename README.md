# full-stack-todo-rust-course

wip - we are building this out now in prep for the real course

## Plan

- [x] Come up with the requirements
- [x] Create user stories
- [x] Design the todo app
- [x] Create the database schema
- [x] Create init.sql file
- [x] Dockerize database
- [ ] Create the backend
- [ ] Put a unique constraint on the username field in the db
- [ ] Create list of components to make
- [ ] Create atomic components
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

## API Todo

- [x] create a user
- [x] login as a user
- [x] logout as a user
- [x] create middleware for authenticated routes (like /users/logout)
- [x] Create a task
- [x] Get all my tasks
- [x] Get one task
- [x] Update task
- [x] soft-delete task

## user stories

- [ ] As a doer, I want to create an account
  - Set base url as a environment variable
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

## Design

View the wireframe at [https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy](https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy)