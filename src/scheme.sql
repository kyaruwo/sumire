DROP DATABASE IF EXISTS sumire;
CREATE DATABASE sumire;
use sumire;
CREATE TABLE Notes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(69),
    body text
);