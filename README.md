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
- [x] As a logged in doer, I want to mark my tasks as complete √
- [x] As a logged out doer, I don't want to be able to see my tasks √
- [x] As a logged in doer, I want to delete a task √
- [x] As a logged in doer, I want to assign a task a priority √
- [x] As a logged in doer, I want to sort my tasks by priority √
- [x] As a logged in doer, I want to sort my tasks by name √
- [x] As a logged in doer, I want to sort my tasks by created order √
- [x] As a logged in doer, I want to filter my tasks by none √
- [x] As a logged in doer, I want to filter my tasks by completed √
- [x] As a logged in doer, I want to filter my tasks by priority √
- [x] As a user, when an error occurs I want to see what happened √
- [x] As a logged in doer viewing task details, I want to complete a task √
- [x] As a logged in doer viewing task details, I want to uncomplete a task √
- [x] As a logged in doer editing a task, I want to cancel without saving √
- [x] As a logged in doer creating a task, I want to cancel without saving √

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
- [x] Write end to end tests with Cypress
- make the data-test attributes consistent

## Design

View the wireframe at [https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy](https://brookspatton508559.invisionapp.com/freehand/todo-app-course-ldCXDnSxy)

# Course Plan

## Yew

| Video / Lesson Title               | Recorded | Section                   | Description                                                                                                                           | Video URI                    |
| ---------------------------------- | -------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| Hype Trailer                       | x        |                           |                                                                                                                                       | https://youtu.be/A0wabjmYRZ0 |
| Introduction to the Course         | x        | Introduction              |                                                                                                                                       | https://youtu.be/w0R1fE3qU5c |
| Prequisites                        |          | Introduction              | What you need to know or have to get the most out of this course                                                                      |                              |
| Join the community                 | x        | Introduction              | We have a community where we can ask questions, and give/receive help                                                                 | https://youtu.be/Hp8_-YF_lEw |
| Setting up Rust                    | x        | Setting Up                | We're installing rust and some nice Cargo packages                                                                                    | https://youtu.be/o0se4Erg_g0 |
| Hello World in Rust                | x        | First Steps               | Our first Rust program                                                                                                                | https://youtu.be/NQ5q7LgpzMQ |
| Hello World in Yew.rs              | x        | First Steps               | Now that we have a working Rust program, let's create our first Yew.rs program                                                        | https://youtu.be/uFUpC8yj6TQ |
| A better Hello World architecture  | x        | First Steps               | Having everything in main.rs isn't ideal, let's set up a basic architecture that will work with us                                    | https://youtu.be/k-gVqe3HgwY |
| Logging to the Console             | x        | First Steps               | Logging to the console is one of the best ways to debug in browsers                                                                   | https://youtu.be/BkPoMEyC7EE |
| Introduction to stylist            | x        | styling                   | We will be using a crate called stylist to apply css to our components                                                                | https://youtu.be/IQS6tWQWeYY |
| Inline styles                      | x        | styling                   | The first and simplest of adding styles is to directly style the html                                                                 | https://youtu.be/qj_mBAcaSaE |
| Separate CSS file                  | x        | styling                   | We don't have to give up our editors helping with with css, we can use a separate css file!                                           | https://youtu.be/RosdwpfOZU4 |
| Passing properties into components | x        | properties                | Components that can't receive data are boring                                                                                         | https://youtu.be/ure8M6FO-PA |
| enum properties                    | x        | properties                | Sometimes we want more advanced properties                                                                                            | https://youtu.be/Gv7N468f-WE |
| Callbacks                          | x        | properties                | We use callbacks so that we can communicate up to parent components                                                                   | https://youtu.be/4yOf0_5r_wg |
| creating a form                    | x        | handling events and hooks | We'll be creating a form so that we can explore handling DOM events                                                                   | https://youtu.be/Y37wK1lMZws |
| handling text field events         | x        | handling events and hooks | Let's add a event handler to the text field                                                                                           | https://youtu.be/BRlt_hrHUfE |
| use state                          | x        | handling events and hooks | React style hooks exist in Yew.rs! this maens we can set and use state in functional components!                                      |                              |
| multiple states                    | x        | handling events and hooks |                                                                                                                                       |                              |
| one complicated state              | x        | handling events and hooks |                                                                                                                                       |                              |
| handling form submit events        | x        | handling events and hooks | It can be nice to handle form submit events rather than just a click listener on buttons                                              |                              |
| Use Context Hook                   | x        | handling events and hooks | access context state without needing prop drill many levels down                                                                      |                              |
| Setting up Yew Router              | x        | router                    | We can set up a frontend router so that we don't have to touch the server when navigating pages                                       |                              |
| Linking to Pages                   | x        | router                    |                                                                                                                                       |                              |
| Programatic Linking to Pages       | x        | router                    |                                                                                                                                       |                              |
| Struct Components                  | x        | struct components         | Struct components are similar to Class-based components in React or Vue.js                                                            |                              |
| styling struct componets           | x        | struct components         | Styling is pretty much the same, the only real difference is where to store the stylesheet                                            |                              |
| struct properties                  | x        | struct components         | The props are stored in the context object                                                                                            |                              |
| struct lifecycle methods           | x        | struct components         | These methods are available on the impl of the component struct                                                                       |                              |
| handling events                    |          | struct components         | Saving state in a struct component uses message passing                                                                               |                              |
| Introduction to centralized stores |          | stores                    | We have two stores we will be using this course, Bounce and Yewdux                                                                    |                              |
| Setting up Yewdux                  |          | stores                    | Yewdux is very similar to Redux for state management                                                                                  |                              |
| Using Yewdux to store form data    |          | stores                    | We want to create a form and save it's data to the Yewdux store                                                                       |                              |
| Setting up Bounce                  |          | stores                    | Bounce is the authors choice for functional component stores                                                                          |                              |
| Using bounce to store form data    |          | stores                    | We want to create a form and save it's data to the Bounce store                                                                       |                              |
| Saving state to local storage      |          | stores                    | To handle staying logged in between page refreshes we can store our internal state, or at least the user information to local storage |                              |
| Restoring local storage to state   |          | stores                    | When the page loads, lets load what we have in local storage                                                                          |                              |
| Installing Docker                  |          | Setting Up                | A server and database has been created to help us focus on front end dev, we can use Docker to set that up easily                     |                              |
| Intro to Docker                    |          | Setting Up                | A crash course on using Docker and Docker-Dompose                                                                                     |                              |
| HTTP get requests                  |          | http                      | Websites are boring unless we can get data from other locations                                                                       |                              |
| HTTP post requests                 |          | http                      | We want to be able to create things in the database, this is usually done with a post request                                         |                              |
| HTTP put requests                  |          | http                      | Updating resources uses either put or patch                                                                                           |                              |
| Setting up the Course Repo         |          | Introducing the Project   | Getting the backend up and running and exploring the API                                                                              |                              |
| Review the project                 |          | Introducing the Project   | Let's look through the Vue.js version of the todo app and see what we are going to build                                              |                              |
| Creating a plan                    |          | Introducing the Project   | Let's plan out the Yew.rs version of the todo app                                                                                     |                              |
| global css                         |          | styling                   | There are some css rules we want available for everything                                                                             |                              |
| Creating a link                    |          | Atom components           | links are a tags that look like normal links or buttons                                                                               |                              |
| Atomic component snippet           |          | Atom components           | We are going to create a lot of these components, let's create a snippet to help things be a bit faster                               |                              |
| Title                              |          | Atom components           |                                                                                                                                       |                              |
| form input                         |          | Atom components           | text and password type, labels                                                                                                        |                              |
| button                             |          | Atom components           | can be disabled, custom colors                                                                                                        |                              |
| select                             |          | Atom components           | select dropdown, labels                                                                                                               |                              |
| data table                         |          | Atom components           | table to organize data nicely                                                                                                         |                              |
| checkbox                           |          | Atom components           | Needs to be a real checkbox, but look nice                                                                                            |                              |
| text area                          |          | Atom components           | label                                                                                                                                 |                              |
| text                               |          | Atom components           | set the color                                                                                                                         |                              |
| icon                               |          | Atom components           | x, check                                                                                                                              |                              |
| logged-out buttons                 |          | Molecule components       | log in, create account                                                                                                                |                              |
| Molecle component snippet          |          | Molecule components       |                                                                                                                                       |                              |
| username/password form             |          | Molecule components       | username, password, button                                                                                                            |                              |
| filter/sort                        |          | Molecule components       | two selects                                                                                                                           |                              |
| edit buttons                       |          | Molecule components       | edit, delete                                                                                                                          |                              |
| edit task                          |          | Molecule components       | title, completed, priority, description, buttons                                                                                      |                              |
| navbar                             |          | Organism components       | link, logged out buttons, add task button, edit buttons                                                                               |                              |
| tasks                              |          | Organism components       | list all tasks data table                                                                                                             |                              |
| home                               |          | View components           |                                                                                                                                       |                              |
| create account                     |          | View components           |                                                                                                                                       |                              |
| login                              |          | View components           |                                                                                                                                       |                              |
| task                               |          | View components           |                                                                                                                                       |                              |
| edit task                          |          | View components           |                                                                                                                                       |                              |
| Review of the project              |          | Conclusion                | Now that we have built the project, let's review what we did                                                                          |                              |
| Next steps                         |          | Conclusion                | Where do you go from here                                                                                                             |                              |
| Feedback                           |          | Conclusion                | I'd love some feedback on the course so that the next one is better!                                                                  |                              |
