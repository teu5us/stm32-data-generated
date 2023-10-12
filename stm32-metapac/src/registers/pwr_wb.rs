
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Pwr",
        extends: None,
        description: Some("Power control"),
        items: &[
            BlockItem {
                name: "cr1",
                description: Some("Power control register 1"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr1"),
                }),
            },
            BlockItem {
                name: "cr2",
                description: Some("Power control register 2"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr2"),
                }),
            },
            BlockItem {
                name: "cr3",
                description: Some("Power control register 3"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr3"),
                }),
            },
            BlockItem {
                name: "cr4",
                description: Some("Power control register 4"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr4"),
                }),
            },
            BlockItem {
                name: "sr1",
                description: Some("Power status register 1"),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Sr1"),
                }),
            },
            BlockItem {
                name: "sr2",
                description: Some("Power status register 2"),
                array: None,
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Sr2"),
                }),
            },
            BlockItem {
                name: "scr",
                description: Some("Power status clear register"),
                array: None,
                byte_offset: 24,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Scr"),
                }),
            },
            BlockItem {
                name: "cr5",
                description: Some("Power control register 5"),
                array: None,
                byte_offset: 28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr5"),
                }),
            },
            BlockItem {
                name: "pucra",
                description: Some("Power Port A pull-up control register"),
                array: None,
                byte_offset: 32,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pdcra",
                description: Some("Power Port A pull-down control register"),
                array: None,
                byte_offset: 36,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pucrb",
                description: Some("Power Port B pull-up control register"),
                array: None,
                byte_offset: 40,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pdcrb",
                description: Some("Power Port B pull-down control register"),
                array: None,
                byte_offset: 44,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pucrc",
                description: Some("Power Port C pull-up control register"),
                array: None,
                byte_offset: 48,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pdcrc",
                description: Some("Power Port C pull-down control register"),
                array: None,
                byte_offset: 52,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pucrd",
                description: Some("Power Port D pull-up control register"),
                array: None,
                byte_offset: 56,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pdcrd",
                description: Some("Power Port D pull-down control register"),
                array: None,
                byte_offset: 60,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pucre",
                description: Some("Power Port E pull-up control register"),
                array: None,
                byte_offset: 64,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pdcre",
                description: Some("Power Port E pull-down control register"),
                array: None,
                byte_offset: 68,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pucrh",
                description: Some("Power Port H pull-up control register"),
                array: None,
                byte_offset: 88,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "pdcrh",
                description: Some("Power Port H pull-down control register"),
                array: None,
                byte_offset: 92,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Pxcr"),
                }),
            },
            BlockItem {
                name: "c2cr1",
                description: Some("CPU2 Power control register 1"),
                array: None,
                byte_offset: 128,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("C2cr1"),
                }),
            },
            BlockItem {
                name: "c2cr3",
                description: Some("CPU2 Power control register 3"),
                array: None,
                byte_offset: 132,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("C2cr3"),
                }),
            },
            BlockItem {
                name: "extscr",
                description: Some("Power status clear register"),
                array: None,
                byte_offset: 136,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Extscr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Sr2",
            extends: None,
            description: Some("Power status register 2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdbf",
                    description: Some("Step Down converter Bypass mode flag"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdsmpsf",
                    description: Some("Step Down converter SMPS mode flag"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reglps",
                    description: Some("Low-power regulator started"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reglpf",
                    description: Some("Low-power regulator flag"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vosf",
                    description: Some("Voltage scaling flag"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdo",
                    description: Some("Power voltage detector output"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvmo1",
                    description: Some("Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvmo3",
                    description: Some("Peripheral voltage monitoring output: VDDA vs. 1.62 V"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some("Power control register 1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some("Low-power mode selection for CPU1"),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpdr",
                    description: Some("Flash power down mode during LPRun for CPU1"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpds",
                    description: Some("Flash power down mode during LPsSleep for CPU1"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbp",
                    description: Some("Disable backup domain write protection"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpr",
                    description: Some("Low-power run"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr1",
            extends: None,
            description: Some("Power status register 1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some("Wakeup flag 1"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 5, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "sdfbf",
                    description: Some("Step Down converter forced in Bypass interrupt flag"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borhf",
                    description: Some("BORH interrupt flag"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blewuf",
                    description: Some("BLE wakeup interrupt flag"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "_802wuf",
                    description: Some("802.15.4 wakeup interrupt flag"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crpef",
                    description: Some("Enable critical radio phase end of activity interrupt flag"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bleaf",
                    description: Some("BLE end of activity interrupt flag"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "af802",
                    description: Some("802.15.4 end of activity interrupt flag"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2hf",
                    description: Some("CPU2 Hold interrupt flag"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wufi",
                    description: Some("Internal Wakeup interrupt flag"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr4",
            extends: None,
            description: Some("Power control register 4"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp1",
                    description: Some("Wakeup pin WKUP1 polarity"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 5, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "vbe",
                    description: Some("VBAT battery charging enable"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbrs",
                    description: Some("VBAT battery charging resistor selection"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2boot",
                    description: Some("BOOT CPU2 after reset or wakeup from Stop or Standby modes"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2cr3",
            extends: None,
            description: Some("CPU2 Power control register 3"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ewup",
                    description: Some("Enable Wakeup pin"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 5, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "eblewup",
                    description: Some("Enable BLE host wakeup interrupt for CPU2"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "e802wup",
                    description: Some("Enable 802.15.4 host wakeup interrupt for CPU2"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apc",
                    description: Some("Apply pull-up and pull-down configuration for CPU2"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eiwul",
                    description: Some("Enable internal wakeup line for CPU2"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some("Power control register 3"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ewup",
                    description: Some("Enable Wakeup pin"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 5, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "eborhsdfb",
                    description: Some("Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrs",
                    description: Some("SRAM2a retention in Standby mode"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apc",
                    description: Some("Apply pull-up and pull-down configuration"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eblea",
                    description: Some("Enable BLE end of activity interrupt for CPU1"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ecrpe",
                    description: Some("Enable critical radio phase end of activity interrupt for CPU1"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "e802a",
                    description: Some("Enable end of activity interrupt for CPU1"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ec2h",
                    description: Some("Enable CPU2 Hold interrupt for CPU1"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eiwul",
                    description: Some("Enable internal wakeup line for CPU1"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr5",
            extends: None,
            description: Some("Power control register 5"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdvos",
                    description: Some("Step Down converter voltage output scaling"),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdsc",
                    description: Some("Step Down converter supplt startup current selection"),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borhc",
                    description: Some("BORH configuration selection"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smpscfg",
                    description: Some("VOS configuration selection (non user)"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdben",
                    description: Some("Enable Step Down converter Bypass mode enabled"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdeb",
                    description: Some("Enable Step Down converter SMPS mode enabled"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "C2cr1",
            extends: None,
            description: Some("CPU2 Power control register 1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some("Low-power mode selection for CPU2"),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpdr",
                    description: Some("Flash power down mode during LPRun for CPU2"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpds",
                    description: Some("Flash power down mode during LPSleep for CPU2"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bleewkup",
                    description: Some("BLE external wakeup signal"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "_802ewkup",
                    description: Some("802.15.4 external wakeup signal"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pxcr",
            extends: None,
            description: Some("Power Port pull-up/down control register"),
            bit_size: 32,
            fields: &[Field {
                name: "pd",
                description: Some("Port A pull-up/down bit y (y=0..15)"),
                bit_offset: 0,
                bit_size: 1,
                array: Some(Array::Regular(RegularArray { len: 16, stride: 1 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some("Power control register 2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some("Power voltage detector enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pls",
                    description: Some("Power voltage detector level selection"),
                    bit_offset: 1,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extscr",
            extends: None,
            description: Some("Power status clear register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c1cssf",
                    description: Some("Clear CPU1 Stop Standby flags"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2cssf",
                    description: Some("Clear CPU2 Stop Standby flags"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccrpf",
                    description: Some("Clear Critical Radio system phase"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c1sbf",
                    description: Some("System Standby flag for CPU1"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c1stopf",
                    description: Some("System Stop flag for CPU1"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2sbf",
                    description: Some("System Standby flag for CPU2"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2stopf",
                    description: Some("System Stop flag for CPU2"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crpf",
                    description: Some("Critical Radio system phase"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c1ds",
                    description: Some("CPU1 deepsleep mode"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2ds",
                    description: Some("CPU2 deepsleep mode"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Scr",
            extends: None,
            description: Some("Power status clear register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some("Clear wakeup flag 1"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(Array::Regular(RegularArray { len: 5, stride: 1 })),
                    enumm: None,
                },
                Field {
                    name: "csmpsfbf",
                    description: Some("Clear SMPS Step Down converter forced in Bypass interrupt flag"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cborhf",
                    description: Some("Clear BORH interrupt flag"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cblewuf",
                    description: Some("Clear BLE wakeup interrupt flag"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c802wuf",
                    description: Some("Clear 802.15.4 wakeup interrupt flag"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ccrpef",
                    description: Some("Clear critical radio phase end of activity interrupt flag"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cbleaf",
                    description: Some("Clear BLE end of activity interrupt flag"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c802af",
                    description: Some("Clear 802.15.4 end of activity interrupt flag"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cc2hf",
                    description: Some("Clear CPU2 Hold interrupt flag"),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
