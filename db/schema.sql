DROP DATABASE IF EXISTS sumire;
CREATE DATABASE sumire;
use sumire;
CREATE TABLE Notes (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(42) NOT NULL,
    body TEXT NOT NULL
);