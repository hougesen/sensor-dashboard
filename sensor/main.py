import requests
import bme680
import time
import datetime


# API HOST
HOST = "192.168.86.64:5000"

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


def read_sensor_data():
    if sensor.get_sensor_data():

        if sensor.data.temperature:
            print(f"temperature: {sensor.data.temperature}")
            send_sensor_data(sensor.data.temperature, 1)

        if sensor.data.pressure:
            print(f"pressure: {sensor.data.pressure}")
            send_sensor_data(sensor.data.pressure, 2)

        if sensor.data.humidity:
            print(f"humidity: {sensor.data.humidity}")
            send_sensor_data(sensor.data.humidity, 3)

        if sensor.data.heat_stable and sensor.data.gas_resistance:
            print(f"gas_resistance: {sensor.data.gas_resistance}")
            send_sensor_data(sensor.data.gas_resistance, 4)


def send_sensor_data(value, type):
    URL = "/measurement/new"
    try:
        requests.post(f"{HOST}/{URL}", data={
            "measurement_type_id": type,
            "location_id": LOCATION_ID,
            "measurement_value": value,
            "measurement_time": datetime.datetime.now()
        })

    except requests.exceptions.RequestException as error:
        print("Something went wrong sending sensor data", error)


try:
    while True:
        print(f"Starting new cycle {datetime.datetime.now()}")
        read_sensor_data()

        time.sleep(POLLING_FREQ)

except KeyboardInterrupt:
    pass
