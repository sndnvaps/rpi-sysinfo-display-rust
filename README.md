0.96 inch i2c  oled, chipset is ssd1306 , size: 128x64


openwrt rpi need to install zoneinfo-asia first

    $opkg install zoneinfo-asia
    $cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
    $touch /etc/timezone
    $echo 'Asia/Shanghai' > /etc/timezone

modify /etc/config/system

    set zonename to 'Asia/Shanghai'
    set timezone to 'CST-8' or 'UTC-8'

cross compile in host

    $cargo build --release --target=aarch64-unknown-linux-musl

use cargo-make to build the project

    $cargo install --force cargo-make
    $cargo make build
