#[doc = "Reader of register OVER"]
pub type R = crate::R<u32, super::OVER>;
#[doc = "Reader of field `OVRE0`"]
pub type OVRE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE1`"]
pub type OVRE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE2`"]
pub type OVRE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE3`"]
pub type OVRE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE4`"]
pub type OVRE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE5`"]
pub type OVRE5_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE6`"]
pub type OVRE6_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE7`"]
pub type OVRE7_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE8`"]
pub type OVRE8_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE9`"]
pub type OVRE9_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE10`"]
pub type OVRE10_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE11`"]
pub type OVRE11_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE12`"]
pub type OVRE12_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE13`"]
pub type OVRE13_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE14`"]
pub type OVRE14_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE15`"]
pub type OVRE15_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE16`"]
pub type OVRE16_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVRE17`"]
pub type OVRE17_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE18`"]
pub type OVRE18_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE19`"]
pub type OVRE19_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE20`"]
pub type OVRE20_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE21`"]
pub type OVRE21_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE22`"]
pub type OVRE22_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> OVRE0_R {
        OVRE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> OVRE1_R {
        OVRE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> OVRE2_R {
        OVRE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> OVRE3_R {
        OVRE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> OVRE4_R {
        OVRE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> OVRE5_R {
        OVRE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> OVRE6_R {
        OVRE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> OVRE7_R {
        OVRE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error 8"]
    #[inline(always)]
    pub fn ovre8(&self) -> OVRE8_R {
        OVRE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Overrun Error 9"]
    #[inline(always)]
    pub fn ovre9(&self) -> OVRE9_R {
        OVRE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun Error 10"]
    #[inline(always)]
    pub fn ovre10(&self) -> OVRE10_R {
        OVRE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Overrun Error 11"]
    #[inline(always)]
    pub fn ovre11(&self) -> OVRE11_R {
        OVRE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Overrun Error 12"]
    #[inline(always)]
    pub fn ovre12(&self) -> OVRE12_R {
        OVRE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Overrun Error 13"]
    #[inline(always)]
    pub fn ovre13(&self) -> OVRE13_R {
        OVRE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Overrun Error 14"]
    #[inline(always)]
    pub fn ovre14(&self) -> OVRE14_R {
        OVRE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Overrun Error 15"]
    #[inline(always)]
    pub fn ovre15(&self) -> OVRE15_R {
        OVRE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Overrun Error 16"]
    #[inline(always)]
    pub fn ovre16(&self) -> OVRE16_R {
        OVRE16_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Overrun Error 17"]
    #[inline(always)]
    pub fn ovre17(&self) -> OVRE17_R {
        OVRE17_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Overrun Error 18"]
    #[inline(always)]
    pub fn ovre18(&self) -> OVRE18_R {
        OVRE18_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Overrun Error 19"]
    #[inline(always)]
    pub fn ovre19(&self) -> OVRE19_R {
        OVRE19_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Overrun Error 20"]
    #[inline(always)]
    pub fn ovre20(&self) -> OVRE20_R {
        OVRE20_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Overrun Error 21"]
    #[inline(always)]
    pub fn ovre21(&self) -> OVRE21_R {
        OVRE21_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Overrun Error 22"]
    #[inline(always)]
    pub fn ovre22(&self) -> OVRE22_R {
        OVRE22_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
