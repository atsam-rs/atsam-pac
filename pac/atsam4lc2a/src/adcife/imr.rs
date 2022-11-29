#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEOC` reader - Sequencer end of conversion Interrupt Mask"]
pub type SEOC_R = crate::BitReader<bool>;
#[doc = "Field `LOVR` reader - Sequencer last converted value overrun Interrupt Mask"]
pub type LOVR_R = crate::BitReader<bool>;
#[doc = "Field `WM` reader - Window monitor Interrupt Mask"]
pub type WM_R = crate::BitReader<bool>;
#[doc = "Field `SMTRG` reader - Sequencer missed trigger event Interrupt Mask"]
pub type SMTRG_R = crate::BitReader<bool>;
#[doc = "Field `TTO` reader - Timer time-out Interrupt Mask"]
pub type TTO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sequencer end of conversion Interrupt Mask"]
    #[inline(always)]
    pub fn seoc(&self) -> SEOC_R {
        SEOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun Interrupt Mask"]
    #[inline(always)]
    pub fn lovr(&self) -> LOVR_R {
        LOVR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window monitor Interrupt Mask"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event Interrupt Mask"]
    #[inline(always)]
    pub fn smtrg(&self) -> SMTRG_R {
        SMTRG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer time-out Interrupt Mask"]
    #[inline(always)]
    pub fn tto(&self) -> TTO_R {
        TTO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
