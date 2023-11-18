# si70xx Library
This library is written in rust and can be used to measure the relative temperature and humidity from the si70xx sensor boards from Silicon Labs.<br>

The following functions are supported:<br>
    - get sensor id - Read the Sensor ID of the device.<br>
        Reads 1st and 2nd Electronic ID or word from device.<br>
        Results in a 64-bit Serial Number being returned.<br>
        The data sheet only calls out 1 byte's meaning, SNB3, the<br>
        fourth byte in the lower word.  That byte is compared to see which<br>
        version of the sensor it is.<br>
    - get firmware version - Reads the firmware version of the sensor.<br>
    - get relative temperature - Reads the relative temperature the sensor is detecting.<br>
    - get relative humidity - Reads the relative humidity the sensor is detecting.<br>

TODO:<br>
    - Add the following commands:<br>
        - get temperature - get absolute temperature<br>
        - get humidity - get absolute humidity<br>
        - get temperature from previous RH measurement - good for testing<br>
        - Reset - Reset the sensor?<br>
        - Figure out how to enable the Heater (section 5.5)<br>
        - Read/Write User Register - Used to control if the Heater is on or off<br>
            Bit map in section 6.1 of Si7021-A20.pdf<br>
            Read reg, or in bit 2, to enable Header Enable, then write reg back<br>
            Read reg, clear bit 2, to disable Header Enable, then write reg back<br>
    - Add checksum byte checks to all commands.  This will help detect transmission errors<br>
    - get a si7020 and Si7013 version of the sensor to test<br>

# Testing
I only own the Si7021-A20 as of writing this.  It would be good to get the other sensors
to test this library with.


# Sample Output from using this lib in stm32-rs application
stm32-rs application which displays relative temp/humidity

Device FW version: 2.0
Device ID (snb_3): Si7021
RAW sensor ID: 0x3D891CCC15FFB5FF
Temperature: 10.37376 Celcius 50.672768 Fahrenheit
% Relative Humidity: 34.683746 % RH

Temperature: 10.33086 Celcius 50.59555 Fahrenheit
% Relative Humidity: 34.668488 % RH

Temperature: 10.30941 Celcius 50.55694 Fahrenheit
% Relative Humidity: 34.668488 % RH

