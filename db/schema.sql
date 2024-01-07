DROP DATABASE IF EXISTS sumire;

CREATE DATABASE sumire;

use sumire;

CREATE TABLE Users (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `email` VARBINARY(57) UNIQUE NOT NULL,
    `name` VARBINARY(32) UNIQUE NOT NULL,
    `password` BLOB NOT NULL,
    `token` VARBINARY(432) UNIQUE,
    `code` VARBINARY(20) UNIQUE,
    `code_retry` BIGINT UNSIGNED DEFAULT 0,
    `code_limit` BIGINT UNSIGNED DEFAULT 0,
    `verified` BOOLEAN DEFAULT 0
);

CREATE EVENT `RESET_code_limit` ON SCHEDULE EVERY 1 DAY STARTS '2024-01-05 00:00:00' DO
UPDATE
    Users
SET
    `code_limit` = 0;

CREATE TABLE UsersLogs (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `request` TEXT NOT NULL,
    `response` TEXT NOT NULL,
    `datetime` DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Notes (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `title` VARBINARY(54) NOT NULL,
    `body` VARBINARY(432) NOT NULL
);

CREATE TABLE NotesLogs (
    id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `request` TEXT NOT NULL,
    `note_id` BIGINT UNSIGNED NOT NULL,
    `response` TEXT NOT NULL,
    `datetime` DATETIME DEFAULT CURRENT_TIMESTAMP
);