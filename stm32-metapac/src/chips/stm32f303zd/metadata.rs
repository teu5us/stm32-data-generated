include!("../metadata_0136.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F303ZD",
    family: "STM32F3",
    line: "STM32F303",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 393216,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "CCMRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x10000000,
            size: 16384,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 65536,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
