
CREATE TABLE IF NOT EXISTS Users (
  UserId SERIAL PRIMARY KEY,
  Username VARCHAR(255) NOT NULL UNIQUE
);


CREATE TABLE IF NOT EXISTS Friends (
  FriendId SERIAL PRIMARY KEY,
  UserId1 INT NOT NULL, 
  UserId2 INT NOT NULL,
  FOREIGN KEY (UserId1) REFERENCES Users(UserId),
  FOREIGN KEY (UserId2) REFERENCES Users(UserId),
  CHECK (UserId1 <> UserId2)
);

CREATE TYPE RequestStatus AS ENUM ('Pending', 'Accepted');

CREATE TABLE IF NOT EXISTS FriendRequests (
  RequestId SERIAL PRIMARY KEY,
  SenderId INT NOT NULL,
  ReceiverId INT NOT NULL,
  Status RequestStatus DEFAULT 'Pending',
  FOREIGN KEY (SenderId) REFERENCES Users(UserId),
  FOREIGN KEY (ReceiverId) REFERENCES Users(UserId),
  CHECK (SenderId <> ReceiverId)
);
