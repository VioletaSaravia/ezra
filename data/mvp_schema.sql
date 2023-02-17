CREATE TABLE
    Families (
        famId INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
    );

CREATE TABLE
    FamiliesWords (
        word TEXT PRIMARY KEY,
        family INTEGER NOT NULL
    );

CREATE TABLE
    Keywords (
        keyId INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        priority INTEGER NOT NULL,
        memory BOOLEAN NOT NULL
    );

CREATE TABLE
    Decompositions (
        decId INTEGER PRIMARY KEY AUTOINCREMENT,
        keyword INTEGER NOT NULL,
        value TEXT NOT NULL
    );

CREATE TABLE
    Recompositions (
        recId INTEGER PRIMARY KEY AUTOINCREMENT,
        decomposition INTEGER NOT NULL,
        value TEXT NOT NULL,
        history INTEGER NULL
    );

CREATE TABLE
    Transformations (
        from_word TEXT PRIMARY KEY,
        to_word TEXT NOT NULL
    );