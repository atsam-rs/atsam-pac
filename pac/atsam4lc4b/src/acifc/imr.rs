#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `ACINT0`"]
pub type ACINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT0`"]
pub type SUTINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT1`"]
pub type ACINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT1`"]
pub type SUTINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT2`"]
pub type ACINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT2`"]
pub type SUTINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT3`"]
pub type ACINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT3`"]
pub type SUTINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT4`"]
pub type ACINT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT4`"]
pub type SUTINT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT5`"]
pub type ACINT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT5`"]
pub type SUTINT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT6`"]
pub type ACINT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT6`"]
pub type SUTINT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACINT7`"]
pub type ACINT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTINT7`"]
pub type SUTINT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFINT0`"]
pub type WFINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFINT1`"]
pub type WFINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFINT2`"]
pub type WFINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFINT3`"]
pub type WFINT3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - AC0 Interrupt Mask"]
    #[inline(always)]
    pub fn acint0(&self) -> ACINT0_R {
        ACINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AC0 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint0(&self) -> SUTINT0_R {
        SUTINT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AC1 Interrupt Mask"]
    #[inline(always)]
    pub fn acint1(&self) -> ACINT1_R {
        ACINT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AC1 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint1(&self) -> SUTINT1_R {
        SUTINT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AC2 Interrupt Mask"]
    #[inline(always)]
    pub fn acint2(&self) -> ACINT2_R {
        ACINT2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AC2 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint2(&self) -> SUTINT2_R {
        SUTINT2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AC3 Interrupt Mask"]
    #[inline(always)]
    pub fn acint3(&self) -> ACINT3_R {
        ACINT3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AC3 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint3(&self) -> SUTINT3_R {
        SUTINT3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AC4 Interrupt Mask"]
    #[inline(always)]
    pub fn acint4(&self) -> ACINT4_R {
        ACINT4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AC4 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint4(&self) -> SUTINT4_R {
        SUTINT4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AC5 Interrupt Mask"]
    #[inline(always)]
    pub fn acint5(&self) -> ACINT5_R {
        ACINT5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AC5 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint5(&self) -> SUTINT5_R {
        SUTINT5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AC6 Interrupt Mask"]
    #[inline(always)]
    pub fn acint6(&self) -> ACINT6_R {
        ACINT6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AC6 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint6(&self) -> SUTINT6_R {
        SUTINT6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AC7 Interrupt Mask"]
    #[inline(always)]
    pub fn acint7(&self) -> ACINT7_R {
        ACINT7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AC7 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint7(&self) -> SUTINT7_R {
        SUTINT7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Window0 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint0(&self) -> WFINT0_R {
        WFINT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Window1 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint1(&self) -> WFINT1_R {
        WFINT1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Window2 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint2(&self) -> WFINT2_R {
        WFINT2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Window3 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint3(&self) -> WFINT3_R {
        WFINT3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
