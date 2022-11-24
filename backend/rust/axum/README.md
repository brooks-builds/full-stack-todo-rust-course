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
    - [x] ***Open the documentation***
- [x] Routing
  - [x] ***Create a router to handle http methods***
    - [x] ***handling HTTP methods***
  - [x] ***Create an extractor to get a string from the body***
    - [x] ***Extracting string from body***
  - [x] ***Create an extractor to get JSON from the body***
    - [x] ***Receiving JSON in a Post***
  - [x] ***Create an extractor to get a path variable from a request***
    - [x] ***Handling path variables***
  - [x] ***Create an extractor to get query parameters***
    - [x] ***Handling query params***
  - [x] ***Create an extractor to get the headers***
    - [x] ***Extracting the User Agent Header***
    - [x] ***Extracting a Custom Header***
  - [x] ***Apply middleware to routes***
    - [x] ***Set CORS headers***
    - [x] ***Using layers to share data between routes***
    - [x] ***Creating middleware function***
      - Explore creating custom middleware again
      - During recording state that versions are very important
      - New version of Axum will change how middleware works
      - WARNING: Derive Clone on struct being added to Extensions
  - [x] ***Return the appropriate status code and message when erroring***
    - [x] ***Returning error status codes***
    - [x] ***Returning success status codes***
  - [x] ***Return JSON data***
    - [x] ***Respond with JSON data***
  - [x] ***Validate incoming data***
    - [x] ***Validating JSON with Serde***
    - [x] ***Custom Extractor with Validation***
- [ ] Data
  - [x] ***Creating a database***
    - [x] ***Set up a local database with Docker***
  - [x] ***Connect a database***
    - [x] ***Introducing SeaORM***
    - [x] ***Connecting to the database***
    - [x] ***SeaORM Models***
    - [x] ***Passing Database To Handlers***
  - [x] ***CRUD data in the database including soft deletions***
    - [x] ***Create a row in the database***
    - [x] ***Get one item from the database***
    - [x] ***Get all items from the database***
    - [x] ***Using filters***
    - [x] ***Atomic updates***
    - [x] ***Patch updates (maybe re-record using into_active_value?)***
    - [x] ***Deleting data***
    - [x] ***Soft-deleting data***
- [ ] Security
  - [ ] Authentication
    - [ ] How auth works
      - [x] ***creating account***
      - [x] *login*
      - [x] *guard route*
      - [x] *logout*
      - [ ] use middleware
    - [ ] Make it secure
      - [ ] Hashing the password
      - [ ] use a JWT
- [ ] Helper Utilities
  - [ ] Custom errors
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
