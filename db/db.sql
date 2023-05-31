-- Create the 'posts' table
CREATE TABLE posts (
    postId INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    createdAt TEXT NOT NULL,
    userId INTEGER,
    FOREIGN KEY (userId) REFERENCES users(userId)
);

-- Create the 'users' table
CREATE TABLE users (
    userId INTEGER PRIMARY KEY,
    firstName TEXT,
    lastName TEXT,
    userName TEXT,
    email TEXT UNIQUE NOT NULL,
    password TEXT
);

-- Create the 'sessions' table
CREATE TABLE sessions (
    sessionId INTEGER PRIMARY KEY,
    userId INTEGER NOT NULL,
    expiresIn TEXT,
    FOREIGN KEY (userId) REFERENCES users(userId)
);
