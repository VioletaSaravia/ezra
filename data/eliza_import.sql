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

INSERT INTO
    Recompositions (decomposition, value, history)
VALUES ( (
            SELECT decId
            FROM
                decompositions
            WHERE
                value = "sorry\w*"
        ),
        "Please don't apologize.",
        0
    ), ( (
            SELECT decId
            FROM
                decompositions
            WHERE
                value = "sorry\w*"
        ),
        "Apologies are not necessary.",
        0
    ), ( (
            SELECT decId
            FROM
                decompositions
            WHERE
                value = "sorry\w*"
        ),
        "What feelings do you have when you apologize?",
        0
    ), ( (
            SELECT decId
            FROM
                decompositions
            WHERE
                value = "sorry\w*"
        ),
        "I've told you that apologies are not required.",
        0
    );

INSERT INTO Transformations
VALUES ("dont", "don't"), ("cant", "can't"), ("wont", "won't"), ("i", "you"), ("you", "i");

INSERT INTO Families (name) VALUES ("belief"), ("family");

INSERT INTO FamiliesWords
VALUES (
        "feel", (
            SELECT famId
            FROM Families
            WHERE
                name = "belief"
        )
    ), (
        "think", (
            SELECT famId
            FROM Families
            WHERE
                name = "belief"
        )
    ), (
        "believe", (
            SELECT famId
            FROM Families
            WHERE
                name = "belief"
        )
    ), (
        "wish", (
            SELECT famId
            FROM Families
            WHERE
                name = "belief"
        )
    ), (
        "mother", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    ), (
        "father", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    ), (
        "sister", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    ), (
        "brother", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    ), (
        "wife", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    ), (
        "husband", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    ), (
        "children", (
            SELECT famId
            FROM Families
            WHERE
                name = "family"
        )
    );