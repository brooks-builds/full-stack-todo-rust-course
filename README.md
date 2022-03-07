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
- [x] Create the frontend
- [x] Set up docker compose for all of the pieces

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
- [x] As a logged in doer viewing task details, I want to edit the task name and description √
- [x] As a logged in doer, I want to create a task √
- [x] As a logged in doer, I want to mark my tasks as complete
- [x] As a logged out doer, I don't want to be able to see my tasks
- [x] As a logged in doer, I want to delete a task
- [x] As a logged in doer, I want to assign a task a priority
- [x] As a logged in doer, I want to sort my tasks by priority
- [x] As a logged in doer, I want to sort my tasks by name
- [x] As a logged in doer, I want to sort my tasks by created order
- [x] As a logged in doer, I want to filter my tasks by none
- [x] As a logged in doer, I want to filter my tasks by completed
- [x] As a logged in doer, I want to filter my tasks by priority
- [x] As a user, when an error occurs I want to see what happened
- [x] As a logged in doer viewing task details, I want to complete a task
- [x] As a logged in doer viewing task details, I want to uncomplete a task
- [x] As a logged in doer editing a task, I want to cancel without saving
- [x] As a logged in doer creating a task, I want to cancel without saving

## Clean up and Polish

- [x] Come up with a font that doesn't suck
- [x] Handle attempting to log in with a user that doesn't exist better (500 error right now)
- [x] Handle if the user isn't logged in, but tries to see a protected page
- [x] Keep the user logged in over page refreshes
- [ ] check other browsers
  - [x] Edge
    - [x] Sorting tasks not working
- [x] Error bar should not flash unless there is a real error
- [x] Decide if we should refactor to smaller components (we are going to save that for a potential course)
- [x] turn list of tasks into a data-table
- [x] Bugs
  - [x] New tasks are created completed if created after editing a task and clicking complete but not saving
  - [x] Cannot uncomplete tasks
- [x] Fix styling by switching to pixels but also breakpoints
  - [x] Desktop (1920 x 1080)
  - [x] Tablet landscape (1080 x 810)
  - [x] Phone landscape (812 x 375)
  - [x] Phone portrait (375 x 812)
- [ ] Write end to end tests with Cypress

## Design

View the wireframe at [https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy](https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy)

# Course Plan

## Yew

- Installing and setting up Yew
- Getting the counter example app running
- Set up a Hello World app
- Hello World component
  - Styling the component
    - inline
    - css in rust
    - separate css file `const STYLE: &'static str = include_str!("style.css");`
- Global CSS Setup
  - reset
  - dark background color
- Bring in the router
  - Note that the links must be in the router, so router will be top level
- State management
  - Yewdux
    - struct based components only if you want to access the state
  - Bounce
    - either
      - small stores based on parts of data to store
      - single state that holds everything, but everything is replaced when a change happens
- Create view placeholders
- How components talk to each other
  - Properties
  - Callbacks
- Create components?
  - [ ] atom
    - [x] link
      - [x] normal link
      - [x] button
    - [x] input form field
    - [ ] button
    - [ ] text
      - [ ] normal
      - [ ] title
    - [ ] select form field
    - [ ] checkbox form field
    - [ ] text field form field
    - [ ] icon
      - [ ] checkmark
      - [ ] x
  - [ ] molecule
    - [ ] edit task buttons
    - [ ] navbar right
    - [ ] select form field with label
    - [ ] data table
    - [ ] checkbox with label
  - [ ] organism
    - [ ] navbar
  - [ ] views
    - [ ] home (tasks)
    - [ ] add task
    - [ ] create account
    - [ ] log in
    - [ ] single task
    - [ ] edit task
- Create the top navbar
- Create a snippet for creating a component
  - atomic
- Sending http requests