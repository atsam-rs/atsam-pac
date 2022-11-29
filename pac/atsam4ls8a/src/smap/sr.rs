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
#[doc = "Field `DONE` reader - Operation done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `HCR` reader - Hold Core reset"]
pub type HCR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `FAIL` reader - Failure"]
pub type FAIL_R = crate::BitReader<bool>;
#[doc = "Field `LCK` reader - Lock"]
pub type LCK_R = crate::BitReader<bool>;
#[doc = "Field `EN` reader - Enabled"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `PROT` reader - Protected"]
pub type PROT_R = crate::BitReader<bool>;
#[doc = "Field `DBGP` reader - Debugger Present"]
pub type DBGP_R = crate::BitReader<bool>;
#[doc = "Field `STATE` reader - State"]
pub type STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hold Core reset"]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protected"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Debugger Present"]
    #[inline(always)]
    pub fn dbgp(&self) -> DBGP_R {
        DBGP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:26 - State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 7) as u8)
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
