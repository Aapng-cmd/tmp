CREATE TABLE IF NOT EXISTS users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(32) unique,
    password VARCHAR(64),
    preferences VARCHAR(64),
    current_balance INTEGER DEFAULT 100,
    priveleged BOOLEAN
);

CREATE TABLE IF NOT EXISTS basket(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(32),
    owner VARCHAR(32),
    cost INTEGER,
    description VARCHAR(64),
    sent_to VARCHAR(32),
    FOREIGN KEY (owner)
        REFERENCES users(username)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS beers(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(32),
    color CHAR,
    cost INTEGER
);

CREATE TABLE IF NOT EXISTS beers_basket(
    basket_id INTEGER,
    beer_id INTEGER,
    FOREIGN KEY (basket_id) 
        REFERENCES basket(id)
        ON DELETE CASCADE,
     FOREIGN KEY (beer_id) 
        REFERENCES beers(id)
        ON DELETE CASCADE
);

INSERT OR REPLACE INTO beers VALUES (1, "Lager", "G", 100), (2, "Pilsner", "L", 100), (3, "Ale", "A", 100), (4, "Stout", "B", 100), (5, "Porter", "D", 100), (6, "IPA", "O", 100), (7, "Sour", "P", 100), (8, "Saison", "G", 100), (9, "Barleywine", "R", 100), (10, "Dubbel", "M", 100), (11, "Bock", "A", 100), (12, "Helles", "L", 100);
