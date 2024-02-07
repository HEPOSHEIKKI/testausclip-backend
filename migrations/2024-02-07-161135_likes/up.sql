CREATE TABLE Likes (
    userid VARCHAR(255) REFERENCES users(userid),
    clipid VARCHAR(255) REFERENCES clips(id),
    PRIMARY KEY (userid, clipid)
);