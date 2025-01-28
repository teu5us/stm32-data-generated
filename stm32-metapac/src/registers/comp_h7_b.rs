
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Comp",
            extends: None,
            description: Some(
                "COMP1.",
            ),
            items: &[
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Comparator status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icfr",
                    description: Some(
                        "Comparator interrupt clear flag register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Icfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "or",
                    description: Some(
                        "Comparator option register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Or",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "Comparator configuration register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "Comparator configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "Comparator configuration register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "COMP channel 1 enable bit.",
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
                    name: "brgen",
                    description: Some(
                        "Scaler bridge enable.",
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
                    name: "scalen",
                    description: Some(
                        "Voltage scaler enable bit.",
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
                    name: "polarity",
                    description: Some(
                        "COMP channel 1 polarity selection bit.",
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
                    name: "iten",
                    description: Some(
                        "COMP channel 1 interrupt enable.",
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
                    name: "hyst",
                    description: Some(
                        "COMP channel 1 hysteresis selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hyst",
                    ),
                },
                Field {
                    name: "pwrmode",
                    description: Some(
                        "Power Mode of the COMP channel 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pwrmode",
                    ),
                },
                Field {
                    name: "inmsel",
                    description: Some(
                        "COMP channel 1 inverting input selection field.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Inmsel",
                    ),
                },
                Field {
                    name: "inpsel",
                    description: Some(
                        "COMP channel 1 non-inverting input selection bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Inpsel",
                    ),
                },
                Field {
                    name: "blanking",
                    description: Some(
                        "COMP channel 1 blanking source selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Blanking",
                    ),
                },
                Field {
                    name: "lock",
                    description: Some(
                        "Lock bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: Some(
                "CFGR1",
            ),
            description: Some(
                "Comparator configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "COMP channel 1 enable bit.",
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
                    name: "brgen",
                    description: Some(
                        "Scaler bridge enable.",
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
                    name: "scalen",
                    description: Some(
                        "Voltage scaler enable bit.",
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
                    name: "polarity",
                    description: Some(
                        "COMP channel 1 polarity selection bit.",
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
                    name: "winmode",
                    description: Some(
                        "Window comparator mode selection bit.",
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
                    name: "iten",
                    description: Some(
                        "COMP channel 1 interrupt enable.",
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
                    name: "hyst",
                    description: Some(
                        "COMP channel 1 hysteresis selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hyst",
                    ),
                },
                Field {
                    name: "pwrmode",
                    description: Some(
                        "Power Mode of the COMP channel 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pwrmode",
                    ),
                },
                Field {
                    name: "inmsel",
                    description: Some(
                        "COMP channel 1 inverting input selection field.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Inmsel",
                    ),
                },
                Field {
                    name: "inpsel",
                    description: Some(
                        "COMP channel 1 non-inverting input selection bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Inpsel",
                    ),
                },
                Field {
                    name: "blanking",
                    description: Some(
                        "COMP channel 1 blanking source selection bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Blanking",
                    ),
                },
                Field {
                    name: "lock",
                    description: Some(
                        "Lock bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icfr",
            extends: None,
            description: Some(
                "Comparator interrupt clear flag register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccif",
                    description: Some(
                        "Clear COMP channel 1 Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Or",
            extends: None,
            description: Some(
                "Comparator option register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afop",
                    description: Some(
                        "Selection of source for alternate function of output ports.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Comparator status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cval",
                    description: Some(
                        "COMP channel 1 output status bit.",
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
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cif",
                    description: Some(
                        "COMP channel 1 Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
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
            name: "Blanking",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "NO_BLANKING",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1OC5",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2OC3",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "TIM3OC3",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "TIM3OC4",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "TIM8OC5",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "TIM15OC1",
                    description: None,
                    value: 6,
                },
            ],
        },
        Enum {
            name: "Hyst",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "LOW",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Inmsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "VREF_1OVER4",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "VREF_1OVER2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "VREF_3OVER4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "VREF",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "INM1",
                    description: None,
                    value: 4,
                },
                EnumVariant {
                    name: "INM2",
                    description: None,
                    value: 5,
                },
                EnumVariant {
                    name: "COMPX_INM1",
                    description: None,
                    value: 6,
                },
                EnumVariant {
                    name: "COMPX_INM2",
                    description: None,
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Inpsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "INP1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "INP2",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pwrmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High speed / full power",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUM",
                    description: Some(
                        "Medium speed / medium power",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUM_EITHER",
                    description: Some(
                        "Medium speed / medium power",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Ultra low power / ultra-low-power",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
                