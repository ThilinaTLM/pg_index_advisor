SELECT * FROM orders WHERE customer_id = 123;
SELECT * FROM users WHERE username = 'john_doe';
SELECT * FROM locations WHERE position <-> point '(5,5)' < 10;
SELECT * FROM documents WHERE to_tsvector('english', text) @@ to_tsquery('english', 'cat');
SELECT * FROM network WHERE ip_address << inet '192.168.1.0/24';
SELECT * FROM log_entries WHERE timestamp BETWEEN '2023-07-01' AND '2023-07-07';
SELECT customer_id FROM orders WHERE order_id = 1000;
SELECT * FROM products WHERE is_active = true AND category = 'Electronics';
INSERT INTO users (username, email) VALUES ('new_user', 'new_user@example.com');
SELECT * FROM employees WHERE lower(first_name) = 'john';

