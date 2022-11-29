#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEOC` reader - Sequencer end of conversion"]
pub type SEOC_R = crate::BitReader<bool>;
#[doc = "Field `LOVR` reader - Sequencer last converted value overrun"]
pub type LOVR_R = crate::BitReader<bool>;
#[doc = "Field `WM` reader - Window monitor"]
pub type WM_R = crate::BitReader<bool>;
#[doc = "Field `SMTRG` reader - Sequencer missed trigger event"]
pub type SMTRG_R = crate::BitReader<bool>;
#[doc = "Field `SUTD` reader - Start-up time done"]
pub type SUTD_R = crate::BitReader<bool>;
#[doc = "Field `TTO` reader - Timer time-out"]
pub type TTO_R = crate::BitReader<bool>;
#[doc = "Field `EN` reader - Enable Status"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `TBUSY` reader - Timer busy"]
pub type TBUSY_R = crate::BitReader<bool>;
#[doc = "Field `SBUSY` reader - Sequencer busy"]
pub type SBUSY_R = crate::BitReader<bool>;
#[doc = "Field `CBUSY` reader - Conversion busy"]
pub type CBUSY_R = crate::BitReader<bool>;
#[doc = "Field `REFBUF` reader - Reference buffer status"]
pub type REFBUF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Sequencer end of conversion"]
    #[inline(always)]
    pub fn seoc(&self) -> SEOC_R {
        SEOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun"]
    #[inline(always)]
    pub fn lovr(&self) -> LOVR_R {
        LOVR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window monitor"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event"]
    #[inline(always)]
    pub fn smtrg(&self) -> SMTRG_R {
        SMTRG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Start-up time done"]
    #[inline(always)]
    pub fn sutd(&self) -> SUTD_R {
        SUTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer time-out"]
    #[inline(always)]
    pub fn tto(&self) -> TTO_R {
        TTO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Status"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer busy"]
    #[inline(always)]
    pub fn tbusy(&self) -> TBUSY_R {
        TBUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sequencer busy"]
    #[inline(always)]
    pub fn sbusy(&self) -> SBUSY_R {
        SBUSY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Conversion busy"]
    #[inline(always)]
    pub fn cbusy(&self) -> CBUSY_R {
        CBUSY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reference buffer status"]
    #[inline(always)]
    pub fn refbuf(&self) -> REFBUF_R {
        REFBUF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
