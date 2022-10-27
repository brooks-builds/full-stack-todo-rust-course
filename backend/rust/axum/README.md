# Lessons

## Video Description

View the code for the Introduction to Axum course at https://github.com/brooks-builds/full-stack-todo-rust-course/tree/main/backend/rust/axum.

Axum is a backend API framework for Rust. It's written by the same team that makes Tokio.rs and therefore is 100% compatible with Tokio. It's meant to be easy to get started with but powerful enough to run your project on.

Find the playlist with more Axum videos at https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS.

You can join the community Discord server on the Brooks Builds website at https://brooksbuilds.com. If you have any questions about the course, Rust, or Axum please bring them to the course help channel.

This video was recorded live on Twitch at https://twitch.tv/brookzerker if you want to check out what Brooks is up to recently.

**Legend**

- [ ] Unit
  - [ ] Standard
    - [ ] Lesson

- [ ] Introduction
  - [ ] Marketing
    - [x] ***Hype Trailer***
    - [ ] Trailer
  - [ ] Introduction
    - [ ] Prerequisites
      - Introductory experience with Rust
      - Programming experience (Have created a todo API in any language)
      - Familiarity with JSON
    - [ ] Introduction to the course
    - [x] ***Why Rust / Why Axum?***
      - Strong type system
      - Tooling
        - compiler messages
        - rust analyzer
        - package manager
      - Fast development time (once you familiar with the framework / language)
      - Fun
- [ ] Hello world
  - [ ] Spin up a hello world server
    - [x] ***Installing Rust (cancelled)***
    - [ ] Using the course repo
    - [x] ***Setting up VS Code***
    - [x] ***Create hello world Axum server***
    - [x] ***Auto restart the server***
    - [x] Open the documentation
- [ ] Routing
  - [x] Create a router to handle http methods
    - [x] ***handling HTTP methods***
  - [x] Create an extractor to get a string from the body
    - [x] Extracting string from body
  - [x] Create an extractor to get JSON from the body
    - [x] Receiving JSON in a Post
  - [x] Create an extractor to get a path variable from a request
    - [x] Handling path variables
  - [x] Create an extractor to get query parameters
    - [x] Handling query params
  - [x] Create an extractor to get the headers
    - [x] Extracting the User Agent Header
    - [x] Extracting a Custom Header
  - [ ] Apply middleware to routes
    - [x] Set CORS headers
    - [ ] Using layers to share data between routes
    - [ ] Creating middleware function
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
    - [ ] Setting up the database
    - [ ] Running the tests
