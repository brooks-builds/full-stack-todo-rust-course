# Possible Problems

## installing Diesel CLI

So it looks like ln -s /usr/local/Cellar/libpq/10.3/bin/psql /usr/local/bin/psql should work and this would only be a problem with macos users

# Next up

- [ ] Use state to pass the connection around


VIPPrime GamingStacking: Also I found in the docs a better way to pass the state in is Router::with_state instead of Router::new
