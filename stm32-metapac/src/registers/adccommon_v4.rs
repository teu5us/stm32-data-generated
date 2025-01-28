
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "AdcCommon",
            extends: None,
            description: Some(
                "Analog-to-Digital Converter",
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
                    byte_offset: 0x8,
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
                    byte_offset: 0xc,
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
                BlockItem {
                    name: "cdr2",
                    description: Some(
                        "ADC x common regular data register for 32-bit dual mode",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdr2",
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
                    name: "dual",
                    description: Some(
                        "Dual ADC mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Dual",
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
                    name: "damdf",
                    description: Some(
                        "Dual ADC Mode Data Format",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Damdf",
                    ),
                },
                Field {
                    name: "ckmode",
                    description: Some(
                        "ADC clock mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckmode",
                    ),
                },
                Field {
                    name: "presc",
                    description: Some(
                        "ADC prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Presc",
                    ),
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREFINT enable",
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
                    name: "vsenseen",
                    description: Some(
                        "Temperature sensor enable",
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
                    name: "vbaten",
                    description: Some(
                        "VBAT enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
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
                    name: "rdata_mst",
                    description: Some(
                        "Regular data of the master ADC",
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
                Field {
                    name: "rdata_slv",
                    description: Some(
                        "Regular data of the slave ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cdr2",
            extends: None,
            description: Some(
                "ADC x common regular data register for 32-bit dual mode",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata_alt",
                    description: Some(
                        "Regular data of the master/slave alternated ADCs",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "ADC Common status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy_mst",
                    description: Some(
                        "Master ADC ready",
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
                    name: "eosmp_mst",
                    description: Some(
                        "End of Sampling phase flag of the master ADC",
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
                    name: "eoc_mst",
                    description: Some(
                        "End of regular conversion of the master ADC",
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
                    name: "eos_mst",
                    description: Some(
                        "End of regular sequence flag of the master ADC",
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
                    name: "ovr_mst",
                    description: Some(
                        "Overrun flag of the master ADC",
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
                    name: "jeoc_mst",
                    description: Some(
                        "End of injected conversion flag of the master ADC",
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
                    name: "jeos_mst",
                    description: Some(
                        "End of injected sequence flag of the master ADC",
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
                    name: "awd_mst",
                    description: Some(
                        "Analog watchdog flag of the master ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jqovf_mst",
                    description: Some(
                        "Injected Context Queue Overflow flag of the master ADC",
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
                    name: "adrdy_slv",
                    description: Some(
                        "Slave ADC ready",
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
                    name: "eosmp_slv",
                    description: Some(
                        "End of Sampling phase flag of the slave ADC",
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
                    name: "eoc_slv",
                    description: Some(
                        "End of regular conversion of the slave ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eos_slv",
                    description: Some(
                        "End of regular sequence flag of the slave ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr_slv",
                    description: Some(
                        "Overrun flag of the slave ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc_slv",
                    description: Some(
                        "End of injected conversion flag of the slave ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeos_slv",
                    description: Some(
                        "End of injected sequence flag of the slave ADC",
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
                    name: "awd_slv",
                    description: Some(
                        "Analog watchdog flag of the slave ADC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "jqovf_slv",
                    description: Some(
                        "Injected Context Queue Overflow flag of the slave ADC",
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
    ],
    enums: &[
        Enum {
            name: "Ckmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ASYNCHRONOUS",
                    description: Some(
                        "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYNC_DIV1",
                    description: Some(
                        "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SYNC_DIV2",
                    description: Some(
                        "Use AHB clock rcc_hclk3 divided by 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SYNC_DIV4",
                    description: Some(
                        "Use AHB clock rcc_hclk3 divided by 4",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Damdf",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NO_PACK",
                    description: Some(
                        "Without data packing, CDR/CDR2 not used",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FORMAT32TO10",
                    description: Some(
                        "CDR formatted for 32-bit down to 10-bit resolution",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FORMAT8",
                    description: Some(
                        "CDR formatted for 8-bit resolution",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dual",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some(
                        "Independent mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DUAL_RJ",
                    description: Some(
                        "Dual, combined regular simultaneous + injected simultaneous mode",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DUAL_RA",
                    description: Some(
                        "Dual, combined regular simultaneous + alternate trigger mode",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DUAL_IJ",
                    description: Some(
                        "Dual, combined interleaved mode + injected simultaneous mode",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DUAL_J",
                    description: Some(
                        "Dual, injected simultaneous mode only",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DUAL_R",
                    description: Some(
                        "Dual, regular simultaneous mode only",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DUAL_I",
                    description: Some(
                        "Dual, interleaved mode only",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DUAL_A",
                    description: Some(
                        "Dual, alternate trigger mode only",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Presc",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "adc_ker_ck_input not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "adc_ker_ck_input divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "adc_ker_ck_input divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "adc_ker_ck_input divided by 6",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "adc_ker_ck_input divided by 8",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "adc_ker_ck_input divided by 10",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "adc_ker_ck_input divided by 12",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "adc_ker_ck_input divided by 16",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "adc_ker_ck_input divided by 32",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "adc_ker_ck_input divided by 64",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "adc_ker_ck_input divided by 128",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "adc_ker_ck_input divided by 256",
                    ),
                    value: 11,
                },
            ],
        },
    ],
};
                