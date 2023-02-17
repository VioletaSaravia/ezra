CREATE TABLE
    Families (
        famId INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );

CREATE TABLE
    FamiliesWords (
        word TEXT PRIMARY KEY,
        family INTEGER NOT NULL
    );

CREATE TABLE
    Keywords (
        keyId INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        priority INTEGER NOT NULL
    );

CREATE TABLE
    Decompositions (
        decId INTEGER PRIMARY KEY,
        keyword INTEGER NOT NULL,
        value TEXT NOT NULL
    );

CREATE TABLE
    Recompositions (
        recId INTEGER PRIMARY KEY,
        decomposition INTEGER NOT NULL,
        value TEXT NOT NULL,
        history INTEGER NULL
    );

CREATE TABLE transformations (
        from
            TEXT PRIMARY KEY,
            to TEXT NOT NULL
    );