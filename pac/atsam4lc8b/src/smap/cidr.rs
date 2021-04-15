#[doc = "Reader of register CIDR"]
pub type R = crate::R<u32, super::CIDR>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `EPROC`"]
pub type EPROC_R = crate::R<u8, u8>;
#[doc = "Reader of field `NVPSIZ`"]
pub type NVPSIZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `NVPSIZ2`"]
pub type NVPSIZ2_R = crate::R<u8, u8>;
#[doc = "Reader of field `SRAMSIZ`"]
pub type SRAMSIZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `ARCH`"]
pub type ARCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `NVPTYP`"]
pub type NVPTYP_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 0x07) as u8)
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
    #[doc = "Bits 16:20 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
