import time
import bme680
import requests
from datetime import datetime

# API HOST
API_HOST = "192.168.86.64:5000"

# Location of pi
LOCATION_ID = 1

# How often to read sensor data
POLLING_FREQ = 1 * 60


try:
    sensor = bme680.BME680(bme680.I2C_ADDR_PRIMARY)
except (RuntimeError, IOError):
    sensor = bme680.BME680(bme680.I2C_ADDR_SECONDARY)


sensor.set_humidity_oversample(bme680.OS_2X)
sensor.set_pressure_oversample(bme680.OS_4X)
sensor.set_temperature_oversample(bme680.OS_8X)
sensor.set_filter(bme680.FILTER_SIZE_3)
sensor.set_gas_status(bme680.ENABLE_GAS_MEAS)
sensor.set_gas_heater_temperature(320)
sensor.set_gas_heater_duration(150)
sensor.select_gas_heater_profile(0)


def read_sensor_data() -> None:
    if sensor.get_sensor_data():

        if sensor.data.temperature:
            print(f"temperature: {sensor.data.temperature}")
            send_sensor_data(sensor.data.temperature, 1)

        if sensor.data.pressure:
            print(f"pressure: {sensor.data.pressure}")
            send_sensor_data(sensor.data.pressure, 2)

        if sensor.data.humidity is not None:
            print(f"humidity: {sensor.data.humidity}")
            send_sensor_data(sensor.data.humidity, 3)

        if sensor.data.heat_stable and sensor.data.gas_resistance:
            print(f"gas_resistance: {sensor.data.gas_resistance}")
            send_sensor_data(sensor.data.gas_resistance, 4)
        else:
            print("sensor.data.heat_stable is not stable")


def send_sensor_data(value: int, type: int) -> None:
    try:
        sensor_data = {
            "measurement_type_id": type,
            "location_id": LOCATION_ID,
            "measurement_value": value,
            "measurement_time": datetime.now().isoformat()
        }

        r = requests.post(f"{API_HOST}/measurement/new", json=sensor_data)
        print("r", r)

    except requests.exceptions.RequestException as error:
        print("Something went wrong sending sensor data", error)


try:
    while True:
        print(f"Starting new cycle {datetime.now()}")
        read_sensor_data()

        # Sleep
        time.sleep(POLLING_FREQ)

except KeyboardInterrupt:
    pass
