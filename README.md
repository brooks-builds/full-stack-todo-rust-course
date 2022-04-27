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

| Video / Lesson Title                                | Recorded | Section                   | Description                                                                                                                           | Video URI                    |
| --------------------------------------------------- | -------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| Hype Trailer                                        | x        |                           |                                                                                                                                       | https://youtu.be/A0wabjmYRZ0 |
| Introduction to the Course                          | x        | Introduction              |                                                                                                                                       | https://youtu.be/w0R1fE3qU5c |
| Prequisites                                         |          | Introduction              | What you need to know or have to get the most out of this course                                                                      |                              |
| Join the community                                  | x        | Introduction              | We have a community where we can ask questions, and give/receive help                                                                 | https://youtu.be/Hp8_-YF_lEw |
| Setting up Rust                                     | x        | Setting Up                | We're installing rust and some nice Cargo packages                                                                                    | https://youtu.be/o0se4Erg_g0 |
| Hello World in Rust                                 | x        | First Steps               | Our first Rust program                                                                                                                | https://youtu.be/NQ5q7LgpzMQ |
| Hello World in Yew.rs                               | x        | First Steps               | Now that we have a working Rust program, let's create our first Yew.rs program                                                        | https://youtu.be/uFUpC8yj6TQ |
| A better Hello World architecture                   | x        | First Steps               | Having everything in main.rs isn't ideal, let's set up a basic architecture that will work with us                                    | https://youtu.be/k-gVqe3HgwY |
| Logging to the Console                              | x        | First Steps               | Logging to the console is one of the best ways to debug in browsers                                                                   | https://youtu.be/NFOBWILWb1o |
| HTML in Rust                                        | x        | html                      |                                                                                                                                       | https://youtu.be/NeClBLl6uYk |
| HTML Conditionals                                   | x        | html                      |                                                                                                                                       | https://youtu.be/sBXnesJGiHM |
| HTML Loops                                          | x        | html                      |                                                                                                                                       | https://youtu.be/VmZLpP0jUGc |
| Introduction to stylist                             | x        | styling                   | We will be using a crate called stylist to apply css to our components                                                                | https://youtu.be/IQS6tWQWeYY |
| Inline styles                                       | x        | styling                   | The first and simplest of adding styles is to directly style the html                                                                 | https://youtu.be/qj_mBAcaSaE |
| Separate CSS file                                   | x        | styling                   | We don't have to give up our editors helping with with css, we can use a separate css file!                                           | https://youtu.be/RosdwpfOZU4 |
| Passing properties into components                  | x        | properties                | Components that can't receive data are boring                                                                                         | https://youtu.be/ure8M6FO-PA |
| enum properties                                     | x        | properties                | Sometimes we want more advanced properties                                                                                            | https://youtu.be/Gv7N468f-WE |
| Callbacks                                           | x        | properties                | We use callbacks so that we can communicate up to parent components                                                                   | https://youtu.be/4yOf0_5r_wg |
| creating a form                                     | x        | handling events and hooks | We'll be creating a form so that we can explore handling DOM events                                                                   | https://youtu.be/Y37wK1lMZws |
| handling text field events                          | x        | handling events and hooks | Let's add a event handler to the text field                                                                                           | https://youtu.be/BRlt_hrHUfE |
| use state                                           | x        | handling events and hooks | React style hooks exist in Yew.rs! this maens we can set and use state in functional components!                                      | https://youtu.be/LtZWzP4XVxs |
| multiple states                                     | x        | handling events and hooks |                                                                                                                                       | https://youtu.be/0lspzbYC1Zw |
| one complicated state                               | x        | handling events and hooks |                                                                                                                                       | https://youtu.be/cdsHbA-mY5s |
| Simplifying Complicated State Updates               | x        | handling events and hooks |                                                                                                                                       | https://youtu.be/R3Bdpb9z43Y |
| handling form submit events                         | x        | handling events and hooks | It can be nice to handle form submit events rather than just a click listener on buttons                                              | https://youtu.be/2JNw-ftN6js |
| Use Context Hook                                    | x        | handling events and hooks | access context state without needing prop drill many levels down                                                                      | https://youtu.be/4XP8ZSgqqLI |
| Use Effect Hook                                     | x        | handling events and hooks | use effect allows us to hook into the event lifecycle and run code on first launch, every refresh, and shut down.                     | https://youtu.be/xdzmMiT6K5E |
| Setting up Yew Router                               | x        | router                    | We can set up a frontend router so that we don't have to touch the server when navigating pages                                       | https://youtu.be/ijogSYDzwiE |
| Linking to Pages                                    | x        | router                    |                                                                                                                                       | https://youtu.be/twl8rl_m2pE |
| Programatic Linking to Pages                        | x        | router                    |                                                                                                                                       | https://youtu.be/kEbfqmfbC_w |
| Struct Components                                   | x        | struct components         | Struct components are similar to Class-based components in React or Vue.js                                                            | https://youtu.be/eG6eKNh2YdI |
| styling struct componets                            | x        | struct components         | Styling is pretty much the same, the only real difference is where to store the stylesheet                                            | https://youtu.be/KC7HCxr6OCE |
| struct properties                                   | x        | struct components         | The props are stored in the context object                                                                                            | https://youtu.be/KC7HCxr6OCE |
| struct lifecycle methods                            | x        | struct components         | These methods are available on the impl of the component struct                                                                       | https://youtu.be/HKAM6p34aIA |
| struct messages                                     | x        | struct components         | Saving state in a struct component uses message passing                                                                               | https://youtu.be/skvKGK9wzsg |
| Setting up Yewdux                                   | x        | stores                    | Yewdux is very similar to Redux for state management                                                                                  | https://youtu.be/-aHUoxHShS8 |
| Using Yewdux to Store Form Data                     | x        | stores                    | We want to create a form and save it's data to the Yewdux store                                                                       | https://youtu.be/cfxiv37bgxQ |
| Yewdux Functional                                   | x        | stores                    |                                                                                                                                       | https://youtu.be/mn3tRwXr22g |
| Persistent State                                    | x        | stores                    | To handle staying logged in between page refreshes we can store our internal state, or at least the user information to local storage | https://youtu.be/rL7W_Jw5beo |
| Installing Docker                                   | x        | Setting Up                | A server and database has been created to help us focus on front end dev, we can use Docker to set that up easily                     | https://youtu.be/Lg0R5q0S7QY |
| Starting the Todo Server                            | x        | Setting Up                | A crash course on using Docker and Docker-Dompose                                                                                     | https://youtu.be/RyOAa7KIyd4 |
| HTTP GET Requests                                   | x        | http                      | Websites are boring unless we can get data from other locations                                                                       | https://youtu.be/P6p5Qzxva1s |
| HTTP post requests                                  | x        | http                      | We want to be able to create things in the database, this is usually done with a post request                                         | https://youtu.be/2uvZDZoR_1c |
| Other HTTP requests                                 | x        | http                      | Updating resources uses either put or patch                                                                                           | https://youtu.be/Fo8wRW4Cjtk |
| Dynamically Setting the API_URI                     | x        | http                      | We want to have one API URI for dev, and one for production                                                                           | https://youtu.be/p07ALGFIWjo |
| Introducing the Project                             | x        | Introducing the Project   | Getting the backend up and running and exploring the API                                                                              | https://youtu.be/Y_51NHzr4Pk |
| Project Solution Live Code - 1                      | x        | Live coding the project   |                                                                                                                                       | uploaded                     |
| Project Solution Live Code - 2                      | x        | Live coding the project   |                                                                                                                                       | uploaded                     |
| Project Solution Live Code - 3                      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/1J6U5m_ZB1w |
| Project Solution Live Code - 4                      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/w6q6anvdisc |
| Project Solution Live Code - 5                      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/RZQOnLMac24 |
| Project Solution Live Code - 6                      | x        | Live coding the project   |                                                                                                                                       | uploaded                     |
| Project Solution Live Code - 7                      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/bNT90-GRjPc |
| Project Solution Live Code - 8                      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/_du6EBaGh5s |
| Project Solution Live Code - 9                      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/bUn3Dvvi9Lk |
| Project Solution Live Code - 10                     | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/SHu2qSo0eEI |
| Edit Text Input: Solution Live Code - 11            | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/eXsfThJ3ubI |
| Editing Task Description: Solution Live Code - 12   | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/cx9yVyQ-DJo |
| Editing Task Priority: Solution Live Code - 13      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/rgo25StmDoA |
| Editing Task Completed: Solution Live Code - 14     | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/XquRqHv9qjo |
| Editing Task Save: Solution Live Code - 15          | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/UOyB7cylfMg |
| Fixing Text Inputs: Solution Live Code - 16         | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/qjDd6Gs-M_g |
| Fixing Textarea: Solution Live Code - 17            | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/dINYVnoMbI8 |
| Fixing Completed Checkbox: Solution Live Code - 18  | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/XNY8zg6S2F8 |
| Load Tasks on Refresh: Solution Live Code - 19      | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/HA_XO8z349I |
| Updating Tasks: Solution Live Code - 20             | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/zqXgsLHMkMg |
| Sucessfully Updating Tasks: Solution Live Code - 21 | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/F7QIS_dgTQc |
| Updating the Task in Yedux: Solution Live Code - 22 | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/CspsMzHyPJg |
| Deleting the Task: Solution Live Code - 23          | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/0SptFxxyJYM |
| Cancelling Editing: Solution Live Code - 24         | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/aQ3jWNX7gCg |
| Creating a New Task: Solution Live Code - 25        | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/2MfPTwHi27g |
| Displaying New Task: Solution Live Code 26          | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/otEpS0Pz8UQ |
| Completing the Task: Solution Live Code 27          | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/IbVyxRAvjT0 |
| Filter and Sort: Solution Live Code 28              | x        | Live coding the project   |                                                                                                                                       | https://youtu.be/Swtsb7xXFsE |
| Review of the project                               |          | Conclusion                | Now that we have built the project, let's review what we did                                                                          |                              |
| Next steps                                          |          | Conclusion                | Where do you go from here                                                                                                             |                              |
| Feedback                                            |          | Conclusion                | I'd love some feedback on the course so that the next one is better!                                                                  |                              |
| Deploying The App                                   |          | Addendum                  | Let's deploy the project to the internet!                                                                                             |                              |
