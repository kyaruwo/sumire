DROP DATABASE IF EXISTS sumire;
CREATE DATABASE sumire;
use sumire;
CREATE TABLE Notes (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(69),
    body text
);