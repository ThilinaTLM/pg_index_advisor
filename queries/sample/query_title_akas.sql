-- Search
SELECT * FROM title_akas WHERE title = 'Baby''s Meal';
SELECT * FROM title_akas WHERE titleId = 'tt0000027';
SELECT * FROM title_akas WHERE ordering = 33;
SELECT * FROM title_akas WHERE region = 'JP';
SELECT * FROM title_akas WHERE language = 'French';
SELECT * FROM title_akas WHERE title = 'The Clown Barber';
SELECT * FROM title_akas WHERE titleId = 'tt0000228';
SELECT * FROM title_akas WHERE ordering = 2;
SELECT * FROM title_akas WHERE region = 'CA';
SELECT * FROM title_akas WHERE language = 'German';
SELECT * FROM title_akas WHERE title = 'Baignade en mer';
SELECT * FROM title_akas WHERE titleId = 'tt0000052';
SELECT * FROM title_akas WHERE ordering = 52;
SELECT * FROM title_akas WHERE region = 'IN';
SELECT * FROM title_akas WHERE language = 'English';
SELECT * FROM title_akas WHERE title = 'Nain grotesque';
SELECT * FROM title_akas WHERE titleId = 'tt0000116';
SELECT * FROM title_akas WHERE ordering = 16;
SELECT * FROM title_akas WHERE region = 'IN';
SELECT * FROM title_akas WHERE language = 'Chinese';

-- Update
UPDATE title_akas SET ordering = 95 WHERE titleId = 'tt0000006';
UPDATE title_akas SET ordering = 69 WHERE ordering = 68
UPDATE title_akas SET ordering = 99 WHERE region = 'US'
UPDATE title_akas SET ordering = 30 WHERE language = 'English'
UPDATE title_akas SET ordering = 61 WHERE titleId = 'tt0000085';
UPDATE title_akas SET ordering = 35 WHERE ordering = 34
UPDATE title_akas SET ordering = 68 WHERE region = 'CA'
UPDATE title_akas SET ordering = 81 WHERE language = 'English'
UPDATE title_akas SET ordering = 63 WHERE titleId = 'tt0000065';
UPDATE title_akas SET ordering = 100 WHERE ordering = 99
UPDATE title_akas SET ordering = 78 WHERE region = 'JP'
UPDATE title_akas SET ordering = 76 WHERE language = 'English'
UPDATE title_akas SET ordering = 82 WHERE titleId = 'tt0000089';
UPDATE title_akas SET ordering = 99 WHERE ordering = 98
UPDATE title_akas SET ordering = 64 WHERE region = 'US'
UPDATE title_akas SET ordering = 24 WHERE language = 'German'
UPDATE title_akas SET ordering = 3 WHERE titleId = 'tt0000199';
UPDATE title_akas SET ordering = 80 WHERE ordering = 79
UPDATE title_akas SET ordering = 73 WHERE region = 'UK'
UPDATE title_akas SET ordering = 78 WHERE language = 'Chinese'

-- Delete
DELETE FROM title_akas WHERE titleId = 'tt0000068';
DELETE FROM title_akas WHERE region = 'IN';
DELETE FROM title_akas WHERE language = 'Spanish';
DELETE FROM title_akas WHERE ordering = 79;
DELETE FROM title_akas WHERE titleId = 'tt0000199';
DELETE FROM title_akas WHERE region = 'UK';
DELETE FROM title_akas WHERE language = 'English';
DELETE FROM title_akas WHERE ordering = 89;
DELETE FROM title_akas WHERE titleId = 'tt0000215';
DELETE FROM title_akas WHERE region = 'CA';