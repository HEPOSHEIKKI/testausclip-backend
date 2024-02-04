CREATE TABLE Posts (
    ID TEXT PRIMARY KEY,
    Title TEXT,
    Description TEXT,
    Private BOOL,
    OwnerID INT REFERENCES Users(UserID)
);
