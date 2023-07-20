-- Table 1: customer
-- Queries:
SELECT * FROM customer WHERE c_id > 500;
SELECT * FROM customer WHERE c_w_id BETWEEN 100 AND 200;
SELECT * FROM customer WHERE c_credit = 'GC';
SELECT * FROM customer WHERE c_first LIKE 'John%';
SELECT * FROM customer WHERE c_balance IS NOT NULL;
SELECT * FROM customer WHERE c_payment_cnt < 100;
SELECT * FROM customer WHERE c_ytd_payment >= 1000;
SELECT * FROM customer WHERE c_delivery_cnt <= 50;
SELECT * FROM customer WHERE c_discount > 0.05;
SELECT * FROM customer WHERE c_credit_lim IN (5000, 10000, 15000);

-- Indexes:
CREATE INDEX idx_customer_c_id ON customer USING btree (c_id);
CREATE INDEX idx_customer_c_w_id ON customer USING btree (c_w_id);
CREATE INDEX idx_customer_c_credit ON customer USING btree (c_credit);
CREATE INDEX idx_customer_c_first ON customer USING btree (c_first text_pattern_ops);
CREATE INDEX idx_customer_c_balance ON customer USING brin (c_balance);
CREATE INDEX idx_customer_c_payment_cnt ON customer USING btree (c_payment_cnt);
CREATE INDEX idx_customer_c_ytd_payment ON customer USING btree (c_ytd_payment);
CREATE INDEX idx_customer_c_delivery_cnt ON customer USING btree (c_delivery_cnt);
CREATE INDEX idx_customer_c_discount ON customer USING btree (c_discount);
CREATE INDEX idx_customer_c_credit_lim ON customer USING btree (c_credit_lim);

-- Running time without indexes : 36781  ms
-- Running time with indexes : 32774  ms
-- Customer table size : 199 MB

-- Table 2: district
-- Queries:
SELECT * FROM district WHERE d_w_id > 500;
SELECT * FROM district WHERE d_id BETWEEN 100 AND 200;
SELECT * FROM district WHERE d_name = 'District10';
SELECT * FROM district WHERE d_street_1 LIKE 'Main%';
SELECT * FROM district WHERE d_ytd IS NOT NULL;
SELECT * FROM district WHERE d_tax < 0.07;
SELECT * FROM district WHERE d_city >= 'City500';
SELECT * FROM district WHERE d_state <= 'State200';
SELECT * FROM district WHERE d_zip > '12345';
SELECT * FROM district WHERE d_next_o_id IN (5000, 10000, 15000);

-- Indexes:
CREATE INDEX idx_district_d_w_id ON district USING btree (d_w_id);
CREATE INDEX idx_district_d_id ON district USING btree (d_id);
CREATE INDEX idx_district_d_name ON district USING btree (d_name);
CREATE INDEX idx_district_d_street_1 ON district USING btree (d_street_1 text_pattern_ops);
CREATE INDEX idx_district_d_ytd ON district (d_id) WHERE d_ytd IS NOT NULL;
CREATE INDEX idx_district_d_tax ON district USING btree (d_tax);
CREATE INDEX idx_district_d_city ON district USING btree (d_city);
CREATE INDEX idx_district_d_state ON district USING btree (d_state);
CREATE INDEX idx_district_d_zip ON district USING btree (d_zip);
CREATE INDEX idx_district_d_next_o_id ON district USING btree (d_next_o_id);

-- Running time without indexes : 52 ms
-- Running time with indexes : 54 ms
-- district table size : 57 KB

-- Table 3: history
-- Queries:
SELECT * FROM history WHERE h_c_id > 500;
SELECT * FROM history WHERE h_c_w_id BETWEEN 100 AND 200;
SELECT * FROM history WHERE h_w_id = 50;
SELECT * FROM history WHERE h_c_d_id < 5;
SELECT * FROM history WHERE h_d_id >= 10;
SELECT * FROM history WHERE h_amount < 1000;
SELECT * FROM history WHERE h_date BETWEEN '2023-01-01' AND '2023-12-31';
SELECT * FROM history WHERE h_data LIKE 'Transaction%';
SELECT * FROM history WHERE h_c_id IS NOT NULL;
SELECT * FROM history WHERE h_c_w_id IN (5000, 10000, 15000);

-- Indexes:
CREATE INDEX idx_history_h_c_id ON history USING btree (h_c_id);
CREATE INDEX idx_history_h_c_w_id ON history USING btree (h_c_w_id);
CREATE INDEX idx_history_h_w_id ON history USING btree (h_w_id);
CREATE INDEX idx_history_h_c_d_id ON history USING btree (h_c_d_id);
CREATE INDEX idx_history_h_d_id ON history USING btree (h_d_id);
CREATE INDEX idx_history_h_amount ON history USING btree (h_amount);
CREATE INDEX idx_history_h_date ON history USING btree (h_date);
CREATE INDEX idx_history_h_data ON history USING btree (h_data text_pattern_ops);
CREATE INDEX idx_history_h_c_id_not_null ON history (h_c_id) WHERE h_c_id IS NOT NULL;
CREATE INDEX idx_history_h_c_w_id ON history USING btree (h_c_w_id);


-- Running time without indexes :  4791 ms
-- Running time with indexes :  4749 ms
-- Customer table size : 27 MB

-- Table 4: orders
-- Queries:
SELECT * FROM orders WHERE o_id > 500;
SELECT * FROM orders WHERE o_w_id BETWEEN 100 AND 200;
SELECT * FROM orders WHERE o_c_id = 10;
SELECT * FROM orders WHERE o_d_id < 100;
SELECT * FROM orders WHERE o_carrier_id LIKE 'Carrier%';
SELECT * FROM orders WHERE o_ol_cnt IS NOT NULL;
SELECT * FROM orders WHERE o_all_local IN (0, 1);
SELECT * FROM orders WHERE o_id >= 50;
SELECT * FROM orders WHERE o_c_id <= 200;
SELECT * FROM orders WHERE o_w_id > 20;

-- Indexes:
CREATE INDEX idx_orders_o_id ON orders USING btree (o_id);
CREATE INDEX idx_orders_o_w_id ON orders USING btree (o_w_id);
CREATE INDEX idx_orders_o_c_id ON orders USING btree (o_c_id);
CREATE INDEX idx_orders_o_d_id ON orders USING btree (o_d_id);
CREATE INDEX idx_orders_o_carrier_id ON orders USING btree (o_carrier_id);
CREATE INDEX idx_orders_o_ol_cnt_not_null ON orders (o_ol_cnt) WHERE o_ol_cnt IS NOT NULL;
CREATE INDEX idx_orders_o_all_local ON orders USING btree (o_all_local);
CREATE INDEX idx_orders_o_id ON orders USING btree (o_id);
CREATE INDEX idx_orders_o_c_id ON orders USING btree (o_c_id);
CREATE INDEX idx_orders_o_w_id ON orders USING btree (o_w_id);

-- Running time without indexes : 6323 seconds
-- Running time with indexes :  5694 second
-- Customer table size : 27 MB