-- Search
SELECT * FROM LAPTOP_DETAIL WHERE cpu_model = 'Intel Core i7';
SELECT * FROM LAPTOP_DETAIL WHERE cpu_speed = 2.4;
SELECT * FROM LAPTOP_DETAIL WHERE memory_size = 8;
SELECT * FROM LAPTOP_DETAIL WHERE memory_type = 'DDR4' AND cpu_speed BETWEEN 3.0 AND 4.0;
SELECT * FROM LAPTOP_DETAIL WHERE ip_address LIKE '192.168%';
SELECT * FROM LAPTOP_DETAIL WHERE cpu_model = 'AMD Ryzen 7' AND memory_type = 'LPDDR4';
SELECT * FROM LAPTOP_DETAIL WHERE memory_size <= 10 AND memory_type = 'DDR3';
SELECT * FROM LAPTOP_DETAIL WHERE cpu_speed > 3.5 AND memory_size BETWEEN 15 AND 25;
SELECT * FROM LAPTOP_DETAIL WHERE memory_size = 32 OR memory_size = 4;
SELECT * FROM LAPTOP_DETAIL WHERE ip_address LIKE '%.37';
SELECT * FROM LAPTOP_DETAIL WHERE cpu_speed < 2.0 OR memory_size > 30;


-- Update
UPDATE LAPTOP_DETAIL SET cpu_model = 'Intel Core i7' WHERE device_mac = '80-6B-C7-F4-85-90';
UPDATE LAPTOP_DETAIL SET cpu_speed = 3.0, memory_size = 16 WHERE device_mac = 'E4-5C-C3-D4-07-A0';
UPDATE LAPTOP_DETAIL SET memory_type = 'DDR4' WHERE memory_type = 'DDR3';
UPDATE LAPTOP_DETAIL SET cpu_speed = 2.0, memory_type = 'LPDDR4' WHERE cpu_model = 'Intel Core i5';
UPDATE LAPTOP_DETAIL SET cpu_model = 'AMD Ryzen 5' WHERE memory_size < 10;
UPDATE LAPTOP_DETAIL SET memory_size = 32 WHERE cpu_speed > 4.0;
UPDATE LAPTOP_DETAIL SET cpu_speed = 2.5, memory_type = 'LPDDR4' WHERE device_mac = '76-BA-7B-8F-E4-EE';
UPDATE LAPTOP_DETAIL SET ip_address = '10.0.0.1' WHERE memory_size = 24;
UPDATE LAPTOP_DETAIL SET memory_size = 8 WHERE memory_type = 'DDR4';


-- Delete
DELETE FROM LAPTOP_DETAIL WHERE device_mac = 'BF-4A-F1-89-25-55';
DELETE FROM LAPTOP_DETAIL WHERE memory_type = 'DDR3';
DELETE FROM LAPTOP_DETAIL WHERE cpu_speed < 2.0 AND memory_size > 20;
DELETE FROM LAPTOP_DETAIL WHERE ip_address = '33.155.196.115';
DELETE FROM LAPTOP_DETAIL WHERE cpu_model = 'Intel Core i5' AND memory_type = 'DDR4';
DELETE FROM LAPTOP_DETAIL WHERE memory_size = 16;
DELETE FROM LAPTOP_DETAIL WHERE cpu_speed > 3.0;
DELETE FROM LAPTOP_DETAIL WHERE memory_size = 32;
DELETE FROM LAPTOP_DETAIL WHERE ip_address = '33.135.196.115';
