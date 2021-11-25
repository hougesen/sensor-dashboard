/* Tables */
CREATE TABLE Locations (
    location_id SERIAL PRIMARY KEY,
    location_name VARCHAR(255) NOT NULL
);
CREATE TABLE MeasurementTypes (
    measurement_type_id SERIAL PRIMARY KEY,
    measurement_type_name VARCHAR(255) NOT NULL
);
CREATE TABLE Measurements (
    measurement_id SERIAL PRIMARY KEY,
    measurement_type_id INT NOT NULL,
    location_id INT NOT NULL,
    measurement_value DECIMAL NOT NULL,
    measurement_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (measurement_type_id) REFERENCES MeasurementTypes(measurement_type_id),
    FOREIGN KEY (location_id) REFERENCES Locations(location_id)
);