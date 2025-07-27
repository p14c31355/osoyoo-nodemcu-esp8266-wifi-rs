#![no_std]
#![no_main]

use esp8266_hal::{
    gpio::{Gpio5, Output, PushPull}, // D1ピンはGPIO5に相当
    prelude::*,
    target::Peripherals,
    timer::Timer,
    // Uartを使う場合、Tx/Rxピンも指定する必要がある
    uart::{Config, Uart}, // Uart0Tx, Uart0Rx は使わないのであれば削除
    rom_functions,
};
use esp_println::println;

#[esp_hal_macro::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    // Uartの初期化部分、Tx/Rxピンの指定を正確に行う
    // NodeMCUの場合、GPIO1はTXD0、GPIO3はRXD0
    let mut uart0 = Uart::new(
        dp.UART0,
        esp8266_hal::uart::Uart0Tx::new(dp.GPIO1), // GPIO1 for TX
        esp8266_hal::uart::Uart0Rx::new(dp.GPIO3), // GPIO3 for RX
        Config::default()
    );

    println!(&mut uart0, "Hello, ESP8266 L-Chika!");

    let io = dp.GPIO.split();
    // D1ピン（GPIO5）を出力として設定
    let mut led: Gpio5<Output<PushPull>> = io.gpio5.into_push_pull_output();

    let mut timer = Timer::new(dp.TIMER0); // Timer0も使用可能

    loop {
        println!(&mut uart0, "LED ON");
        led.set_high().unwrap(); // LEDを点灯
        rom_functions::ets_delay_us(1_000_000); // 1秒待機 (マイクロ秒単位)

        println!(&mut uart0, "LED OFF");
        led.set_low().unwrap(); // LEDを消灯
        rom_functions::ets_delay_us(1_000_000); // 1秒待機 (マイクロ秒単位)
    }
}

// パニックハンドラ (必須)
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("Panic: {:?}", info);
    // UARTが出力されるまで少し待つ
    rom_functions::ets_delay_us(100_000);
    loop {}
}