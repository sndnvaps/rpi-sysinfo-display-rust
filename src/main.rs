extern crate alloc;
extern crate linux_embedded_hal as hal;
extern crate ssd1306;

use std::thread;
use std::time::Duration;

pub mod sysinfo;

#[macro_use]
extern crate scan_fmt;
use std::error::Error;

use alloc::string::ToString;
use chrono::Local;
use local_ipaddress as ip;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Baseline, Text},
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use hal::I2cdev;

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // just make a loop
    loop {
        let rpi_ip = ip::get().unwrap();
        let rpi_time = Local::now().format("%Y-%m-%d %H:%M").to_string();
        let cpu_temp = sysinfo::get_cpu_temp();
        let meminfo = sysinfo::get_ram();
        let cpu_usage = sysinfo::get_sys_cpu_usage();
        println!("main loop cpu_use = {}",cpu_usage);

        // println!("MemTotal {} kB,MemFree {} kB",meminfo.total,meminfo.free);

        let ip_info_show = "IP: ".to_owned() + &rpi_ip;
        let meminfo_show = "RAM: ".to_owned()
            + &((meminfo.total - meminfo.free) / 1024).to_string()
            + " / "
            + &(meminfo.total / 1024).to_string()
            + " MB";
        let cpu_info_show = format!("CPU: {}% {}`C",cpu_usage,&cpu_temp);
        println!("cpu_info_show -> {}",cpu_info_show);

        //let cpu_info_show = "CPU LOAD: ".to_owned() + &cpu_usage + "% " + &cpu_temp + &"`C";

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(&rpi_time, Point::new(3, 2), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(&ip_info_show, Point::new(3, 18), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(&cpu_info_show, Point::new(3, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(&meminfo_show, Point::new(3, 46), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Line::new(Point::new(0, 0), Point::new(127, 0))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        Line::new(Point::new(0, 0), Point::new(0, 63))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        Line::new(Point::new(0, 63), Point::new(127, 63))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        Line::new(Point::new(127, 0), Point::new(127, 63))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        Line::new(Point::new(0, 16), Point::new(127, 16))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        Line::new(Point::new(0, 15), Point::new(127, 15))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
        //need to clear_buffer() after flush
        display.clear_buffer();

        thread::sleep(Duration::from_secs(10));
    }
}
