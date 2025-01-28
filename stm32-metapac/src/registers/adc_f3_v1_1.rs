
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "Analog-to-digital converter",
            ),
            items: &[
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "control register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "control register 2",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpr1",
                    description: Some(
                        "sample time register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpr2",
                    description: Some(
                        "sample time register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpr3",
                    description: Some(
                        "sample time register 3",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr1",
                    description: Some(
                        "injected channel data offset register 1",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr2",
                    description: Some(
                        "injected channel data offset register 2",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr3",
                    description: Some(
                        "injected channel data offset register 3",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jofr4",
                    description: Some(
                        "injected channel data offset register 4",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jofr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr",
                    description: Some(
                        "watchdog higher threshold register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr",
                    description: Some(
                        "watchdog lower threshold register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr1",
                    description: Some(
                        "regular sequence register 1",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr2",
                    description: Some(
                        "regular sequence register 2",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr3",
                    description: Some(
                        "regular sequence register 3",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr4",
                    description: Some(
                        "regular sequence register 4",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sqr5",
                    description: Some(
                        "regular sequence register 5",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sqr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jsqr",
                    description: Some(
                        "injected sequence register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jsqr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr1",
                    description: Some(
                        "injected data register x1",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr2",
                    description: Some(
                        "injected data register 2",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr3",
                    description: Some(
                        "injected data register 3",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr4",
                    description: Some(
                        "injected data register 4",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "regular data register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smpr0",
                    description: Some(
                        "sample time register 0",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr",
                    description: Some(
                        "ADC common status register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "ADC common control register",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "ADC common control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcpre",
                    description: Some(
                        "ADC prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsvrefe",
                    description: Some(
                        "Temperature sensor and VREFINT enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some(
                        "Analog watchdog channel select bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocie",
                    description: Some(
                        "Interrupt enable for EOC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdie",
                    description: Some(
                        "Analog watchdog interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeocie",
                    description: Some(
                        "Interrupt enable for injected channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scan",
                    description: Some(
                        "Scan mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdsgl",
                    description: Some(
                        "Enable the watchdog on a single channel in scan mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jauto",
                    description: Some(
                        "Automatic injected group conversion",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discen",
                    description: Some(
                        "Discontinuous mode on regular channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdiscen",
                    description: Some(
                        "Discontinuous mode on injected channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discnum",
                    description: Some(
                        "Discontinuous mode channel count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Discnum",
                    ),
                },
                Field {
                    name: "pdd",
                    description: Some(
                        "Power down during the delay phase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdi",
                    description: Some(
                        "Power down during the idle phase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jawden",
                    description: Some(
                        "Analog watchdog enable on injected channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awden",
                    description: Some(
                        "Analog watchdog enable on regular channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "res",
                    description: Some(
                        "Resolution",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Res",
                    ),
                },
                Field {
                    name: "ovrie",
                    description: Some(
                        "Overrun interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adon",
                    description: Some(
                        "A/D Converter ON / OFF",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cont",
                    description: Some(
                        "Continuous conversion",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc_cfg",
                    description: Some(
                        "ADC configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "AdcCfg",
                    ),
                },
                Field {
                    name: "dels",
                    description: Some(
                        "Delay selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Dels",
                    ),
                },
                Field {
                    name: "dma",
                    description: Some(
                        "Direct memory access mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dds",
                    description: Some(
                        "DMA disable selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocs",
                    description: Some(
                        "End of conversion selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "align",
                    description: Some(
                        "Data alignment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jextsel",
                    description: Some(
                        "External event select for injected group",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Jextsel",
                    ),
                },
                Field {
                    name: "jexten",
                    description: Some(
                        "External trigger enable for injected channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Exten",
                    ),
                },
                Field {
                    name: "jswstart",
                    description: Some(
                        "Start conversion of injected channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extsel",
                    description: Some(
                        "External event select for regular group",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Extsel",
                    ),
                },
                Field {
                    name: "exten",
                    description: Some(
                        "External trigger enable for regular channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Exten",
                    ),
                },
                Field {
                    name: "swstart",
                    description: Some(
                        "Start conversion of regular channels",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "ADC common status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd1",
                    description: Some(
                        "Analog watchdog flag of the ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc1",
                    description: Some(
                        "End of conversion of the ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc1",
                    description: Some(
                        "Injected channel end of conversion of the ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jstrt1",
                    description: Some(
                        "Injected channel Start flag of the ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strt1",
                    description: Some(
                        "Regular channel Start flag of the ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr1",
                    description: Some(
                        "Overrun flag of the ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adons1",
                    description: Some(
                        "ADON Status of ADC1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "regular data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata",
                    description: Some(
                        "Regular data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr",
            extends: None,
            description: Some(
                "watchdog higher threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ht",
                    description: Some(
                        "Analog watchdog higher threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jdr1",
            extends: None,
            description: Some(
                "injected data register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jdr2",
            extends: None,
            description: Some(
                "injected data register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jdr3",
            extends: None,
            description: Some(
                "injected data register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jdr4",
            extends: None,
            description: Some(
                "injected data register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jofr1",
            extends: None,
            description: Some(
                "injected channel data offset register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset1",
                    description: Some(
                        "Data offset for injected channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jofr2",
            extends: None,
            description: Some(
                "injected channel data offset register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset2",
                    description: Some(
                        "Data offset for injected channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jofr3",
            extends: None,
            description: Some(
                "injected channel data offset register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset3",
                    description: Some(
                        "Data offset for injected channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jofr4",
            extends: None,
            description: Some(
                "injected channel data offset register x",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "joffset4",
                    description: Some(
                        "Data offset for injected channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jsqr",
            extends: None,
            description: Some(
                "injected sequence register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jsq",
                    description: Some(
                        "conversion in injected sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jl",
                    description: Some(
                        "Injected sequence length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr",
            extends: None,
            description: Some(
                "watchdog lower threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lt",
                    description: Some(
                        "Analog watchdog lower threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Smpr0",
            extends: None,
            description: Some(
                "sample time register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "channel 30-31 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr1",
            extends: None,
            description: Some(
                "sample time register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "channel 20-29 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr2",
            extends: None,
            description: Some(
                "sample time register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "channel 10-19 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr3",
            extends: None,
            description: Some(
                "sample time register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "channel 0-9 sampling time selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 3,
                            },
                        ),
                    ),
                    enumm: Some(
                        "SampleTime",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sqr1",
            extends: None,
            description: Some(
                "regular sequence register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "25th-29th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "l",
                    description: Some(
                        "Regular channel sequence length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr2",
            extends: None,
            description: Some(
                "regular sequence register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "19th-24th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr3",
            extends: None,
            description: Some(
                "regular sequence register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "13th-18th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr4",
            extends: None,
            description: Some(
                "regular sequence register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "7th-12th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr5",
            extends: None,
            description: Some(
                "regular sequence register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "1st-6th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 5,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd",
                    description: Some(
                        "Analog watchdog flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc",
                    description: Some(
                        "Regular channel end of conversion",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc",
                    description: Some(
                        "Injected channel end of conversion",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jstrt",
                    description: Some(
                        "Injected channel start flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strt",
                    description: Some(
                        "Regular channel start flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr",
                    description: Some(
                        "Overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adons",
                    description: Some(
                        "ADC ON status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcnr",
                    description: Some(
                        "Regular channel not ready",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jcnr",
                    description: Some(
                        "Injected channel not ready",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "AdcCfg",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BANK_A",
                    description: Some(
                        "Bank A selected for channels ADC_IN0..31",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BANK_B",
                    description: Some(
                        "Bank B selected for channels ADC_IN0..31b",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dels",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_DELAY",
                    description: Some(
                        "No Delay",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AFTER_READ",
                    description: Some(
                        "Until the converted data have been read",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DELAY_7_CLK",
                    description: Some(
                        "Delay 7 APB clock cycles after the conversion",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DELAY_15_CLK",
                    description: Some(
                        "Delay 16 APB clock cycles after the conversion",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DELAY_31_CLK",
                    description: Some(
                        "Delay 31 APB clock cycles after the conversion",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DELAY_63_CLK",
                    description: Some(
                        "Delay 63 APB clock cycles after the conversion",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DELAY_127_CLK",
                    description: Some(
                        "Delay 127 APB clock cycles after the conversion",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DELAY_255_CLK",
                    description: Some(
                        "Delay 255 APB clock cycles after the conversion",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Discnum",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DISCNUM_1",
                    description: Some(
                        "1 conversions are discontinued and the conversion is carried out on one channel",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DISCNUM_2",
                    description: Some(
                        "2 conversion is discontinued and the conversions are carried out on 2 channels",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DISCNUM_3",
                    description: Some(
                        "3 conversions are discontinued and the conversions are carried out on 3 channels",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DISCNUM_4",
                    description: Some(
                        "4 conversions are discontinued and the conversions are carried out on 4 channels",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DISCNUM_5",
                    description: Some(
                        "5 conversions are discontinued and the conversions are carried out on 5 channels",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DISCNUM_6",
                    description: Some(
                        "6 conversions are discontinued and the conversions are carried out on 6 channels",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DISCNUM_7",
                    description: Some(
                        "7 conversions are discontinued and the conversions are carried out on 7 channels",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DISCNUM_8",
                    description: Some(
                        "8 conversions are discontinued and the conversions are carried out on 8 channels",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Exten",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Trigger detection disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RISING",
                    description: Some(
                        "Trigger detection on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLING",
                    description: Some(
                        "Trigger detection on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTH",
                    description: Some(
                        "Trigger detection on both edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Extsel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "TIM9_CC2",
                    description: Some(
                        "Timer 9 CC2 event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM9_TRGO",
                    description: Some(
                        "Timer 9 TRGO event",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2_CC3",
                    description: Some(
                        "Timer 2 CC3 event",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2_CC2",
                    description: Some(
                        "Timer 2 CC2 event",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM3_TRGO",
                    description: Some(
                        "Timer 3 TRGO event",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM4_CC4",
                    description: Some(
                        "Timer 4 CC4 event",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM2_TRGO",
                    description: Some(
                        "Timer 2 TRGO event",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "TIM3_CC1",
                    description: Some(
                        "Timer 3 CC1 event",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "TIM3_CC3",
                    description: Some(
                        "Timer 3 CC3 event",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "TIM4_TRGO",
                    description: Some(
                        "Timer 4 TRGO event",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "TIM6_TRGO",
                    description: Some(
                        "Timer 6 TRGO event",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "EXTI_LINE11",
                    description: Some(
                        "External interrupt line 11",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Jextsel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "TIM9_CC1",
                    description: Some(
                        "Timer 9 CC1 event",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM9_TRGO",
                    description: Some(
                        "Timer 9 TRGO event",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2_TRGO",
                    description: Some(
                        "Timer 2 TRGO event",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2_CC1",
                    description: Some(
                        "Timer 2 CC1 event",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM3_CC4",
                    description: Some(
                        "Timer 3 CC4 event",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM4_TRGO",
                    description: Some(
                        "Timer 4 TRGO event",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM4_CC1",
                    description: Some(
                        "Timer 4 CC1 event",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "TIM4_CC2",
                    description: Some(
                        "Timer 4 CC2 event",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "TIM4_CC3",
                    description: Some(
                        "Timer 4 CC3 event",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "TIM10_CC1",
                    description: Some(
                        "Timer 4 CC3 event",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "TIM7_TRGO",
                    description: Some(
                        "Timer 7 TRGO event",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "EXTI_LINE15",
                    description: Some(
                        "External interrupt line 15",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Res",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "BITS12",
                    description: Some(
                        "12-bit resolution",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS10",
                    description: Some(
                        "10-bit resolution",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "BITS8",
                    description: Some(
                        "8-bit resolution",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BITS6",
                    description: Some(
                        "6-bit resolution",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "SampleTime",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CYCLES4",
                    description: Some(
                        "4 ADC clock cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES9",
                    description: Some(
                        "9 ADC clock cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES16",
                    description: Some(
                        "16 ADC clock cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES24",
                    description: Some(
                        "24 ADC clock cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES48",
                    description: Some(
                        "48 ADC clock cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES96",
                    description: Some(
                        "96 ADC clock cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES192",
                    description: Some(
                        "192 ADC clock cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES384",
                    description: Some(
                        "384 ADC clock cycles",
                    ),
                    value: 7,
                },
            ],
        },
    ],
};
                