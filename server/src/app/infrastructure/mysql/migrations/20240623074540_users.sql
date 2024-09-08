CREATE TABLE users
(
  id CHAR(36) PRIMARY KEY,
  name VARCHAR(255),
  email VARCHAR(255),
  password VARCHAR(255),
  auth_provider VARCHAR(50) NOT NULL COMMENT '認証プロバイダー(例: google, apple, credential)',
  created_at TIMESTAMP NOT NULL
);