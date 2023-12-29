# si70xx Library
This library can be used to measure the relative temperature and humidity from the si70xx sensor boards from Silicon Labs.<br>

|Command Description                                    | Command Code        |Function Implemented?|
|-------------------------------------------------------|---------------------|---------------------|
|Measure Relative Humidity, Hold Master Mode            | 0xE5                |yes                  |
|Measure Relative Humidity, No Hold Master Mode         | 0xF5                |yes                  |
|Measure Temperature, Hold Master Mode                  | 0xE3                |yes                  |
|Measure Temperature, No Hold Master Mode               | 0xF3                |yes                  |
|Read Temperature Value from Previous RH Measurement    | 0xE0                |yes                  |
|Reset                                                  | 0xFE                |yes                  |
|Read Electronic ID 1st Byte                            | 0xFA 0x0F           |yes                  |
|Read Electronic ID 2nd Byte                            | 0xFC 0xC9           |yes                  |
|Read Firmware Revision                                 | 0x84 0xB8           |yes                  |
|Write RH/T User Register 1                             | 0xE6                |no                   |
|Read RH/T User Register 1                              | 0xE7                |no                   |
|Write Heater Control Register                          | 0x51                |no                   |
|Read Heater Control Register                           | 0x11                |no                   |

# Testing
I only own the Si7021-A20 as of writing this.  It would be good to get the other sensors
to test this library with.


# Sample Output via openocd from using this lib in stm32-rs application
stm32-rs application which displays relative temp/humidity

Device FW version: 2.0<br>
Device ID (snb_3): Si7021<br>
RAW sensor ID: 0x3D891CCC15FFB5FF<br>
Temperature: 10.37376 C 50.672768 F<br>
% Rel Humidity: 34.683746 % RH<br>
<br>
Temperature: 10.33086 C 50.59555 F<br>
% Rel Humidity: 34.668488 % RH<br>
<br>
Temperature: 10.30941 C 50.55694 F<br>
% Rel Humidity: 34.668488 % RH<br>

# Sample Output via itmdump from using this lib in stm32-rs application
Device FW version: 2.0<br>
Device ID (snb_3): Si7021<br>
RAW sensor ID: 0x3D891CCC15FFB5FF<br>
Rel Temperature: 21.125671 C 70.02621 F<br>
% Rel Humidity: 34.691376 % RH<br>
<br>
Rel Temperature: 21.104218 C 69.987595 F<br>
% Rel Humidity: 34.66086 % RH<br>
