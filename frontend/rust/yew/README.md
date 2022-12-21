# Introduction to Yew

## Video Lessons

- [ ] Updating to Yew 0.20
  - [ ] Reviewing the 0.20 changelog
  - [ ] Update the solution to 0.20
    - Set up docker to run express server and database
      - possibly run `docker system prune` to remove old docker networks in case of docker compose errors
    - Start frontend
    - Run Cypress tests
      - Possibly update Cypress
    - Add in Rust workspace
    - Update Cargo.toml
      - Yew needs `csr` feature
    - Run server
      - Maybe need to delete `Cargo.toml` file in case of bad versions
    - Fix problems
      - Errors
      - Stylist is putting debug string into classes, switch to style! macro instead of css! macro
