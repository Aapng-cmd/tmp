-- Drop existing database and user if they exist
DROP DATABASE IF EXISTS note_database;
DROP USER IF EXISTS 'user1'@'%';

-- Create a new database
CREATE DATABASE note_database;

-- Use the newly created database
USE note_database;

-- Create users table
CREATE TABLE users (
  id INT AUTO_INCREMENT,
  username VARCHAR(255) UNIQUE,
  password VARCHAR(255),
  PRIMARY KEY (id)
);

-- Create notes table
CREATE TABLE notes (
  id INT AUTO_INCREMENT,
  user_id INT,
  content TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create user
CREATE USER 'user1'@'%' IDENTIFIED BY 'passWord';

-- Grant privileges to the user
GRANT ALL PRIVILEGES ON note_database.* TO 'user1'@'%';
