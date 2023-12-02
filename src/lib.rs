#![no_std]

use cortex_m_semihosting::hprintln;
use stm32l4xx_hal::gpio::{Alternate, OpenDrain, Output, AF4, PB8, PB9};
use stm32l4xx_hal::{i2c::I2c, pac, prelude::*};

const I2C_SECOND_ADDR: u8 = 0x40;

/// Get the relative humidity of the sensor device, Hold Mater Mode.
#[allow(clippy::type_complexity)]
pub fn get_rel_humidity(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    // issue command to "Measure Relative Humidity, Hold Master Mode"
    let buf_i = [0xE5u8, 0];
    let mut buf_o = [0u8; 2];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut humidity: u16 = 0;
    if let Ok(_val) = result {
        humidity = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting humidity: {:?}", result);
    }
    humidity
}

/// Get the relative humidity of the sensor device, No Hold Mater Mode.
#[allow(clippy::type_complexity)]
pub fn get_rel_humidity_nh(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    // issue command to "Measure Relative Humidity, No Hold Master Mode"
    let buf_i = [0xF5u8, 0];
    let mut buf_o = [0u8; 2];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut humidity: u16 = 0;
    if let Ok(_val) = result {
        humidity = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting humidity: {:?}", result);
    }
    humidity
}

/// Get the relative temperature of the sensor device, Hold Master Mode.
#[allow(clippy::type_complexity)]
pub fn get_rel_temperature(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    // issue command to "Measure Temperature, Hold Master Mode"
    let buf_i = [0xE3u8, 0];
    let mut buf_o = [0u8; 2];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut temperature: u16 = 0;
    if let Ok(_val) = result {
        temperature = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting temperature: {:?}", result);
    }
    temperature
}

/// Get the relative temperature of the sensor device, No Hold Master Mode.
#[allow(clippy::type_complexity)]
pub fn get_rel_temperature_nh(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    // issue command to "Measure Temperature, No Hold Master Mode"
    let buf_i = [0xF3u8, 0];
    let mut buf_o = [0u8; 2];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut temperature: u16 = 0;
    if let Ok(_val) = result {
        temperature = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting temperature: {:?}", result);
    }
    temperature
}

