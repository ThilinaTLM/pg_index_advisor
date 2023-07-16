CREATE TABLE title_akas (
    titleId VARCHAR(10),
    ordering INT,
    title VARCHAR(255),
    region VARCHAR(255),
    language VARCHAR(255),
    types VARCHAR(255),
    attributes VARCHAR(255),
    isOriginalTitle BOOLEAN
); 

-- CREATE TABLE title_akas (
--     titleId VARCHAR(10),
--     ordering INT,
--     title VARCHAR(255),
--     region VARCHAR(255),
--     language VARCHAR(255),
--     types VARCHAR(255)[] DEFAULT '{}'::VARCHAR[],
--     attributes VARCHAR(255)[] DEFAULT '{}'::VARCHAR[],
--     isOriginalTitle BOOLEAN
-- ); 