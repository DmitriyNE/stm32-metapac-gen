
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc1",
            extends: None,
            description: Some(
                "Analog to Digital Converter ADC1",
            ),
            items: &[
                BlockItem {
                    name: "isr",
                    description: Some(
                        "interrupt and status register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 16,
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
                BlockItem {
                    name: "smpr",
                    description: Some(
                        "sample time register 1-2",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcsel",
                    description: Some(
                        "channel preselection register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcsel",
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
                    byte_offset: 48,
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
                    byte_offset: 52,
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
                    byte_offset: 56,
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
                    byte_offset: 60,
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
                    name: "dr",
                    description: Some(
                        "regular Data Register",
                    ),
                    array: None,
                    byte_offset: 64,
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
                    name: "jsqr",
                    description: Some(
                        "injected sequence register",
                    ),
                    array: None,
                    byte_offset: 76,
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
                    name: "ofr",
                    description: Some(
                        "offset number 1-4 register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcomp",
                    description: Some(
                        "gain compensation register",
                    ),
                    array: None,
                    byte_offset: 112,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcomp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jdr",
                    description: Some(
                        "injected data register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Jdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd2cr",
                    description: Some(
                        "analog watchdog 2 configuration register",
                    ),
                    array: None,
                    byte_offset: 160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd2cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awd3cr",
                    description: Some(
                        "analog watchdog 3 configuration register",
                    ),
                    array: None,
                    byte_offset: 164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awd3cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr1",
                    description: Some(
                        "watchdog threshold register 1",
                    ),
                    array: None,
                    byte_offset: 168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr1",
                    description: Some(
                        "watchdog threshold register 1",
                    ),
                    array: None,
                    byte_offset: 172,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr2",
                    description: Some(
                        "watchdog lower threshold register 2",
                    ),
                    array: None,
                    byte_offset: 176,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr2",
                    description: Some(
                        "watchdog higher threshold register 2",
                    ),
                    array: None,
                    byte_offset: 180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ltr3",
                    description: Some(
                        "watchdog lower threshold register 3",
                    ),
                    array: None,
                    byte_offset: 184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ltr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "htr3",
                    description: Some(
                        "watchdog higher threshold register 3",
                    ),
                    array: None,
                    byte_offset: 188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Htr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "difsel",
                    description: Some(
                        "differential mode selection register",
                    ),
                    array: None,
                    byte_offset: 192,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Difsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfact",
                    description: Some(
                        "calibration factors register",
                    ),
                    array: None,
                    byte_offset: 196,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calfact",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calfact2",
                    description: Some(
                        "calibration factor register",
                    ),
                    array: None,
                    byte_offset: 200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calfact2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Awd2cr",
            extends: None,
            description: Some(
                "analog watchdog 2 configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd2ch",
                    description: Some(
                        "Analog watchdog 2 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 2. AWD2CH[i] = 0: ADC analog input channel-i is not monitored by AWD2 AWD2CH[i] = 1: ADC analog input channel-i is monitored by AWD2 When AWD2CH[19:0] = 000..0, the analog Watchdog 2 is disabled Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers. Software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awd3cr",
            extends: None,
            description: Some(
                "analog watchdog 3 configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd3ch",
                    description: Some(
                        "Analog watchdog 3 channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by the analog watchdog 3. AWD3CH[i] = 0: ADC analog input channel-i is not monitored by AWD3 AWD3CH[i] = 1: ADC analog input channel-i is monitored by AWD3 When AWD3CH[19:0] = 000..0, the analog Watchdog 3 is disabled Note: The channels selected by AWD3CH must be also selected into the SQRi or JSQRi registers. The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calfact",
            extends: None,
            description: Some(
                "user control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i_apb_addr",
                    description: Some(
                        "Delayed write access address This bitfield contains the address that is being written during delayed write accesses.",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i_apb_data",
                    description: Some(
                        "Delayed write access data This bitfield contains the data that are being written during delayed write accesses.",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "validity",
                    description: Some(
                        "Delayed write access status bit This bit indicates the communication status between the ADC digital and analog blocks.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "latch_coef",
                    description: Some(
                        "Calibration factor latch enable bit This bit latches the calibration factor in the CALFACT[31:0] bits.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "capture_coef",
                    description: Some(
                        "Calibration factor capture enable bit This bit enables the internal calibration factor capture.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Calfact2",
            extends: None,
            description: Some(
                "calibration factor register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calfact",
                    description: Some(
                        "Linearity or offset calibration factor These bits can be written either by hardware or by software. They contain the 32-bit offset or linearity calibration factor. When CAPTURE_COEF is set to 1, the calibration factor of the analog block is read back and stored in CALFACT[31:0], indexed by CALINDEX[3:0] bits. When LATCH_COEF is set to 1, the calibration factor of the analog block is updated with the value programmed in CALFACT[31:0], indexed by CALINDEX[3:0] bits. To read all calibration factors, perform nine accesses to the CALFACT2 register. To write all calibration factors, perform eight accesses to the CALFACT2 register. Note: The software is allowed to write these bits only when ADEN\u{a0}=\u{a0}1, ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (ADC is enabled and no calibration is ongoing and no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmngt",
                    description: Some(
                        "Data management configuration This bit is set and cleared by software to select how the ADC interface output data are managed. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Dmngt",
                    ),
                },
                Field {
                    name: "res",
                    description: Some(
                        "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Res",
                    ),
                },
                Field {
                    name: "extsel",
                    description: Some(
                        "External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Refer to the ADC external trigger for regular channels in signals for details on trigger mapping. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 5,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exten",
                    description: Some(
                        "External trigger enable and polarity selection for regular channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of a regular group. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Exten",
                    ),
                },
                Field {
                    name: "ovrmod",
                    description: Some(
                        "Overrun Mode This bit is set and cleared by software and configure the way data overrun is managed. Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ovrmod",
                    ),
                },
                Field {
                    name: "cont",
                    description: Some(
                        "Single / continuous conversion mode for regular conversions This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN\u{a0}=\u{a0}1 and CONT\u{a0}=\u{a0}1. The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "autdly",
                    description: Some(
                        "Delayed conversion mode This bit is set and cleared by software to enable/disable the Auto Delayed Conversion mode.. Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discen",
                    description: Some(
                        "Discontinuous mode for regular channels This bit is set and cleared by software to enable/disable Discontinuous mode for regular channels. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both DISCEN\u{a0}=\u{a0}1 and CONT\u{a0}=\u{a0}1. It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set. The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discnum",
                    description: Some(
                        "Discontinuous mode channel count These bits are written by software to define the number of regular channels to be converted in discontinuous mode, after receiving an external trigger. ... Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 17,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdiscen",
                    description: Some(
                        "Discontinuous mode on injected channels This bit is set and cleared by software to enable/disable discontinuous mode on the injected channels of a group. Note: The software is allowed to write this bit only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing). It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1sgl",
                    description: Some(
                        "Enable the watchdog 1 on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWD1CH[4:0] bits or on all the channels Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Awd1sgl",
                    ),
                },
                Field {
                    name: "awd1en",
                    description: Some(
                        "Analog watchdog 1 enable on regular channels This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jawd1en",
                    description: Some(
                        "Analog watchdog 1 enable on injected channels This bit is set and cleared by software Note: The software is allowed to write this bit only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing).",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jauto",
                    description: Some(
                        "Automatic injected group conversion This bit is set and cleared by software to enable/disable automatic injected group conversion after regular group conversion. Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no regular nor injected conversion is ongoing).",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1ch",
                    description: Some(
                        "Analog watchdog 1 channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved, must not be used Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers. Software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 26,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rovse",
                    description: Some(
                        "Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jovse",
                    description: Some(
                        "Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovss",
                    description: Some(
                        "Oversampling right shift This bit field is set and cleared by software to define the right shifting applied to the raw oversampling result. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 5,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trovs",
                    description: Some(
                        "Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Trovs",
                    ),
                },
                Field {
                    name: "rovsm",
                    description: Some(
                        "Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rovsm",
                    ),
                },
                Field {
                    name: "bulb",
                    description: Some(
                        "Bulb sampling mode This bit is set and cleared by software to select the bulb sampling mode. SMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swtrig",
                    description: Some(
                        "Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smptrig",
                    description: Some(
                        "Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN[1:0] bits must be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN[1:0] bits is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osr",
                    description: Some(
                        "Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. 2: 3x ... 1023: 1024x Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 16,
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lftrig",
                    description: Some(
                        "Low-frequency trigger This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lshift",
                    description: Some(
                        "Left shift factor This bitfield is set and cleared by software to define the left shifting applied to the final result with or without oversampling. Note: The software is allowed to write this bit only when ADSTART\u{a0}= 0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 28,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aden",
                    description: Some(
                        "ADC enable control This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag ADRDY has been set. It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command. Note: The software is allowed to set ADEN only when all bits of CR registers are 0 (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0) except for bit ADVREGEN which must be 1 (and the software must have wait for the startup time of the voltage regulator).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addis",
                    description: Some(
                        "ADC disable command This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state (OFF state). It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at this time). Note: The software is allowed to set ADDIS only when ADEN\u{a0}=\u{a0}1 and both ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adstart",
                    description: Some(
                        "ADC start of regular conversion This bit is set by software to start ADC conversion of regular channels. Depending on the configuration bits EXTEN[1:0], a conversion starts immediately (software trigger configuration) or once a regular hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware:. in Single conversion mode (CONT\u{a0}=\u{a0}0, DISCEN\u{a0}=\u{a0}0) when software trigger is selected (EXTEN[1:0]\u{a0}=\u{a0}0x0): at the assertion of the end of regular conversion sequence (EOS) flag. In Discontinuous conversion mode (CONT\u{a0}=\u{a0}0, DISCEN\u{a0}=\u{a0}1), when the software trigger is selected (EXTEN[1:0]\u{a0}=\u{a0}0x0): at the end of conversion (EOC) flag. in all other cases: after the execution of the ADSTP command, at the same time that ADSTP is cleared by hardware. Note: The software is allowed to set ADSTART only when ADEN\u{a0}=\u{a0}1 and ADDIS\u{a0}=\u{a0}0 (ADC is enabled and there is no pending request to disable the ADC) In Auto-injection mode (JAUTO\u{a0}=\u{a0}1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared).",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jadstart",
                    description: Some(
                        "ADC start of injected conversion This bit is set by software to start ADC conversion of injected channels. Depending on the configuration bits JEXTEN[1:0], a conversion starts immediately (software trigger configuration) or once an injected hardware trigger event occurs (hardware trigger configuration). It is cleared by hardware: in Single conversion mode when software trigger is selected (JEXTSEL\u{a0}=\u{a0}0x0): at the assertion of the end of injected conversion sequence (JEOS) flag. in all cases: after the execution of the JADSTP command, at the same time as JADSTP is cleared by hardware. Note: The software is allowed to set JADSTART only when ADEN\u{a0}=\u{a0}1 and ADDIS\u{a0}=\u{a0}0 (ADC is enabled and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO\u{a0}=\u{a0}1), regular and auto-injected conversions are started by setting bit ADSTART (JADSTART must be kept cleared).",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adstp",
                    description: Some(
                        "ADC stop of regular conversion command This bit is set by software to stop and discard an ongoing regular conversion (ADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC regular sequence and triggers can be re-configured. The ADC is then ready to accept a new start of regular conversions (ADSTART command). Note: The software is allowed to set ADSTP only when ADSTART\u{a0}=\u{a0}1 and ADDIS\u{a0}=\u{a0}0 (ADC is enabled and eventually converting a regular conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO\u{a0}=\u{a0}1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adstp",
                    ),
                },
                Field {
                    name: "jadstp",
                    description: Some(
                        "ADC stop of injected conversion command This bit is set by software to stop and discard an ongoing injected conversion (JADSTP Command). It is cleared by hardware when the conversion is effectively discarded and the ADC injected sequence and triggers can be re-configured. The ADC is then ready to accept a new start of injected conversions (JADSTART command). Note: The software is allowed to set JADSTP only when JADSTART\u{a0}=\u{a0}1 and ADDIS\u{a0}=\u{a0}0 (ADC is enabled and eventually converting an injected conversion and there is no pending request to disable the ADC). In Auto-injection mode (JAUTO\u{a0}=\u{a0}1), setting ADSTP bit aborts both regular and injected conversions (do not use JADSTP).",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adstp",
                    ),
                },
                Field {
                    name: "adcallin",
                    description: Some(
                        "Linearity calibration This bit is set and cleared by software to enable the linearity calibration. Note: The software is allowed to write this bit only when the ADC is disabled and is not calibrating (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, JADSTP\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calindex",
                    description: Some(
                        "Calibration factor This bitfield controls the calibration factor to be read or written. Calibration index 0 is dedicated to single-ended and differential offsets, calibration index 1 to 7 to the linearity calibration factors, and index 8 to the internal offset: Others: Reserved, must not be used Note: CALFACT2[31:0] correspond to the location of CALINDEX[3:0] calibration factor data (see for details).",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "advregen",
                    description: Some(
                        "voltage regulator enable This bits is set by software to enable the ADC voltage regulator. Before performing any operation such as launching a calibration or enabling the ADC, the ADC voltage regulator must first be enabled and the software must wait for the regulator start-up time. For more details about the ADC voltage regulator enable and disable sequences, refer to (ADVREGEN). The software can program this bit field only when the ADC is disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "deeppwd",
                    description: Some(
                        "Deep-power-down enable This bit is set and cleared by software to put the ADC in Deep-power-down mode. Note: The software is allowed to write this bit only when the ADC is disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, JADSTP\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adcal",
                    description: Some(
                        "ADC calibration This bit is set by software to start the ADC calibration. It is cleared by hardware after calibration is complete. Note: The software is allowed to launch a calibration by setting ADCAL only when ADEN\u{a0}=\u{a0}0.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Difsel",
            extends: None,
            description: Some(
                "differential mode selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "difsel",
                    description: Some(
                        "Differential mode for channels 19 to 0 These bits are set and cleared by software. They allow selecting if a channel is configured as single ended or differential mode. DIFSEL[i] = 0: ADC analog input channel-i is configured in single ended mode DIFSEL[i] = 1: ADC analog input channel-i is configured in differential mode Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL\u{a0}=\u{a0}0, JADSTART\u{a0}=\u{a0}0, JADSTP\u{a0}=\u{a0}0, ADSTART\u{a0}=\u{a0}0, ADSTP\u{a0}=\u{a0}0, ADDIS\u{a0}=\u{a0}0 and ADEN\u{a0}=\u{a0}0).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Difsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "regular Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata",
                    description: Some(
                        "Regular data converted These bits are read-only. They contain the conversion result from the last converted regular channel. The data are left- or right-aligned as described in.",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gcomp",
            extends: None,
            description: Some(
                "gain compensation register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gcompcoeff",
                    description: Some(
                        "Gain compensation coefficient These bits are set and cleared by software to program the gain compensation coefficient. ... ... The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756. Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is 1.",
                    ),
                    bit_offset: 0,
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gcomp",
                    description: Some(
                        "Gain compensation mode This bit is set and cleared by software to enable the gain compensation mode. Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr1",
            extends: None,
            description: Some(
                "watchdog threshold register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr1",
                    description: Some(
                        "Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).",
                    ),
                    bit_offset: 0,
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdfilt1",
                    description: Some(
                        "Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 29,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr2",
            extends: None,
            description: Some(
                "watchdog higher threshold register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr2",
                    description: Some(
                        "Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).",
                    ),
                    bit_offset: 0,
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Htr3",
            extends: None,
            description: Some(
                "watchdog higher threshold register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htr3",
                    description: Some(
                        "Analog watchdog 3 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).",
                    ),
                    bit_offset: 0,
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdyie",
                    description: Some(
                        "ADC ready interrupt enable This bit is set and cleared by software to enable/disable the ADC Ready interrupt. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmpie",
                    description: Some(
                        "End of sampling flag interrupt enable for regular conversions This bit is set and cleared by software to enable/disable the end of the sampling phase interrupt for regular conversions. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocie",
                    description: Some(
                        "End of regular conversion interrupt enable This bit is set and cleared by software to enable/disable the end of a regular conversion interrupt. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosie",
                    description: Some(
                        "End of regular sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of regular sequence of conversions interrupt. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovrie",
                    description: Some(
                        "Overrun interrupt enable This bit is set and cleared by software to enable/disable the Overrun interrupt of a regular conversion. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeocie",
                    description: Some(
                        "End of injected conversion interrupt enable This bit is set and cleared by software to enable/disable the end of an injected conversion interrupt. Note: Software is allowed to write this bit only when JADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeosie",
                    description: Some(
                        "End of injected sequence of conversions interrupt enable This bit is set and cleared by software to enable/disable the end of injected sequence of conversions interrupt. Note: Software is allowed to write this bit only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing).",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1ie",
                    description: Some(
                        "Analog watchdog 1 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 1 interrupt. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd2ie",
                    description: Some(
                        "Analog watchdog 2 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd3ie",
                    description: Some(
                        "Analog watchdog 3 interrupt enable This bit is set and cleared by software to enable/disable the analog watchdog 2 interrupt. Note: Software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "interrupt and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy",
                    description: Some(
                        "ADC ready This bit is set by hardware after the ADC has been enabled (bit ADEN\u{a0}=\u{a0}1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmp",
                    description: Some(
                        "End of sampling flag This bit is set by hardware during the conversion of any channel (only for regular channels), at the end of the sampling phase.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc",
                    description: Some(
                        "End of conversion flag This bit is set by hardware at the end of each regular conversion of a channel when a new data is available in the DR register. It is cleared by software writing 1 to it or by reading the DR register.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eos",
                    description: Some(
                        "End of regular sequence flag This bit is set by hardware at the end of the conversions of a regular sequence of channels. It is cleared by software writing 1 to it.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovr",
                    description: Some(
                        "ADC overrun This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to it.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc",
                    description: Some(
                        "Injected channel end of conversion flag This bit is set by hardware at the end of each injected conversion of a channel when a new data is available in the corresponding JDRy register. It is cleared by software writing 1 to it or by reading the corresponding JDRy register.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeos",
                    description: Some(
                        "Injected channel end of sequence flag This bit is set by hardware at the end of the conversions of all injected channels in the group. It is cleared by software writing 1 to it.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd1",
                    description: Some(
                        "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT1[11:0] and HT1[11:0] of LTR1, & HTR1 register. It is cleared by software. writing 1 to it.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd2",
                    description: Some(
                        "Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT2[7:0] and HT2[7:0] of LTR2 & HTR2 register. It is cleared by software writing 1 to it.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awd3",
                    description: Some(
                        "Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in the fields LT3[7:0] and HT3[7:0] of LTR3 & HTR3 register. It is cleared by software writing 1 to it.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ldordy",
                    description: Some(
                        "ADC voltage regulator ready This bit is set by hardware. It indicates that the ADC internal supply is ready. The ADC is available after tADCVREG_SETUP time.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jdr",
            extends: None,
            description: Some(
                "ADC injected data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jdata",
                    description: Some(
                        "Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in.",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jsqr",
            extends: None,
            description: Some(
                "ADC injected sequence register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jl",
                    description: Some(
                        "Injected channel sequence length These bits are written by software to define the total number of conversions in the injected channel conversion sequence. Note: The software is allowed to write these bits only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing.",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jextsel",
                    description: Some(
                        "External trigger selection for injected group These bits select the external event used to trigger the start of conversion of an injected group: ... Refer to the ADC external trigger for injected channels in internal signals for details on trigger mapping. Note: The software is allowed to write these bits only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing.",
                    ),
                    bit_offset: 2,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jexten",
                    description: Some(
                        "External trigger enable and polarity selection for injected channels These bits are set and cleared by software to select the external trigger polarity and enable the trigger of an injected group. Note: The software is allowed to write these bits only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing.",
                    ),
                    bit_offset: 7,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Jexten",
                    ),
                },
                Field {
                    name: "jsq1",
                    description: Some(
                        "1st conversion in the injected sequence These bits are written by software with the channel number (0..19) assigned as the 1st in the injected conversion sequence. Note: The software is allowed to write these bits only when JADSTART\u{a0}=\u{a0}0 (which ensures that no injected conversion is ongoing.",
                    ),
                    bit_offset: 9,
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr1",
            extends: None,
            description: Some(
                "ADC watchdog threshold register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr1",
                    description: Some(
                        "Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).",
                    ),
                    bit_offset: 0,
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr2",
            extends: None,
            description: Some(
                "ADC watchdog lower threshold register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr2",
                    description: Some(
                        "Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).",
                    ),
                    bit_offset: 0,
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ltr3",
            extends: None,
            description: Some(
                "ADC watchdog lower threshold register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltr3",
                    description: Some(
                        "Analog watchdog 3 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 3. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).",
                    ),
                    bit_offset: 0,
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofr",
            extends: None,
            description: Some(
                "ADC offset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offset1",
                    description: Some(
                        "Data offset y for the channel programmed into OFFSETy_CH[4:0] bits These bits are written by software to define the offset y to be subtracted from the raw converted data when converting a channel (regular or injected). The channel to which the data offset y applies must be programmed to the OFFSETy_CH[4:0] bits. The conversion result can be read from in the DR (regular conversion) or from in the JDRyi registers (injected conversion). When OFFSETy[21:0] bitfield is reset, the offset compensation is disabled. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing). If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y value is considered for the subtraction. For example, if OFFSET1_CH[4:0]\u{a0}=\u{a0}4 and OFFSET2_CH[4:0]\u{a0}=\u{a0}4, this is OFFSET1[25:0] that is subtracted when converting channel 4.",
                    ),
                    bit_offset: 0,
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "posoff",
                    description: Some(
                        "offset sign This bit is set and cleared by software to enable the positive offset. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usat",
                    description: Some(
                        "Unsigned saturation enable This bit is written by software to enable or disable the unsigned saturation feature. Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ssate",
                    description: Some(
                        "Signed saturation enable This bit is written by software to enable or disable the Signed saturation feature. (see OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details). Note: The software is allowed to write this bit only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "offset1_ch",
                    description: Some(
                        "Channel selection for the data offset y These bits are written by software to define the channel to which the offset programmed into OFFSETy[25:0] bits applies. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing). If OFFSETy_EN bit is set, it is not allowed to select the same channel in different OFRy registers.",
                    ),
                    bit_offset: 27,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcsel",
            extends: None,
            description: Some(
                "ADC channel preselection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcsel",
                    description: Some(
                        "Channel i (VINP[i]) preselection These bits are written by software to preselect the input channel I/O instance to be converted. Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Pcsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Smpr",
            extends: None,
            description: Some(
                "ADC sample time register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp",
                    description: Some(
                        "Channel x sampling time selection (x = 0 to 9) These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART\u{a0}=\u{a0}0 (which ensures that no conversion is ongoing).",
                    ),
                    bit_offset: 0,
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
                "regular sequence register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "l",
                    description: Some(
                        "Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART\u{a0}=\u{a0}0 (which ensures that no regular conversion is ongoing).",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 1-4",
                    ),
                    bit_offset: 6,
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 6,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr2",
            extends: None,
            description: Some(
                "regular sequence register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 5-9",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
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
                "regular sequence register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 10-14",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 6,
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
                "regular sequence register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some(
                        "group regular sequencer rank 15-16",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 6,
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
            name: "Adstp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOP",
                    description: Some(
                        "Stop conversion of channel",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Awd1sgl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ALL",
                    description: Some(
                        "Analog watchdog 1 enabled on all channels",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "Analog watchdog 1 enabled on single channel selected in AWD1CH",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Difsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLEENDED",
                    description: Some(
                        "Input channel is configured in single-ended mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIFFERENTIAL",
                    description: Some(
                        "Input channel is configured in differential mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dmngt",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DR",
                    description: Some(
                        "Store output data in DR only",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DMA_ONESHOT",
                    description: Some(
                        "DMA One Shot Mode selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DFSDM",
                    description: Some(
                        "DFSDM mode selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DMA_CIRCULAR",
                    description: Some(
                        "DMA Circular Mode selected",
                    ),
                    value: 3,
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
                    name: "RISINGEDGE",
                    description: Some(
                        "Trigger detection on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "Trigger detection on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTHEDGES",
                    description: Some(
                        "Trigger detection on both the rising and falling edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Jexten",
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
                    name: "RISINGEDGE",
                    description: Some(
                        "Trigger detection on the rising edge",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "Trigger detection on the falling edge",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "BOTHEDGES",
                    description: Some(
                        "Trigger detection on both the rising and falling edges",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ovrmod",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PRESERVE",
                    description: Some(
                        "Preserve DR register when an overrun is detected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERWRITE",
                    description: Some(
                        "Overwrite DR register when an overrun is detected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTPRESELECTED",
                    description: Some(
                        "Input channel x is not pre-selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRESELECTED",
                    description: Some(
                        "Pre-select input channel x",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Res",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FOURTEENBIT",
                    description: Some(
                        "14-bit resolution",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TWELVEBIT",
                    description: Some(
                        "12-bit resolution",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TENBIT",
                    description: Some(
                        "10-bit resolution",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "EIGHTBIT",
                    description: Some(
                        "8-bit resolution",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Rovsm",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CONTINUED",
                    description: Some(
                        "Oversampling is temporary stopped and continued after injection sequence",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RESUMED",
                    description: Some(
                        "Oversampling is aborted and resumed from start after injection sequence",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SampleTime",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CYCLES5",
                    description: Some(
                        "5 clock cycles",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES6",
                    description: Some(
                        "6 clock cycles",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES12",
                    description: Some(
                        "12 clock cycles",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES20",
                    description: Some(
                        "20 clock cycles",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES36",
                    description: Some(
                        "36 clock cycles",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES68",
                    description: Some(
                        "68 clock cycles",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES391",
                    description: Some(
                        "391 clock cycles",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES814",
                    description: Some(
                        "814 clock cycles",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Trovs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "AUTOMATIC",
                    description: Some(
                        "All oversampled conversions for a channel are run following a trigger",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRIGGERED",
                    description: Some(
                        "Each oversampled conversion for a channel needs a new trigger",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                