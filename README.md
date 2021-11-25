# sensor-dashboard

Sensor dashboard built around the BME680 sensor for tracking temperature, humidity, pressure and gas readings.

Created using Rust, Python and PostgreSQL.

### Dashboard

The purpose of the dashboard is to easily track changes in the sensor readings. I am planning on creating the dashboard using React or Nuxt.js - still haven't decided, yet.

### Sensor

The sensor reader is written in Python using the BME680 library created by the people at Pimoroni.

It is set to read the sensor data every minute. The data is then sent to the api for tracking.

### API

The api is currently a basic rest api written in Rust & Actix connected to a PostgreSQL database.

### Database

The project is built around a PostgreSQL database with the following tables:

-   Locations

    -   location_id INT PK,
    -   location_name VARCHAR(255) NOT NULL

-   MeasurementTypes

    -   measurement_type_id INT PK
    -   measurement_type_name VARCHAR(255)

-   Measurements
    -   measurement_id INT PK
    -   measurement_type_id INT FK (Ref: MeasurementTypes.measurement_type_id)
    -   location_id INT FK (Ref: Locations.location_id)
    -   measurement_value DECIMAL
    -   measurement_time TIMESTAMP
