//! SMBIOS Types
//!
//! Defines the types used in SMBIOS Records 
//!
//! ## License
//!
//! Copyright (c) Microsoft Corporation.
//!
//! SPDX-License-Identifier: Apache-2.0

use bitfield::bitfield;
extern crate alloc;

///
/// Processor Information
///
#[derive(Copy, Clone, Debug)]
pub enum ProcessorTypeData {
    ProcessorOther = 0x01,
    ProcessorUnknown,
    CentralProcessor,
    MathProcessor,
    DspProcessor,
    VideoProcessor,
}

///
/// Processor Information - Processor Family
///
#[derive(Copy, Clone, Debug)]
pub enum ProcessorFamilyData {
    Other = 0x01,
    Unknown,
    Processor8086,
    Processor80286,
    Intel386,
    Intel486,
    Processor8087,
    Processor80287,
    Processor80387,
    Processor80487,
    Pentium,
    PentiumPro,
    PentiumII,
    PentiumMMX,
    Celeron,
    PentiumIIXeon,
    PentiumIII,
    M1Family,
    M2Family,
    IntelCeleronM,
    IntelPentium4Ht,
    IntelProcessor,
    AmdDuron = 0x18,
    K5Family,
    K6Family,
    K6_2,
    K6_3,
    AmdAthlon,
    Amd29000,
    K6_2Plus,
    PowerPC,
    PowerPC601,
    PowerPC603,
    PowerPC603Plus,
    PowerPC604,
    PowerPC620,
    PowerPCx704,
    PowerPC750,
    IntelCoreDuo,
    IntelCoreDuoMobile,
    IntelCoreSoloMobile,
    IntelAtom,
    IntelCoreM,
    IntelCorem3,
    IntelCorem5,
    IntelCorem7,
    Alpha,
    Alpha21064,
    Alpha21066,
    Alpha21164,
    Alpha21164PC,
    Alpha21164a,
    Alpha21264,
    Alpha21364,
    AmdTurionIIUltraDualCoreMobileM,
    AmdTurionIIDualCoreMobileM,
    AmdAthlonIIDualCoreM,
    AmdOpteron6100Series,
    AmdOpteron4100Series,
    AmdOpteron6200Series,
    AmdOpteron4200Series,
    AmdFxSeries,
    MipsFamily,
    MipsR4000,
    MipsR4200,
    MipsR4400,
    MipsR4600,
    MipsR10000,
    AmdCSeries,
    AmdESeries,
    AmdASeries,
    AmdGSeries,
    AmdZSeries,
    AmdRSeries,
    AmdOpteron4300,
    AmdOpteron6300,
    AmdOpteron3300,
    AmdFireProSeries,
    Sparc,
    SuperSparc,
    MicroSparcII,
    MicroSparcIIep,
    UltraSparc,
    UltraSparcII,
    UltraSparcIii,
    UltraSparcIII,
    UltraSparcIIIi,
    Processor68040 = 0x60,
    Processor68xxx,
    Processor68000,
    Processor68010,
    Processor68020,
    Processor68030,
    AmdAthlonX4QuadCore,
    AmdOpteronX1000Series,
    AmdOpteronX2000Series,
    AmdOpteronASeries,
    AmdOpteronX3000Series,
    AmdZen,
    HobbitFamily = 0x70,
    CrusoeTM5000 = 0x78,
    CrusoeTM3000,
    EfficeonTM8000,
    Weitek = 0x80,
    Itanium = 0x82,
    AmdAthlon64,
    AmdOpteron,
    AmdSempron,
    AmdTurion64Mobile,
    DualCoreAmdOpteron,
    AmdAthlon64X2DualCore,
    AmdTurion64X2Mobile,
    QuadCoreAmdOpteron,
    ThirdGenerationAmdOpteron,
    AmdPhenomFxQuadCore,
    AmdPhenomX4QuadCore,
    AmdPhenomX2DualCore,
    AmdAthlonX2DualCore,
    Parisc,
    PaRisc8500,
    PaRisc8000,
    PaRisc7300LC,
    PaRisc7200,
    PaRisc7100LC,
    PaRisc7100,
    V30Family = 0xA0,
    QuadCoreIntelXeon3200Series,
    DualCoreIntelXeon3000Series,
    QuadCoreIntelXeon5300Series,
    DualCoreIntelXeon5100Series,
    DualCoreIntelXeon5000Series,
    DualCoreIntelXeonLV,
    DualCoreIntelXeonULV,
    DualCoreIntelXeon7100Series,
    QuadCoreIntelXeon5400Series,
    QuadCoreIntelXeon,
    DualCoreIntelXeon5200Series,
    DualCoreIntelXeon7200Series,
    QuadCoreIntelXeon7300Series,
    QuadCoreIntelXeon7400Series,
    MultiCoreIntelXeon7400Series,
    PentiumIIIXeon,
    PentiumIIISpeedStep,
    Pentium4,
    IntelXeon,
    As400,
    IntelXeonMP,
    AMDAthlonXP,
    AMDAthlonMP,
    IntelItanium2,
    IntelPentiumM,
    IntelCeleronD,
    IntelPentiumD,
    IntelPentiumEx,
    IntelCoreSolo,
    Reserved,
    IntelCore2,
    IntelCore2Solo,
    IntelCore2Extreme,
    IntelCore2Quad,
    IntelCore2ExtremeMobile,
    IntelCore2DuoMobile,
    IntelCore2SoloMobile,
    IntelCoreI7,
    DualCoreIntelCeleron,
    Ibm390,
    G4,
    G5,
    EsaG6,
    ZArchitecture,
    IntelCoreI5,
    IntelCoreI3,
    IntelCoreI9,
    IntelXeonD,
    ViaC7M = 0xD2,
    ViaC7D,
    ViaC7,
    ViaEden,
    MultiCoreIntelXeon,
    DualCoreIntelXeon3Series,
    QuadCoreIntelXeon3Series,
    ViaNano,
    DualCoreIntelXeon5Series,
    QuadCoreIntelXeon5Series,
    DualCoreIntelXeon7Series = 0xDD,
    QuadCoreIntelXeon7Series,
    MultiCoreIntelXeon7Series,
    MultiCoreIntelXeon3400Series,
    AmdOpteron3000Series = 0xE4,
    AmdSempronII,
    EmbeddedAmdOpteronQuadCore,
    AmdPhenomTripleCore,
    AmdTurionUltraDualCoreMobile,
    AmdTurionDualCoreMobile,
    AmdAthlonDualCore,
    AmdSempronSI,
    AmdPhenomII,
    AmdAthlonII,
    SixCoreAmdOpteron,
    AmdSempronM,
    I860 = 0xFA,
    I960,
    IndicatorFamily2 = 0xFE,
    Reserved1,
    ARMv7 = 0x0100,
    ARMv8,
    ARMv9,
    Sh3,
    Sh4,
    Arm = 0x0118,
    StrongARM,
    Processor6x86 = 0x012C,
    MediaGX,
    Mii,
    WinChip = 0x0140,
    Dsp = 0x015E,
    VideoProcessor = 0x01F4,
    RiscvRV32 = 0x0200,
    RiscVRV64,
    RiscVRV128,
    LoongArch = 0x0258,
    Loongson1,
    Loongson2,
    Loongson3,
    Loongson2K,
    Loongson3A,
    Loongson3B,
    Loongson3C,
    Loongson3D,
    Loongson3E,
    DualCoreLoongson2K,
    QuadCoreLoongson3A = 0x026C,
    MultiCoreLoongson3A,
    QuadCoreLoongson3B,
    MultiCoreLoongson3B,
    MultiCoreLoongson3C,
    MultiCoreLoongson3D,
    IntelCore3 = 0x0300,
    IntelCore5,
    IntelCore7,
    IntelCore9,
    IntelCoreUltra3,
    IntelCoreUltra5,
    IntelCoreUltra7,
    IntelCoreUltra9,
}

///
/// Processor Information - Processor Upgrade
///
#[derive(Copy, Clone, Debug)]
pub enum ProcessorUpgrade {
    Other = 0x01,
    Unknown,
    DaughterBoard,
    ZIFSocket,
    ReplaceablePiggyBack,
    None,
    LIFSocket,
    Slot1,
    Slot2,
    Pin370Socket,
    SlotA,
    SlotM,
    Socket423,
    SocketA, //  Socket 462
    Socket478,
    Socket754,
    Socket940,
    Socket939,
    SocketmPGA604,
    SocketLGA771,
    SocketLGA775,
    SocketS1,
    SocketAM2,
    SocketF1207,
    SocketLGA1366,
    SocketG34,
    SocketAM3,
    SocketC32,
    SocketLGA1156,
    SocketLGA1567,
    SocketPGA988A,
    SocketBGA1288,
    SocketrPGA988B,
    SocketBGA1023,
    SocketBGA1224,
    SocketLGA1155,
    SocketLGA1356,
    SocketLGA2011,
    SocketFS1,
    SocketFS2,
    SocketFM1,
    SocketFM2,
    SocketLGA2011_3,
    SocketLGA1356_3,
    SocketLGA1150,
    SocketBGA1168,
    SocketBGA1234,
    SocketBGA1364,
    SocketAM4,
    SocketLGA1151,
    SocketBGA1356,
    SocketBGA1440,
    SocketBGA1515,
    SocketLGA3647_1,
    SocketSP3,
    SocketSP3r2,
    SocketLGA2066,
    SocketBGA1392,
    SocketBGA1510,
    SocketBGA1528,
    SocketLGA4189,
    SocketLGA1200,
    SocketLGA4677,
    SocketLGA1700,
    SocketBGA1744,
    SocketBGA1781,
    SocketBGA1211,
    SocketBGA2422,
    SocketLGA1211,
    SocketLGA2422,
    SocketLGA5773,
    SocketBGA5773,
    SocketAM5,
    SocketSP5,
    SocketSP6,
    SocketBGA883,
    SocketBGA1190,
    SocketBGA4129,
    SocketLGA4710,
    SocketLGA7529,
    SocketBGA1964, //ANS: check for the rest of list, not on 3.7
    SocketBGA1792,
    SocketBGA2049,
    SocketBGA2551,
    SocketLGA1851,
    SocketBGA2114,
    SocketBGA2833,
    // Use this when no other valid enumeration is available.
    // When this enumeration is used, Socket Type at offset 32h must be non-null.
    NotAvailable = 0xFF,
}

bitfield! {
    ///
    /// Processor Information - Voltage
    ///
    pub struct ProcessorVoltage (u8);
    impl Debug;
    pub processor_voltage_capability_5v, set_processor_voltage_capability_5v: 0;
    pub processor_voltage_capability_3_3v, set_processor_voltage_capability_3_3v: 1;
    pub processor_voltage_capability_2_9v, set_processor_voltage_capability_2_9v: 2;
    pub processor_voltage_capability_reserved, set_processor_voltage_capability_reserved: 3;
    pub processor_voltage_reserved, set_processor_voltage_reserved: 6, 4;
    pub processor_voltage_indicate_legacy, set_processor_voltage_indicate_legacy: 7;
}

bitfield! {
    ///
    /// Processor Information - Status
    ///
    pub struct ProcessorInformationStatus (u8);
    impl Debug;
    pub cpu_status, set_cpu_status: 2, 0;
    // reserved must be zero
    pub reserved, set_reserved: 5, 3;
    pub cpu_socket_populated, set_cpu_socket_populated: 6;
    // reserved2 must be zero
    pub reserved2, set_reserved2: 7;
}

bitfield! {
    ///
    /// Processor Information - Status
    ///
    pub struct ProcessorCharacteristics (u16);
    impl Debug;
    pub reserved, set_reserved : 0;
    pub unknown, set_unknown : 1;
    pub _64bit_capable, set_64bit_capable : 2;
    pub multi_core, set_multi_core : 3;
    pub hardware_thread, set_hardware_thread : 4;
    pub execute_protection, set_execute_protection : 5;
    pub enhanced_virtualization, set_enhanced_virtualization : 6;
    pub performance_control, set_performance_control : 7;
    pub _128bit_capable, set_128bit_capable : 8;
    pub arm64_soc_id, set_arm64_soc_id : 9;
    pub reserved2, set_reserved2 : 15, 10;
}

bitfield! {
    ///
    /// Cache Information - Cache Configuration
    ///
    pub struct CacheConfiguration (u16);
    impl Debug;
    pub cache_level, set_cache_level: 2, 0;
    pub cache_socketed, set_cache_socketed: 3;
    // reserved must be zero
    pub reserved, set_reserved: 4;
    pub location, set_location: 6, 5;
    pub enabled_disabled, set_enabled_disabled: 7;
    pub operational_mode, set_operational_mode: 9, 8;
    // reserved2 must be zero
    pub reserved2, set_reserved2: 15, 10;
}

impl CacheConfiguration {
    pub fn new(
        cache_level: u16,
        cache_socketed: bool,
        location: u16,
        enabled_disabled: bool,
        operational_mode: u16,
    ) -> Self {
        let mut config = CacheConfiguration(0);
        config.set_cache_level(cache_level);
        config.set_cache_socketed(cache_socketed);
        config.set_reserved(false); // reserved must be zero
        config.set_location(location);
        config.set_enabled_disabled(enabled_disabled);
        config.set_operational_mode(operational_mode);
        config.set_reserved2(0); // reserved2 must be zero
        config
    }
}

bitfield! {
    ///
    /// Cache Information - Cache Size
    ///
    pub struct CacheSize (u16);
    impl Debug;
    pub max_size, set_max_size : 14, 0;
    pub granularity, set_granularity : 15;
}

bitfield! {
    ///
    /// Cache Information - Cache Size 2
    ///
    pub struct CacheSize2 (u32);
    impl Debug;
    pub max_size, set_max_size : 30, 0;
    pub granularity, set_granularity : 31;
}

bitfield! {
    ///
    /// Cache Information - SRAM Type
    ///
    pub struct CacheSramTypeData(u16);
    impl Debug;
    pub other, set_other: 0;
    pub unknown, set_unknown: 1;
    pub non_burst, set_non_burst: 2;
    pub burst, set_burst: 3;
    pub pipeline_burst, set_pipeline_burst: 4;
    pub synchronous, set_synchronous: 5;
    pub asynchronous, set_asynchronous: 6;
    // reserved must be zero
    pub reserved, set_reserved: 15, 7;
}

///
/// Cache Information - Error Correction Type
///
#[derive(Copy, Clone, Debug)]
pub enum ErrorCorrectionType {
    Other = 0x01,
    Unknown,
    None,
    Parity,
    SingleBitEcc,
    MutliBitEcc,
}

///
/// Cache Information - System Cache Type
///
#[derive(Copy, Clone, Debug)]
pub enum SystemCacheType {
    Other = 0x01,
    Unknown,
    Instruction,
    Data,
    Unified,
}

///
/// Cache Information - Error Correction Type
///
#[derive(Copy, Clone, Debug)]
pub enum AssociativityField {
    Other = 0x01,
    Unknown,
    DirectMapped,
    SetAssociative2Way,
    SetAssociative4Way,
    FullyAssociative,
    SetAssociative8Way,
    SetAssociative16Way,
    SetAssociative12Way,
    SetAssociative24Way,
    SetAssociative32Way,
    SetAssociative48Way,
    SetAssociative64Way,
    SetAssociative20Way,
}

///
/// System Slots - Slot Type
///
#[derive(Copy, Clone, Debug)]
pub enum SlotType {
    Other = 0x01,
    Unknown,
    Isa,
    Mca,
    Eisa,
    Pci,
    PcCard,
    VlVesa,
    Proprietary,
    ProcessorCardSlot,
    ProprietaryMemroyCardSlot,
    IoRiserCardSlot,
    NuBus,
    // 66MHz Capable PCI
    Pci66mhz,
    Agp,
    Agp2x,
    Agp4x,
    PciX,
    Agp8x,
    M2Socket1DP,
    M2Socket1SD,
    M2Socket2,
    M2Socket3,
    MxmTypeI,
    MxmTypeII,
    MxmTypeIIIStandard,
    MxmTypeIIIHe,
    MxmTypeIV,
    Mxm3TypeA,
    Mxm3TypeB,
    // PCI Express Gen 2 SFF-8639 (U.2)
    PciExpressGen2Sff8629,
    // PCI Express Gen 3 SFF-8639 (U.2)
    PciExpressGen3Sff8629,
    // PCI Express Mini 52-pin (CEM spec. 2.0) with bottom-side keep-outs
    PciExpressMini52PinBottomSideKeepOuts,
    // PCI Express Mini 52-pin (CEM spec. 2.0) without bottom-side keep-outs.
    PciExpressMini52Pin,
    PciExpressMini76Pin,
    // PCI Express Gen 4 SFF-8639 (U.2)
    PciExpressGen4Sff8639,
    // PCI Express Gen 5 SFF-8639 (U.2)
    PciExpressGen5Sff8639,
    OcpNic3SFF,
    OcpNic3LFF,
    // OCP NIC Prior to 3.0
    OcpNicPrior,
    Pc98C20 = 0xA0,
    Pc98C24,
    Pc98E,
    Pc98LocalBus,
    Pc98Card,
    PciExpress,
    PciExpressx1,
    PciExpressx2,
    PciExpressx4,
    PciExpressx8,
    PciExpressx16,
    PciExpressGen2,
    PciExpressGen2x1,
    PciExpressGen2x2,
    PciExpressGen2x4,
    PciExpressGen2x8,
    PciExpressGen2x16,
    PciExpressGen3,
    PciExpressGen3x1,
    PciExpressGen3x2,
    PciExpressGen3x4,
    PciExpressGen3x8,
    PciExpressGen3x16,
    PciExpressGen4 = 0xB8,
    PciExpressGen4x1,
    PciExpressGen4x2,
    PciExpressGen4x4,
    PciExpressGen4x8,
    PciExpressGen4x16,
    PciExpressGen5,
    PciExpressGen5x1,
    PciExpressGen5x2,
    PciExpressGen5x4,
    PciExpressGen5x8,
    PciExpressGen5x16,
    PciExpressGen6,
    // Enterprise and Datacenter 1U E1 Form Factor Slot (EDSFF E1.S, E1.L)
    EdsffE1SE1L,
    // Enterprise and Datacenter 3" E3 Form Factor Slot (EDSFF E3.S, E3.L)
    EdsffE3SE3L,
}

///
/// System Slots - Slot Data Bus Width
///
#[derive(Copy, Clone, Debug)]
pub enum SlotWidth {
    Other = 0x01,
    Unknown,
    Bit8,
    Bit16,
    Bit32,
    Bit64,
    Bit128,
    X1,
    X2,
    X4,
    X8,
    X12,
    X16,
    X32,
}

///
/// System Slots - Current Usage
///
#[derive(Copy, Clone, Debug)]
pub enum CurrentUsage {
    Other = 0x01,
    Unknown,
    Available,
    InUse,
    Unavailable,
}

///
/// System Slots - Slot Length
///
#[derive(Copy, Clone, Debug)]
pub enum SlotLength {
    Other = 0x01,
    Unknown,
    ShortLength,
    LongLength,
    // 2.5" drive form factor
    DriveFF25,
    // 3.5" drive form factor
    DriveFF35,
}

bitfield! {
    ///
    /// System Slots - Slot Charcteristics 1
    ///
    pub struct SlotCharacteristics1(u8);
    impl Debug;
    pub characteristics_unknown, set_characteristics_unknown: 0;
    pub provides_5_volts, set_provides_5_volts: 1;
    pub provides_3_volts, set_provides_3_volts: 2;
    pub shared_slot, set_shared_slot: 3;
    pub pc_supports_pccard16, set_pc_supports_pccard16: 4;
    pub pc_supports_cardbus, set_pc_supports_cardbus: 5;
    pub pc_supports_zoomvideo, set_pc_supports_zoomvideo: 6;
    pub pc_supports_modemringresume, set_pc_supports_modemringresume: 7;
}

bitfield! {
    ///
    /// System Slots - Slot Charcteristics 2
    ///
    /// Various fields were added through SMBIOS Version 3. For not-applicable fields use '0'.
    ///
    pub struct  SlotCharacteristics2(u8);
    impl Debug;
    pub pci_supports_pme, set_pci_supports_pme: 0;
    pub supports_hotplug, set_supports_hotplug: 1;
    pub pci_supports_smbus, set_pci_supports_smbus: 2;
    pub pcie_supports_bifurcation, set_pcie_supports_bifurcation: 3;
    pub supports_async_removal, set_supports_async_removal: 4;
    // Flexbus slot, CXL 1.0 capable
    pub flexbus_slot1, set_flexbus_slot1: 5;
    // Flexbus slot, CXL 2.0 capable
    pub flexbus_slot2, set_flexbus_slot2: 6;
    // Flexbus slot, CXL 3.0 capable
    pub flexbus_slot3, set_flexbus_slot3: 7;
}

bitfield! {
    ///
    /// System Slots - Device/Function Number
    ///
    #[derive(Copy, Clone)] 
    pub struct  DeviceFunctionNumber(u8);
    impl Debug;
    pub function_number, set_function_number : 2, 0;
    pub device_number, set_device_number : 7, 3;
}

///
/// System Slots - Peer Segment/Bus/Device/Function/Width Groups
///
#[derive(Copy, Clone, Debug)]
pub struct MiscSlotPeerGroup {
    pub segment_group_num: u16,
    pub bus_num: u8,
    pub dev_func_num: DeviceFunctionNumber,
    pub data_bus_width: u8,
}

///
/// Memory Array - Location
///
#[derive(Copy, Clone, Debug)]
pub enum MemoryArrayLocation {
    Other = 0x01,
    Unknown,
    SystemBoard,
    IsaAddOn,
    EisaAddOn,
    PciAddOn,
    McaAddOn,
    PcmciaAddOn,
    ProprietaryAddOn,
    NuBus,
    Pc98C20AddOn = 0xA0,
    Pc98C24AddOn,
    Pc98EAddOn,
    Pc98LocalAddOn,
    CxlAddOn,
}

///
/// Memory Array - Use
///
#[derive(Copy, Clone, Debug)]
pub enum MemoryArrayUse {
    Other = 0x01,
    Unknown,
    SystemMemory,
    VideoMemory,
    FlashMemory,
    NonVolatileRam,
    CacheMemory,
}

///
/// Memory Array - Error Correction Types
///
#[derive(Copy, Clone, Debug)]
pub enum ErrorCorrectionTypes {
    Other = 0x01,
    Unknown,
    None,
    Parity,
    SingleBitEcc,
    MultiBitEcc,
    Crc,
}

///
/// Memory Device - Form Factor
///
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MemoryFormFactor {
    Other = 0x01,
    Unknown,
    Simm,
    Sip,
    Chip,
    Dip,
    Zip,
    ProprietaryCard,
    Dimm,
    Tsop,
    RowOfChips,
    Rimm,
    Sodimm,
    Srimm,
    FbDimm,
    Die,
    Camm,
    Cudimm,
    Csodimm,
}

///
/// Memory Device - Type
///
#[derive(Copy, Clone, Debug)]
pub enum MemoryDeviceType {
    Other = 0x01,
    Unknown,
    Dram,
    Edram,
    Vram,
    Sram,
    Ram,
    Rom,
    Flash,
    Eeprom,
    Feprom,
    Eprom,
    Cdram,
    ThreeDram,
    Sdram,
    Sgram,
    Rdram,
    Ddr,
    Ddr2,
    Ddr2FbDimm,
    Ddr3 = 0x18,
    Fbd2,
    Ddr4,
    Lpddr,
    Lpddr2,
    Lpddr3,
    Lpddr4,
    LogicalNonVolatileDevice,
    Hbm,
    Hbm2,
    Ddr5,
    Lpddr5,
    Hbm3,
    Mrdimm,
}

bitfield! {
    ///
    /// Memory Device - Type Detail
    ///
    pub struct MemoryDeviceTypeDetails(u16);
    impl Debug;
    pub reserved, set_reserved: 0;
    pub other, set_other: 1;
    pub unknown, set_unknown: 2;
    pub fast_paged, set_fast_paged: 3;
    pub static_column, set_static_column: 4;
    pub pseudo_static, set_pseudo_static: 5;
    pub rambus, set_rambus: 6;
    pub synchronous, set_synchronous: 7;
    pub cmos, set_cmos: 8;
    pub edo, set_edo: 9;
    pub window_dram, set_window_dram: 10;
    pub cache_dram, set_cache_dram: 11;
    pub nonvolatile, set_nonvolatile: 12;
    pub registered, set_registered: 13;
    pub unbuffered, set_unbuffered: 14;
    pub lr_dimm, set_lr_dimm: 15;
}

///
/// Memory Device - Memory Technology
///
#[derive(Copy, Clone, Debug)]
pub enum MemoryDeviceTechnology {
    Other = 0x01,
    Unknown,
    Dram,
    NvdimmN,
    NvdimmF,
    NvdimmP,
    IntelOptanePersistentMemory,
}

bitfield! {
    ///
    /// Memory Device - Attributes
    ///
    pub struct MemoryDeviceAttributes(u8);
    impl Debug;
    pub rank, set_rank: 3, 0;
    pub reserved, set_reserved: 7, 4;
}

bitfield! {
    ///
    /// Memory Device - Operating Mode Capability
    ///
    pub struct MemoryCapability(u16);
    impl Debug;
    // reserved is set to zero
    pub reserved, set_reserved : 0;
    pub other, set_other : 1;
    pub unknown, set_unknown : 2;
    pub volatile_memory, set_volatile_memory : 3;
    // Byte-accessible persistent memory
    pub byte_persistent_memory, set_byte_persistent_memory : 4;
    // Block-accessible persistent memory
    pub block_persistent_memory, set_block_persistent_memory : 5;
    // reserved2 is set to zero
    pub reserved2, set_reserved2 : 15, 6;
}

macro_rules! impl_to_le_bytes_u8 {
    ($($t:ty),*) => {
        $(
            impl $t {
                pub fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
                    alloc::vec![*self as u8]
                }
            }
        )*
    };
}

macro_rules! impl_to_le_bytes_u16 {
    ($($t:ty),*) => {
        $(
            impl $t {
                pub fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
                    (*self as u16).to_le_bytes().to_vec()
                }
            }
        )*
    };
}

macro_rules! impl_to_le_bytes_bitfield_u8 {
    ($($t:ty),*) => {
        $(
            impl $t {
                pub fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
                    alloc::vec![self.0]
                }
            }
        )*
    };
}

macro_rules! impl_to_le_bytes_bitfield_u16 {
    ($($t:ty),*) => {
        $(
            impl $t {
                pub fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
                    self.0.to_le_bytes().to_vec()
                }
            }
        )*
    };
}

macro_rules! impl_to_le_bytes_bitfield_u32 {
    ($($t:ty),*) => {
        $(
            impl $t {
                pub fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
                    self.0.to_le_bytes().to_vec()
                }
            }
        )*
    };
}

// Use the macros
impl_to_le_bytes_u8!(
    ProcessorTypeData,
    ProcessorUpgrade,
    ErrorCorrectionType,
    SystemCacheType,
    AssociativityField,
    SlotWidth,
    CurrentUsage,
    SlotLength,
    MemoryArrayLocation,
    MemoryArrayUse,
    ErrorCorrectionTypes,
    MemoryFormFactor,
    MemoryDeviceType,
    MemoryDeviceTechnology
);

impl_to_le_bytes_u16!(
    ProcessorFamilyData,
    SlotType
);

impl_to_le_bytes_bitfield_u8!(
    ProcessorVoltage,
    ProcessorInformationStatus,
    SlotCharacteristics1,
    SlotCharacteristics2,
    DeviceFunctionNumber,
    MemoryDeviceAttributes
);

impl_to_le_bytes_bitfield_u16!(
    ProcessorCharacteristics,
    CacheConfiguration,
    CacheSize,
    CacheSramTypeData,
    MemoryDeviceTypeDetails,
    MemoryCapability
);

impl_to_le_bytes_bitfield_u32!(
    CacheSize2
);

impl MiscSlotPeerGroup {
    pub fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut bytes = alloc::vec::Vec::new();
        bytes.extend_from_slice(&self.segment_group_num.to_le_bytes());
        bytes.push(self.bus_num);
        bytes.push(self.dev_func_num.0);
        bytes.push(self.data_bus_width);
        bytes
    }
}