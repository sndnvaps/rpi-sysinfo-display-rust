0.96 inch oled, chipset is ssd1306 , size: 128x64


openwrt rpi need to install zoneinfo-asia first

$opkg install zoneinfo-asia
$cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
$touch /etc/timezone
$echo 'Asia/Shanghai' > /etc/timezone

modify /etc/config/system

set zonename to 'Asia/Shanghai'
set timezone to 'CST-8' or 'UTC-8'