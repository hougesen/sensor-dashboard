INSERT INTO measurements (
        measurement_type_id,
        location_id,
        measurement_value,
        measurement_time
    )
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;