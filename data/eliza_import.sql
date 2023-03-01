INSERT INTO
    Keywords (name, priority, memory)
VALUES ('sorry', 0, false), ('remember', 5, false);

INSERT INTO
    Decompositions (keyword, value)
VALUES ( (
            SELECT keyId
            FROM Keywords
            WHERE
                name = "sorry"
        ),
        "sorry\w*"
    );

WITH sorry AS (
        SELECT decId
        FROM decompositions
        WHERE
            value = "sorry\w*"
    )
INSERT INTO
    Recompositions (decomposition, value, history)
VALUES (
        sorry,
        "Please don't apologize.",
        0
    ), (
        sorry,
        "Apologies are not necessary.",
        0
    ), (
        sorry,
        "What feelings do you have when you apologize?",
        0
    ), (
        sorry,
        "I've told you that apologies are not required.",
        0
    );

INSERT INTO Transformations
VALUES ("dont", "don't"), ("cant", "can't"), ("wont", "won't"), ("i", "you"), ("you", "i");

INSERT INTO Families (name) VALUES ("belief"), ("family");

with belief AS (
        SELECT famId
        FROM Families
        WHERE
            name = "belief"
    ),
    fam AS (
        SELECT famId
        FROM Families
        WHERE name = "family"
    )
INSERT INTO FamiliesWords
VALUES ("feel", belief), ("think", belief), ("believe", belief), ("wish", belief), ("mother", fam), ("father", fam), ("sister", fam), ("brother", fam), ("wife", fam), ("husband", fam), ("children", fam);