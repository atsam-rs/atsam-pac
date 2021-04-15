#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `SEOC`"]
pub type SEOC_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOVR`"]
pub type LOVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WM`"]
pub type WM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMTRG`"]
pub type SMTRG_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUTD`"]
pub type SUTD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TTO`"]
pub type TTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBUSY`"]
pub type TBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBUSY`"]
pub type SBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBUSY`"]
pub type CBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `REFBUF`"]
pub type REFBUF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Sequencer end of conversion"]
    #[inline(always)]
    pub fn seoc(&self) -> SEOC_R {
        SEOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun"]
    #[inline(always)]
    pub fn lovr(&self) -> LOVR_R {
        LOVR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Window monitor"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event"]
    #[inline(always)]
    pub fn smtrg(&self) -> SMTRG_R {
        SMTRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start-up time done"]
    #[inline(always)]
    pub fn sutd(&self) -> SUTD_R {
        SUTD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer time-out"]
    #[inline(always)]
    pub fn tto(&self) -> TTO_R {
        TTO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer busy"]
    #[inline(always)]
    pub fn tbusy(&self) -> TBUSY_R {
        TBUSY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Sequencer busy"]
    #[inline(always)]
    pub fn sbusy(&self) -> SBUSY_R {
        SBUSY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Conversion busy"]
    #[inline(always)]
    pub fn cbusy(&self) -> CBUSY_R {
        CBUSY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Reference buffer status"]
    #[inline(always)]
    pub fn refbuf(&self) -> REFBUF_R {
        REFBUF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
