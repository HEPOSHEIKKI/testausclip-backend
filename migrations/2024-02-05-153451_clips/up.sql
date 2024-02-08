CREATE TABLE Clips (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(100),
    description TEXT,
    private BOOL DEFAULT FALSE,
    owner_id VARCHAR(255) REFERENCES Users(id),
    game TEXT,
    upload_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    file_name VARCHAR(255),
    views INT DEFAULT 0
)
