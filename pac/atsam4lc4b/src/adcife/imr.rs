#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `SEOC`"]
pub type SEOC_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOVR`"]
pub type LOVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WM`"]
pub type WM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMTRG`"]
pub type SMTRG_R = crate::R<bool, bool>;
#[doc = "Reader of field `TTO`"]
pub type TTO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Sequencer end of conversion Interrupt Mask"]
    #[inline(always)]
    pub fn seoc(&self) -> SEOC_R {
        SEOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun Interrupt Mask"]
    #[inline(always)]
    pub fn lovr(&self) -> LOVR_R {
        LOVR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Window monitor Interrupt Mask"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event Interrupt Mask"]
    #[inline(always)]
    pub fn smtrg(&self) -> SMTRG_R {
        SMTRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer time-out Interrupt Mask"]
    #[inline(always)]
    pub fn tto(&self) -> TTO_R {
        TTO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
