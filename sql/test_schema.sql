-- Create the first table
CREATE TABLE table1 (
    id SERIAL PRIMARY KEY,
    name TEXT,
    value INTEGER
);

-- Insert some data into the first table
INSERT INTO table1 (name, value) VALUES
('Name1', 1),
('Name2', 2),
('Name3', 3);

-- Create an index on the 'name' column of the first table
CREATE INDEX table1_name_idx ON table1 (name);


-- Create the second table
CREATE TABLE table2 (
    id SERIAL PRIMARY KEY,
    description TEXT,
    value DOUBLE PRECISION
);

-- Insert some data into the second table
INSERT INTO table2 (description, value) VALUES
('Description1', 1.1),
('Description2', 2.2),
('Description3', 3.3);

-- Create an index on the 'value' column of the second table
CREATE INDEX table2_value_idx ON table2 (value);
