#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `ACIMPL0`"]
pub type ACIMPL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL1`"]
pub type ACIMPL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL2`"]
pub type ACIMPL2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL3`"]
pub type ACIMPL3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL4`"]
pub type ACIMPL4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL5`"]
pub type ACIMPL5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL6`"]
pub type ACIMPL6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACIMPL7`"]
pub type ACIMPL7_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIMPL0`"]
pub type WIMPL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIMPL1`"]
pub type WIMPL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIMPL2`"]
pub type WIMPL2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WIMPL3`"]
pub type WIMPL3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 Implemented"]
    #[inline(always)]
    pub fn acimpl0(&self) -> ACIMPL0_R {
        ACIMPL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 Implemented"]
    #[inline(always)]
    pub fn acimpl1(&self) -> ACIMPL1_R {
        ACIMPL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 2 Implemented"]
    #[inline(always)]
    pub fn acimpl2(&self) -> ACIMPL2_R {
        ACIMPL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 3 Implemented"]
    #[inline(always)]
    pub fn acimpl3(&self) -> ACIMPL3_R {
        ACIMPL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 4 Implemented"]
    #[inline(always)]
    pub fn acimpl4(&self) -> ACIMPL4_R {
        ACIMPL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 5 Implemented"]
    #[inline(always)]
    pub fn acimpl5(&self) -> ACIMPL5_R {
        ACIMPL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 6 Implemented"]
    #[inline(always)]
    pub fn acimpl6(&self) -> ACIMPL6_R {
        ACIMPL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 7 Implemented"]
    #[inline(always)]
    pub fn acimpl7(&self) -> ACIMPL7_R {
        ACIMPL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Window0 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl0(&self) -> WIMPL0_R {
        WIMPL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Window1 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl1(&self) -> WIMPL1_R {
        WIMPL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Window2 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl2(&self) -> WIMPL2_R {
        WIMPL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Window3 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl3(&self) -> WIMPL3_R {
        WIMPL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
