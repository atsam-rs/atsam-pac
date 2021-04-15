#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `ACCS0`"]
pub type ACCS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY0`"]
pub type ACRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS1`"]
pub type ACCS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY1`"]
pub type ACRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS2`"]
pub type ACCS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY2`"]
pub type ACRDY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS3`"]
pub type ACCS3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY3`"]
pub type ACRDY3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS4`"]
pub type ACCS4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY4`"]
pub type ACRDY4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS5`"]
pub type ACCS5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY5`"]
pub type ACRDY5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS6`"]
pub type ACCS6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY6`"]
pub type ACRDY6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACCS7`"]
pub type ACCS7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACRDY7`"]
pub type ACRDY7_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFCS0`"]
pub type WFCS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFCS1`"]
pub type WFCS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFCS2`"]
pub type WFCS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFCS3`"]
pub type WFCS3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - AC0 Current Comparison Status"]
    #[inline(always)]
    pub fn accs0(&self) -> ACCS0_R {
        ACCS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AC0 Ready"]
    #[inline(always)]
    pub fn acrdy0(&self) -> ACRDY0_R {
        ACRDY0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AC1 Current Comparison Status"]
    #[inline(always)]
    pub fn accs1(&self) -> ACCS1_R {
        ACCS1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AC1 Ready"]
    #[inline(always)]
    pub fn acrdy1(&self) -> ACRDY1_R {
        ACRDY1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AC2 Current Comparison Status"]
    #[inline(always)]
    pub fn accs2(&self) -> ACCS2_R {
        ACCS2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AC2 Ready"]
    #[inline(always)]
    pub fn acrdy2(&self) -> ACRDY2_R {
        ACRDY2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AC3 Current Comparison Status"]
    #[inline(always)]
    pub fn accs3(&self) -> ACCS3_R {
        ACCS3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AC3 Ready"]
    #[inline(always)]
    pub fn acrdy3(&self) -> ACRDY3_R {
        ACRDY3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AC4 Current Comparison Status"]
    #[inline(always)]
    pub fn accs4(&self) -> ACCS4_R {
        ACCS4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AC4 Ready"]
    #[inline(always)]
    pub fn acrdy4(&self) -> ACRDY4_R {
        ACRDY4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AC5 Current Comparison Status"]
    #[inline(always)]
    pub fn accs5(&self) -> ACCS5_R {
        ACCS5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AC5 Ready"]
    #[inline(always)]
    pub fn acrdy5(&self) -> ACRDY5_R {
        ACRDY5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AC6 Current Comparison Status"]
    #[inline(always)]
    pub fn accs6(&self) -> ACCS6_R {
        ACCS6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AC6 Ready"]
    #[inline(always)]
    pub fn acrdy6(&self) -> ACRDY6_R {
        ACRDY6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AC7 Current Comparison Status"]
    #[inline(always)]
    pub fn accs7(&self) -> ACCS7_R {
        ACCS7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AC7 Ready"]
    #[inline(always)]
    pub fn acrdy7(&self) -> ACRDY7_R {
        ACRDY7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Window0 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs0(&self) -> WFCS0_R {
        WFCS0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Window1 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs1(&self) -> WFCS1_R {
        WFCS1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Window2 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs2(&self) -> WFCS2_R {
        WFCS2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Window3 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs3(&self) -> WFCS3_R {
        WFCS3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
