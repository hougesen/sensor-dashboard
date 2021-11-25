INSERT INTO measurementtypes (measurement_type_name)
VALUES ($1)
RETURNING $table_fields;