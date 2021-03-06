CREATE TABLE deltas (
    id SERIAL PRIMARY KEY,
    prediction_id INTEGER NOT NULL REFERENCES predictions(id),
    name VARCHAR NOT NULL,
    value REAL NOT NULL,
    positive_uncertainty REAL NOT NULL CHECK(positive_uncertainty >= 0),
    negative_uncertainty REAL NOT NULL CHECK(negative_uncertainty >= 0),
    repetition SMALLINT NOT NULL CHECK(repetition BETWEEN 1 AND 4), -- 1 = monthly, 2 = weekly, 3 = daily, 4 = once
    start_on DATE NOT NULL,
    end_on DATE CHECK((repetition = 4 AND end_on IS NULL) OR (repetition != 4 AND end_on >= start_on)),
    repeat_day SMALLINT CHECK ((repetition = 1 AND repeat_day BETWEEN 1 AND 31) OR (repetition != 1 AND repeat_day IS NULL)),
    repeat_weekday VARCHAR CHECK((repetition = 2 AND repeat_weekday = ANY('{Mon,Tue,Wed,Thu,Fri,Sat,Sun}'::VARCHAR[])) OR (repetition != 2 AND repeat_weekday IS NULL))
)