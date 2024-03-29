#[doc = "Register `CIDR` reader"]
pub struct R(crate::R<CIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version of the Device"]
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EPROC_R = crate::FieldReader<u8, EPROC_A>;
#[doc = "Embedded Processor"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPROC_A {
    #[doc = "1: ARM946ES"]
    ARM946ES = 1,
    #[doc = "2: ARM7TDMI"]
    ARM7TDMI = 2,
    #[doc = "3: Cortex-M3"]
    CM3 = 3,
    #[doc = "4: ARM920T"]
    ARM920T = 4,
    #[doc = "5: ARM926EJS"]
    ARM926EJS = 5,
    #[doc = "6: Cortex-A5"]
    CA5 = 6,
    #[doc = "7: Cortex-M4"]
    CM4 = 7,
}
impl From<EPROC_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROC_A) -> Self {
        variant as _
    }
}
impl EPROC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPROC_A> {
        match self.bits {
            1 => Some(EPROC_A::ARM946ES),
            2 => Some(EPROC_A::ARM7TDMI),
            3 => Some(EPROC_A::CM3),
            4 => Some(EPROC_A::ARM920T),
            5 => Some(EPROC_A::ARM926EJS),
            6 => Some(EPROC_A::CA5),
            7 => Some(EPROC_A::CM4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROC_A::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROC_A::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == EPROC_A::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROC_A::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROC_A::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == EPROC_A::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == EPROC_A::CM4
    }
}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NVPSIZ_R = crate::FieldReader<u8, NVPSIZ_A>;
#[doc = "Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ_A) -> Self {
        variant as _
    }
}
impl NVPSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ_A> {
        match self.bits {
            0 => Some(NVPSIZ_A::NONE),
            1 => Some(NVPSIZ_A::_8K),
            2 => Some(NVPSIZ_A::_16K),
            3 => Some(NVPSIZ_A::_32K),
            5 => Some(NVPSIZ_A::_64K),
            7 => Some(NVPSIZ_A::_128K),
            9 => Some(NVPSIZ_A::_256K),
            10 => Some(NVPSIZ_A::_512K),
            12 => Some(NVPSIZ_A::_1024K),
            14 => Some(NVPSIZ_A::_2048K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ_A::_2048K
    }
}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type NVPSIZ2_R = crate::FieldReader<u8, NVPSIZ2_A>;
#[doc = "Second Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ2_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ2_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ2_A) -> Self {
        variant as _
    }
}
impl NVPSIZ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ2_A> {
        match self.bits {
            0 => Some(NVPSIZ2_A::NONE),
            1 => Some(NVPSIZ2_A::_8K),
            2 => Some(NVPSIZ2_A::_16K),
            3 => Some(NVPSIZ2_A::_32K),
            5 => Some(NVPSIZ2_A::_64K),
            7 => Some(NVPSIZ2_A::_128K),
            9 => Some(NVPSIZ2_A::_256K),
            10 => Some(NVPSIZ2_A::_512K),
            12 => Some(NVPSIZ2_A::_1024K),
            14 => Some(NVPSIZ2_A::_2048K),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2_A::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2_A::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2_A::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2_A::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2_A::_2048K
    }
}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SRAMSIZ_R = crate::FieldReader<u8, SRAMSIZ_A>;
#[doc = "Internal SRAM Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAMSIZ_A {
    #[doc = "0: 48 Kbytes"]
    _48K = 0,
    #[doc = "1: 192 Kbytes"]
    _192K = 1,
    #[doc = "2: 2 Kbytes"]
    _2K = 2,
    #[doc = "3: 6 Kbytes"]
    _6K = 3,
    #[doc = "4: 24 Kbytes"]
    _24K = 4,
    #[doc = "5: 4 Kbytes"]
    _4K = 5,
    #[doc = "6: 80 Kbytes"]
    _80K = 6,
    #[doc = "7: 160 Kbytes"]
    _160K = 7,
    #[doc = "8: 8 Kbytes"]
    _8K = 8,
    #[doc = "9: 16 Kbytes"]
    _16K = 9,
    #[doc = "10: 32 Kbytes"]
    _32K = 10,
    #[doc = "11: 64 Kbytes"]
    _64K = 11,
    #[doc = "12: 128 Kbytes"]
    _128K = 12,
    #[doc = "13: 256 Kbytes"]
    _256K = 13,
    #[doc = "14: 96 Kbytes"]
    _96K = 14,
    #[doc = "15: 512 Kbytes"]
    _512K = 15,
}
impl From<SRAMSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZ_A) -> Self {
        variant as _
    }
}
impl SRAMSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSIZ_A {
        match self.bits {
            0 => SRAMSIZ_A::_48K,
            1 => SRAMSIZ_A::_192K,
            2 => SRAMSIZ_A::_2K,
            3 => SRAMSIZ_A::_6K,
            4 => SRAMSIZ_A::_24K,
            5 => SRAMSIZ_A::_4K,
            6 => SRAMSIZ_A::_80K,
            7 => SRAMSIZ_A::_160K,
            8 => SRAMSIZ_A::_8K,
            9 => SRAMSIZ_A::_16K,
            10 => SRAMSIZ_A::_32K,
            11 => SRAMSIZ_A::_64K,
            12 => SRAMSIZ_A::_128K,
            13 => SRAMSIZ_A::_256K,
            14 => SRAMSIZ_A::_96K,
            15 => SRAMSIZ_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZ_A::_48K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZ_A::_192K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == SRAMSIZ_A::_2K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZ_A::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZ_A::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZ_A::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZ_A::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZ_A::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZ_A::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZ_A::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZ_A::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZ_A::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZ_A::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZ_A::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZ_A::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZ_A::_512K
    }
}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ARCH_R = crate::FieldReader<u8, ARCH_A>;
#[doc = "Architecture Identifier"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARCH_A {
    #[doc = "60: SAM4E Series"]
    SAM4E = 60,
}
impl From<ARCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCH_A) -> Self {
        variant as _
    }
}
impl ARCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARCH_A> {
        match self.bits {
            60 => Some(ARCH_A::SAM4E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SAM4E`"]
    #[inline(always)]
    pub fn is_sam4e(&self) -> bool {
        *self == ARCH_A::SAM4E
    }
}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NVPTYP_R = crate::FieldReader<u8, NVPTYP_A>;
#[doc = "Nonvolatile Program Memory Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPTYP_A {
    #[doc = "0: ROM"]
    ROM = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    ROMLESS = 1,
    #[doc = "2: Embedded Flash Memory"]
    FLASH = 2,
    #[doc = "3: ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    ROM_FLASH = 3,
    #[doc = "4: SRAM emulating ROM"]
    SRAM = 4,
}
impl From<NVPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPTYP_A) -> Self {
        variant as _
    }
}
impl NVPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPTYP_A> {
        match self.bits {
            0 => Some(NVPTYP_A::ROM),
            1 => Some(NVPTYP_A::ROMLESS),
            2 => Some(NVPTYP_A::FLASH),
            3 => Some(NVPTYP_A::ROM_FLASH),
            4 => Some(NVPTYP_A::SRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYP_A::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYP_A::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYP_A::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYP_A::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYP_A::SRAM
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub type EXT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr](index.html) module"]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr::R](R) reader structure"]
impl crate::Readable for CIDR_SPEC {
    type Reader = R;
}
