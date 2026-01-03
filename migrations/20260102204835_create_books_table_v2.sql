-- Add migration script here
-- Add migration script here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    isbn VARCHAR(13) UNIQUE,
    price DECIMAL(10, 2) NOT NULL,
    stock_quantity INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    published_date DATE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create an index on title for faster searches
CREATE INDEX idx_books_title ON books(title);

-- Create an index on author for faster searches
CREATE INDEX idx_books_author ON books(author);
