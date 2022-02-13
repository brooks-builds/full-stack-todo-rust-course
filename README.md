# full-stack-todo-rust-course

wip - we are building this out now in prep for the real course

## Plan

- [x] Come up with the requirements
- [x] Create user stories
- [x] Design the todo app
- [x] Create the database schema
- [x] Create init.sql file
- [x] Dockerize database
- [x] Create the backend
- [x] Put a unique constraint on the username field in the db
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

- [x] As a doer, I want to create an account √
  - [x] Set base url as a environment variable
- [x] As a doer, I want to log in to my existing account √
- [x] As a doer logging in for the first time, I want my account to see default tasks √
- [x] As a logged in doer, I want to see a list of my tasks √
- [x] As a logged in doer, I want to click on a task and see it's details √
  - [x] break out the checkbox as its own component
- [x] As a logged in doer viewing task details, I want to edit the task name and description
- [x] As a logged in doer, I want to create a task
- [x] As a logged in doer, I want to mark my tasks as complete
- [ ] As a logged out doer, I don't want to be able to see my tasks
- [ ] As a logged in doer, I want to delete a task
- [ ] As a logged in doer, I want to be able to log out
- [ ] As a logged in doer, I want to assign a task a priority
- [ ] As a logged in doer, I want to sort my tasks by priority
- [ ] As a logged in doer, I want to filter my tasks by priority
- [ ] As a user, when an error occurs I want to see what happened
- [ ] As a logged in doer viewing task details, I want to complete a task
- [ ] As a logged in doer viewing task details, I want to uncomplete a task
- [ ] As a logged in doer editing a task, I want to cancel without saving
- [ ] As a logged in doer creating a task, I want to cancel without saving

## Clean up and Polish

- [ ] Come up with a font that doesn't suck
- [ ] Update colors to be nice and less eye bleeding
- [ ] Handle attempting to log in with a user that doesn't exist better (500 error right now)
- [ ] Handle if the user isn't logged in, but tries to see a protected page
- [ ] Keep the user logged in over page refreshes
- [ ] check other browsers

## Design

View the wireframe at [https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy](https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy)