# Next

- [x] Move DateTime to be deserialized in the update request struct for update

# Look into

- [x] Are we testing user cannot uncheck another users task
- [x] All types of updates (patch, atomic) are guarded so that other users can't update any task they want

# Plan

**Legend**

- [ ] Unit
  - [ ] Standard
    - [ ] Lesson

- [ ] Hello world
  - [ ] Spin up a hello world server
    - [ ] Installing Rust
    - [ ] Using the course repo
    - [ ] Setting up VS Code
    - [ ] Create hello world Axum server
    - [ ] Manually testing the server with Thunderclient
- [ ] Routing
  - [ ] Create a router to handle http methods
    - [ ] handling HTTP methods
  - [ ] Create an extractor to get a string from the body
    - [ ] Receiving data in a Post
  - [ ] Create an extractor to get JSON from the body
    - [ ] Receiving JSON in a Post
  - [ ] Create an extractor to get a path variable from a request
    - [ ] Handling path variables
  - [ ] Create an extractor to get query parameters
    - [ ] Handling query params
  - [ ] Create an extractor to get the headers
    - [ ] Handling headers
  - [ ] Apply middleware to routes
    - [ ] Creating middleware function
    - [ ] Positioning middleware functions
  - [ ] Create a layer to share data between routes
    - [ ] Using layers to share data between routes
  - [ ] Return the appropriate status code and message when erroring
    - [ ] Returning status codes
  - [ ] Return JSON data
    - [ ] Respond with JSON data
- [ ] Data
  - [ ] Creating a database
    - [ ] Set up a local database with Docker
  - [ ] Connect a database
    - [ ] Install SeaORM
    - [ ] Connect SeaORM to the database
    - [ ] Generate the SeaORM data models
  - [ ] Validate incoming data
    - [ ] Using serde to deserialize data
    - [ ] Validate data with a Custom JSON extractor
  - [ ] CRUD data in the database including soft deletions
    - [ ] Create a row in the database
    - [ ] Get one item from the database
    - [ ] Get all items from the database
    - [ ] Using filters
    - [ ] Atomic updates
    - [ ] Patch updates
    - [ ] Deleting data
    - [ ] Soft-deleting data
    - [ ] Handling nulls
  - [ ] Work with time
    - [ ] What kind of time to use in the database
    - [ ] Storing current time in the database
    - [ ] Getting a time from the database
    - [ ] Responding with time
- [ ] Security
  - [ ] Load secrets with environment variables
    - [ ] Loading environment variables
    - [ ] dotenvy to load a .env file
  - [ ] Create a JWT
    - [ ] Create a JWT
    - [ ] Validate the JWT
  - [ ] Handle errors without crashing
    - [ ] Custom error response
    - [ ] converting errors
  - [ ] Hash passwords
    - [ ] Hash password
    - [ ] Validate hash
- [ ] Devops
  - [ ] Run the server in a Docker container
    - [ ] Use a docker container for production
    - [ ] Use a docker container for development
  - [ ] Deploy the server
    - [ ] Directly to a VPS
- [ ] Project
  - [ ] Create a server that passes all tests
    - [ ] Introduce the project
    - [ ] Project follow along
    - [ ] Spinning up the project
    - [ ] 
