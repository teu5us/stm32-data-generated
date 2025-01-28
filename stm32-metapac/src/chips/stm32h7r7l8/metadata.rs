include!("../metadata_0501.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32H7R7L8",
                family: "STM32H7",
                line: "STM32H7R7/7S7",
                memory: &[
    MemoryRegion {
        name: "ITCM",
        kind: MemoryRegionKind::Ram,
        address: 0x0,
        size: 196608,
        settings: None,
    },
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 65536,
        settings: Some(
            FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "DTCM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 196608,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x24000000,
        size: 131072,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM2",
        kind: MemoryRegionKind::Ram,
        address: 0x24020000,
        size: 131072,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM3",
        kind: MemoryRegionKind::Ram,
        address: 0x24040000,
        size: 131072,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM4",
        kind: MemoryRegionKind::Ram,
        address: 0x24060000,
        size: 73728,
        settings: None,
    },
    MemoryRegion {
        name: "AHB_SRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x30000000,
        size: 16384,
        settings: None,
    },
    MemoryRegion {
        name: "AHB_SRAM2",
        kind: MemoryRegionKind::Ram,
        address: 0x30004000,
        size: 16384,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };