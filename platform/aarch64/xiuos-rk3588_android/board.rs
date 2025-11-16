// Copyright (c) 2025 Syswonder
// hvisor is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
//
// Syswonder Website:
//      https://www.syswonder.org
//
// Authors:
//

use crate::{
    arch::{
        mmu::MemoryType,
        zone::{GicConfig, Gicv3Config, HvArchZoneConfig},
    },
    config::*,
};

pub const BOARD_NAME: &str = "xiuos-rk3588_android";

pub const BOARD_NCPUS: usize = 8;
pub const BOARD_UART_BASE: u64 = 0xfeb5_0000;

#[rustfmt::skip]
pub static BOARD_MPIDR_MAPPINGS: [u64; BOARD_NCPUS] = [
    0x000,   // cpu0
    0x100,   // cpu1
    0x200,   // cpu2
    0x300,   // cpu3
    0x400,   // cpu4
    0x500,   // cpu5
    0x600,   // cpu6
    0x700,   // cpu7
];

/// The physical memory layout of the board.
/// Each address should align to 2M (0x20_0000).
/// Addresses must be in ascending order.
#[rustfmt::skip]
pub const BOARD_PHYSMEM_LIST: &[(u64, u64, MemoryType)] = &[
 // (        start,           end,               type)
    (  0x0000_0000,   0x0020_0000, MemoryType::Device),     // Includes low-address SRAM, marked as Device
    (  0x0020_0000,   0x0840_0000, MemoryType::Normal),
    (  0x0940_0000,   0xf000_0000, MemoryType::Normal),
    (  0xf000_0000, 0x1_0000_0000, MemoryType::Device),     // Dense device region, marked as Device.
    (0x1_0000_0000, 0x3_fc00_0000, MemoryType::Normal),
 // (0x3_fc50_0000, 0x3_fff0_0000, MemoryType::Normal),
    (0x3_fc40_0000, 0x4_0000_0000, MemoryType::Normal),     // aligned to 2 MiB
    (0x4_f000_0000, 0x5_0000_0000, MemoryType::Normal),
];

pub const ROOT_ZONE_DTB_ADDR: u64 = 0x0830_0000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x4040_0000;
pub const ROOT_ZONE_ENTRY: u64 = 0x4040_0000;
pub const ROOT_ZONE_CPUS: u64 = (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

pub const ROOT_ZONE_NAME: &str = "root-linux";

pub const ROOT_ZONE_MEMORY_REGIONS: &[HvConfigMemoryRegion] = &[
    // /proc/iomem System RAM
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0020_0000,
        virtual_start: 0x0020_0000,
        size: 0x0820_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0940_0000,
        virtual_start: 0x0940_0000,
        size: 0xe6c0_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x1_0000_0000,
        virtual_start: 0x1_0000_0000,
        size: 0x2_fc00_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x3_fc50_0000,
        virtual_start: 0x3_fc50_0000,
        size: 0x03a0_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x4_f000_0000,
        virtual_start: 0x4_f000_0000,
        size: 0x1000_0000,
    },
    // Ramoops
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0011_0000,
        virtual_start: 0x0011_0000,
        size: 0x000f_0000,
    },
    // /proc/iomem Devices I/O
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfb00_0000,
        virtual_start: 0xfb00_0000,
        size: 0x0020_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfc00_0000,
        virtual_start: 0xfc00_0000,
        size: 0x0200_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe00_0000,
        virtual_start: 0xfe00_0000,
        size: 0x0080_0000,
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfea0_0000,
        virtual_start: 0xfea0_0000,
        size: 0x0050_0000,
    },
    // SRAM and Other Devices
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x0010_f000,
        virtual_start: 0x0010_f000,
        // size: 0x0100, // 10f000.sram
        size: 0x1000, // aligned with page size
    },
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xff00_1000,
        virtual_start: 0xff00_1000,
        size: 0x000e_e000, //ff001000.sram
    },
    // Unknown Region, maybe we should ask vendor for help
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x0010_0000,
        virtual_start: 0x0010_0000,
        size: 0xf000,
    },
    // Let's try
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xa_0000_0000,
        virtual_start: 0xa_0000_0000,
        size: 0x2_0000_0000, //ff001000.sram
    },
];

pub const ROOT_ZONE_IRQS_BITMAP: &[BitmapWord] = &get_irqs_bitmap(&[
    0x27, 0x28, 0x28, 0x29, 0x2a, 0x2b, 0x2d, 0x2e, 0x40, 0x5d, 0x5e, 0x5f, 0x60, 0x65, 0x66, 0x67,
    0x68, 0x69, 0x6d, 0x6e, 0x6f, 0x70, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e,
    0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8e,
    0x8f, 0x8f, 0x90, 0x90, 0x91, 0x92, 0x92, 0x93, 0x93, 0x94, 0x95, 0x95, 0x96, 0x97, 0x97, 0x98,
    0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa7, 0xa8, 0xa8, 0xa9, 0xa9, 0xaa, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3,
    0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbc, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4,
    0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7,
    0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9,
    0xea, 0xeb, 0xec, 0xed, 0xee, 0xf1, 0xf7, 0xf8, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0x102, 0x103,
    0x109, 0x10a, 0x10f, 0x110, 0x110, 0x111, 0x112, 0x113, 0x114, 0x115, 0x115, 0x116, 0x117,
    0x118, 0x119, 0x11a, 0x11a, 0x11b, 0x11c, 0x11d, 0x11e, 0x11f, 0x11f, 0x120, 0x121, 0x122,
    0x123, 0x124, 0x124, 0x125, 0x126, 0x127, 0x131, 0x132, 0x133, 0x135, 0x136, 0x137, 0x138,
    0x139, 0x141, 0x15b, 0x15d, 0x15e, 0x15f, 0x160, 0x161, 0x162, 0x163, 0x164, 0x165, 0x166,
    0x167, 0x168, 0x169, 0x16a, 0x16b, 0x16c, 0x16d, 0x16e, 0x16f, 0x170, 0x171, 0x172, 0x173,
    0x174, 0x175, 0x176, 0x177, 0x178, 0x178, 0x178, 0x178, 0x179, 0x17a, 0x17a, 0x17a, 0x17a,
    0x17b, 0x17c, 0x17c, 0x17c, 0x17c, 0x17d, 0x17e, 0x17e, 0x17e, 0x17e, 0x17f, 0x188, 0x189,
    0x18f, 0x191, 0x193, 0x196, 0x19b, 0x19d, 0x19f, 0x1a2, 0x1a7, 0x1a8, 0x1a9, 0x1aa, 0x1ad,
    0x1ae, 0x1b0, 0x1c7, 0x1d4,
]);

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    is_aarch32: 0,
    gic_config: GicConfig::Gicv3(Gicv3Config {
        gicd_base: 0xfe60_0000,
        gicd_size: 0x0001_0000,
        gicr_base: 0xfe68_0000,
        gicr_size: 0x0010_0000,
        gits_base: 0x0,
        gits_size: 0x0,
    }),
};

// not configured
pub const ROOT_PCI_CONFIG: HvPciConfig = HvPciConfig {
    ecam_base: 0x0,
    ecam_size: 0x0,
    io_base: 0x0,
    io_size: 0x0,
    pci_io_base: 0x0,
    mem32_base: 0x0,
    mem32_size: 0x0,
    pci_mem32_base: 0x0,
    mem64_base: 0x0,
    mem64_size: 0x0,
    pci_mem64_base: 0x0,
};

pub const ROOT_ZONE_IVC_CONFIG: &[HvIvcConfig] = &[];
