# sensor-dashboard

Sensor dashboard built around the BME680 sensor, which supports measurring temperature, humidity, pressure and gas.

## Dashboard

## API

## Sensor

## Database

The project is built around a postgresql database with the following tables:

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
