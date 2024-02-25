extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;
extern crate reqwest;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use ev3dev_lang_rust::{sensors::ColorSensor, Ev3Result};
use menu::Menu;

mod menu;

fn main() -> Ev3Result<()> {
    let mut entries = Menu::new()?;

    entries.add("Hello World", handler_hello_world);
    entries.add("Color Sensor", handler_color_sensor);
    entries.add("Get own IP", handler_get_ip);

    entries.run()
}

fn handler_hello_world() {
    println!("Hello World");
}

fn handler_color_sensor() {
    if let Err(err) = println_color_sensor() {
        println!("Could not print color sensor value: {:?}", err)
    }
}

fn handler_get_ip() {
    let ip = get_own_ip();
    println!("IP: {:?}", ip);
}

fn println_color_sensor() -> Ev3Result<()> {
    let color_sensor = ColorSensor::find()?;
    color_sensor.set_mode_rgb_raw()?;
    println!("{:?}", color_sensor.get_rgb()?);
    Ok(())
}

fn get_own_ip() -> reqwest::Result<String> {
    let resp =
        reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;

    Ok(resp.get("origin").cloned().unwrap_or_default())
}
