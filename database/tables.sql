/* Tables */
CREATE TABLE locations (
    location_id SERIAL PRIMARY KEY,
    location_name VARCHAR(255) NOT NULL,
    UNIQUE(location_name)
);
CREATE TABLE measurement_types (
    measurement_type_id SERIAL PRIMARY KEY,
    measurement_type_name VARCHAR(255) NOT NULL,
    UNIQUE(measurement_type_name)
);
CREATE TABLE measurements (
    measurement_id SERIAL PRIMARY KEY,
    measurement_type_id INT NOT NULL,
    location_id INT NOT NULL,
    measurement_value REAL NOT NULL,
    measurement_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_measurement_type_id FOREIGN KEY (measurement_type_id) REFERENCES measurement_types(measurement_type_id),
    CONSTRAINT fk_location_id FOREIGN KEY (location_id) REFERENCES locations(location_id)
);