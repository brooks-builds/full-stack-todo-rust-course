CREATE TABLE IF NOT EXISTS users (
  id        SERIAL PRIMARY KEY,
  username  VARCHAR(64) NOT NULL UNIQUE,
  password  VARCHAR(64) NOT NULL,
  deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
  id            SERIAL PRIMARY KEY,
  priority      VARCHAR(4) DEFAULT NULL,
  completed     BOOLEAN DEFAULT FALSE,
  title         VARCHAR(255) NOT NULL,
  completed_at  TIMESTAMP DEFAULT NULL,
  description   TEXT,
  deleted_at    TIMESTAMP DEFAULT NULL,
  user_id       INTEGER NOT NULL, 
  is_default    BOOLEAN DEFAULT FALSE,
  CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
);