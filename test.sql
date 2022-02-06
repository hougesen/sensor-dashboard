SELECT AVG(measurements.measurement_value) AS average_value,
    measurements.measurement_time::date,
    locations.location_name AS location_name,
    measurement_types.measurement_type_name AS measurement_type_name
FROM measurements
    INNER JOIN locations ON measurements.location_id = locations.location_id
    INNER JOIN measurement_types ON measurements.measurement_type_id = measurement_types.measurement_type_id
WHERE measurements.location_id = 1
    AND measurements.measurement_type_id = 1
GROUP BY locations.location_name,
    measurement_types.measurement_type_name,
    measurements.measurement_time::date
ORDER BY measurements.measurement_time::date;