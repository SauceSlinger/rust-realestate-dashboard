-- Seed data for testing

-- Insert sample properties
INSERT INTO properties (title, address, city, state, zip_code, property_type, bedrooms, bathrooms, square_feet, purchase_price, current_value, monthly_rent, status, notes)
VALUES 
    ('Sunset Villa', '123 Ocean View Dr', 'San Francisco', 'CA', '94122', 'residential', 3, 2.5, 2100, 850000, 920000, 4500, 'occupied', 'Beautiful ocean view property'),
    ('Downtown Loft', '456 Market St', 'San Francisco', 'CA', '94102', 'residential', 2, 2, 1400, 650000, 680000, 3200, 'occupied', 'Modern loft in SOMA'),
    ('Garden Cottage', '789 Green St', 'Seattle', 'WA', '98101', 'residential', 2, 1, 1100, 450000, 485000, 2400, 'vacant', 'Charming cottage with garden'),
    ('Commercial Plaza', '1000 Business Blvd', 'Austin', 'TX', '78701', 'commercial', NULL, NULL, 5000, 1200000, 1350000, 8500, 'occupied', 'Prime commercial location');

-- Insert sample tenants
INSERT INTO tenants (property_id, first_name, last_name, email, phone, lease_start, lease_end, monthly_rent, deposit_amount, status, notes)
VALUES
    (1, 'John', 'Doe', 'john.doe@email.com', '555-0101', '2024-01-01', '2025-01-01', 4500, 9000, 'active', 'Excellent tenant'),
    (2, 'Jane', 'Smith', 'jane.smith@email.com', '555-0102', '2024-03-01', '2025-03-01', 3200, 6400, 'active', 'Works from home');

-- Insert sample calendar events
INSERT INTO calendar_events (title, description, event_type, property_id, start_time, end_time, reminder_minutes, completed)
VALUES
    ('Annual Inspection - Sunset Villa', 'Yearly property inspection', 'inspection', 1, '2025-11-15 10:00:00', '2025-11-15 12:00:00', 1440, 0),
    ('Rent Due - Downtown Loft', 'Monthly rent payment', 'rent_due', 2, '2025-11-01 00:00:00', NULL, 4320, 0),
    ('HVAC Maintenance', 'Check and service HVAC system', 'maintenance', 1, '2025-11-20 14:00:00', '2025-11-20 16:00:00', 2880, 0),
    ('Lease Renewal - John Doe', 'Discuss lease renewal', 'lease_renewal', 1, '2024-12-01 15:00:00', '2024-12-01 16:00:00', 10080, 0);

-- Insert sample maintenance records
INSERT INTO maintenance_records (property_id, title, description, priority, status, cost, scheduled_date, contractor, notes)
VALUES
    (1, 'Fix leaking faucet', 'Kitchen faucet needs repair', 'medium', 'pending', 150, '2025-11-10 09:00:00', 'ABC Plumbing', 'Tenant reported issue'),
    (3, 'Repaint exterior', 'Exterior walls need fresh paint', 'low', 'pending', 2500, '2025-12-01 08:00:00', 'Premium Painters', 'Scheduled for winter'),
    (2, 'Replace air filter', 'Monthly HVAC maintenance', 'low', 'completed', 50, '2024-10-15 10:00:00', NULL, 'Completed on time');

-- Insert sample market data
INSERT INTO market_data (location, median_price, average_price, inventory_count, days_on_market, price_change_percent, data_source, recorded_date)
VALUES
    ('San Francisco, CA', 1200000, 1350000, 850, 45, -2.3, 'zillow', '2024-10-01'),
    ('San Francisco, CA', 1220000, 1365000, 820, 43, -1.8, 'zillow', '2024-10-15'),
    ('Seattle, WA', 850000, 920000, 1250, 38, 1.2, 'redfin', '2024-10-01'),
    ('Seattle, WA', 865000, 935000, 1180, 36, 1.8, 'redfin', '2024-10-15'),
    ('Austin, TX', 520000, 580000, 2100, 32, 3.5, 'zillow', '2024-10-01'),
    ('Austin, TX', 535000, 595000, 2050, 30, 2.9, 'zillow', '2024-10-15');
