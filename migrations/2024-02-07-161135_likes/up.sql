CREATE TABLE Likes (
    user_id VARCHAR(255) REFERENCES users(id),
    clip_id VARCHAR(255) REFERENCES clips(id),
    PRIMARY KEY (user_id, clip_id)
);