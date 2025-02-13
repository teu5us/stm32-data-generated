
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "AdcCommon",
            extends: None,
            description: Some(
                "ADC common registers",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "ADC Common status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    byte_offset: 0x4,
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
                BlockItem {
                    name: "cdr",
                    description: Some(
                        "ADC common regular data register for dual and triple modes",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdr",
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
                    name: "multi",
                    description: Some(
                        "Multi ADC mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Multi",
                    ),
                },
                Field {
                    name: "delay",
                    description: Some(
                        "Delay between 2 sampling phases",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dds",
                    description: Some(
                        "DMA disable selection for multi-ADC mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dds",
                    ),
                },
                Field {
                    name: "dma",
                    description: Some(
                        "Direct memory access mode for multi ADC mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dma",
                    ),
                },
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
                    enumm: Some(
                        "Adcpre",
                    ),
                },
                Field {
                    name: "vbate",
                    description: Some(
                        "VBAT enable",
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
            name: "Cdr",
            extends: None,
            description: Some(
                "ADC common regular data register for dual and triple modes",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "1st data item of a pair of regular conversions",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
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
                    name: "awd",
                    description: Some(
                        "Analog watchdog event occurred",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "eoc",
                    description: Some(
                        "End of conversion of ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jeoc",
                    description: Some(
                        "Injected channel end of conversion of ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jstrt",
                    description: Some(
                        "Injected channel conversion started",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "strt",
                    description: Some(
                        "regular channel conversion started",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ovr",
                    description: Some(
                        "Overrun occurred",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Adcpre",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PCLK2 divided by 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PCLK2 divided by 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PCLK2 divided by 6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PCLK2 divided by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dds",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "No new DMA request is issued after the last transfer",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CONTINUOUS",
                    description: Some(
                        "DMA requests are issued as long as data are converted and DMA=01, 10 or 11",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dma",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "DMA mode disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MODE1",
                    description: Some(
                        "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MODE2",
                    description: Some(
                        "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MODE3",
                    description: Some(
                        "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Multi",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some(
                        "All the ADCs independent: independent mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DUAL_RJ",
                    description: Some(
                        "Dual ADC1 and ADC2, combined regular and injected simultaneous mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DUAL_RA",
                    description: Some(
                        "Dual ADC1 and ADC2, combined regular and alternate trigger mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DUAL_J",
                    description: Some(
                        "Dual ADC1 and ADC2, injected simultaneous mode only",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DUAL_R",
                    description: Some(
                        "Dual ADC1 and ADC2, regular simultaneous mode only",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DUAL_I",
                    description: Some(
                        "Dual ADC1 and ADC2, interleaved mode only",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DUAL_A",
                    description: Some(
                        "Dual ADC1 and ADC2, alternate trigger mode only",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "TRIPLE_RJ",
                    description: Some(
                        "Triple ADC, regular and injected simultaneous mode",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "TRIPLE_RA",
                    description: Some(
                        "Triple ADC, regular and alternate trigger mode",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "TRIPLE_J",
                    description: Some(
                        "Triple ADC, injected simultaneous mode only",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "TRIPLE_R",
                    description: Some(
                        "Triple ADC, regular simultaneous mode only",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "TRIPLE_I",
                    description: Some(
                        "Triple ADC, interleaved mode only",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "TRIPLE_A",
                    description: Some(
                        "Triple ADC, alternate trigger mode only",
                    ),
                    value: 24,
                },
            ],
        },
    ],
};
                