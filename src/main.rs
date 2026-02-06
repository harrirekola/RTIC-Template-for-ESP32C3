#![no_main]
#![no_std]

#[rtic::app(device = esp32c3, dispatchers=[FROM_CPU_INTR0, FROM_CPU_INTR1])]
mod app {
    use esp_backtrace as _;
    use esp_hal as _;
    use esp_println::println;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        println!("init");
        (Shared {}, Local {})
    }

    #[idle()]
    fn idle(_: idle::Context) -> ! {
        loop {
            println!("idle");
        }
    }
}