/// Get the relative temperature from the previous relative humidity measurement of the sensor device.
#[allow(clippy::type_complexity)]
pub fn get_last_rel_temperature(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u16 {
    // issue command to "Read Temperature Value from Previous RH Measurement"
    let buf_i = [0xE0u8, 0];
    let mut buf_o = [0u8; 2];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut temperature: u16 = 0;
    if let Ok(_val) = result {
        temperature = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
    } else {
        hprintln!("error getting temperature: {:?}", result);
    }
    temperature
}

/// Send the Software Reset to the device to allow it to reset.
#[allow(clippy::type_complexity)]
pub fn reset_sensor(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) {
    let buf_i = [0xFEu8, 0];
    let mut buf_o = [0u8; 4];
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    let mut _reset_status: u16 = 0;
    if let Ok(_val) = result {
        _reset_status = (0xFF_00 & (buf_o[0] as u16) << 8) | 0x00_FF & (buf_o[1] as u16);
        //hprintln!("_reset_status: {}", _reset_status);
    } else {
        hprintln!("error sending reset sensor: {:?}", result);
    }
}

/// Read the Electronic ID 1st and 2nd Byte and return the 64-bit Serial Number.
#[allow(clippy::type_complexity)]
pub fn get_sensor_id(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u64 {
    let mut buf_i = [0xFAu8, 0x0Fu8];
    let mut buf_o = [0u8; 4];
    let mut sensor_id: u64 = 0;
    let mut result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    if let Ok(_val) = result {
        sensor_id = ((buf_o[0] as u64) << 56)
            | ((buf_o[1] as u64) << 48)
            | ((buf_o[2] as u64) << 40)
            | ((buf_o[3] as u64) << 32);
    } else {
        hprintln!("error getting sensor ID: {:?}", result);
    }
    buf_i[0] = 0xFC;
    buf_i[1] = 0xC9;
    result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    if let Ok(_val) = result {
        sensor_id = (sensor_id + ((buf_o[0] as u64) << 24))
            | ((buf_o[1] as u64) << 16)
            | ((buf_o[2] as u64) << 8)
            | (buf_o[3] as u64);
    } else {
        hprintln!("error getting sensor ID: {:?}", result);
    }
    sensor_id
}

/// Get the firmware version from the sensor device.
#[allow(clippy::type_complexity)]
pub fn get_fw_version(
    i2c_dev: &mut stm32l4xx_hal::i2c::I2c<
        stm32l4xx_hal::pac::I2C1,
        (
            PB8<Alternate<AF4, Output<OpenDrain>>>,
            PB9<Alternate<AF4, Output<OpenDrain>>>,
        ),
    >,
) -> u8 {
    let buf_i = [0x84u8, 0xB8u8];
    let mut buf_o = [0u8, 2];
    let mut fw_ver: u8 = 0;
    let result = i2c_dev.write_read(I2C_SECOND_ADDR, &buf_i, &mut buf_o);
    if let Ok(_val) = result {
        fw_ver = buf_o[0];
    } else {
        hprintln!("error getting firmware version: {:?}", result);
    }
    fw_ver
}

/// Print the firmware version to the semi-hosting shell.
pub fn hprint_fw_version(fw_ver: u8) {
    if fw_ver == 0xFF {
        hprintln!("Device FW version: 1.0");
    } else if fw_ver == 0x20 {
        hprintln!("Device FW version: 2.0");
    } else {
        hprintln!("Device FW version: unknown");
    }
}

/// Print the sensor id to the semi-hosting shell.
pub fn hprint_sensor_id(sensor_id: u64) {
    let snb_3 = (0x0000_0000_FF00_0000 & sensor_id) >> 24;
    let mut _dev_id: &str = Default::default();
    if snb_3 == 0x00 || snb_3 == 0xFF {
        _dev_id = "engineering sample";
    } else if snb_3 == 0x0D {
        _dev_id = "Si7013";
    } else if snb_3 == 0x14 {
        _dev_id = "Si7020";
    } else if snb_3 == 0x15 {
        _dev_id = "Si7021";
    } else {
        _dev_id = "unknown";
    }
    hprintln!("Device ID (snb_3): {}", _dev_id);
    hprintln!("RAW sensor ID: {:#08X}", sensor_id);
}

/// Print the relative temperature to the semi-hosting shell.
///
/// Formula for relative temperature conversion
/// ( (175.72 * Temp_code) / 65536) - 46.85 = Relative Temperature in Celcius
///
pub fn hprint_temperature(temperature: u16) {
    let temper_c = ((175.72 * temperature as f32) / 65536.0) - 46.85;
    let temper_f = (temper_c * 1.8) + 32.0;
    hprintln!(
        "Relative temperature: {} Celcius {} Fahrenheit",
        temper_c,
        temper_f
    );
}

/// Print the relative humidity to the semi-hosting shell.
///
/// Formula for Relative Humidity % conversion
/// ( (125 * RH_code) / 65536) - 6 = % Relative Humidity
///
pub fn hprint_humidity(humidity: u16) {
    let percent_rh: f32 = ((125.0 * humidity as f32) / 65536.0) - 6.0;
    hprintln!("% Relative Humidity: {} % RH", percent_rh);
}

/// Initialize the I2C settings for the STM32L476 to talk to the Si7021 temperature sensor.
/// Setup GPIO pins pb8 and pb9 as ouput and scl and sda respectively.
#[allow(clippy::type_complexity)]
pub fn i2c_init() -> stm32l4xx_hal::i2c::I2c<
    stm32l4xx_hal::pac::I2C1,
    (
        PB8<Alternate<AF4, Output<OpenDrain>>>,
        PB9<Alternate<AF4, Output<OpenDrain>>>,
    ),
> {
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut power = dp.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut power);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let scl = gpiob
        .pb8
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let sda = gpiob
        .pb9
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1r1)
}
