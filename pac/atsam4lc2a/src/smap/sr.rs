#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - Operation done"]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCR` reader - Hold Core reset"]
pub struct HCR_R(crate::FieldReader<bool, bool>);
impl HCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERR` reader - Bus error"]
pub struct BERR_R(crate::FieldReader<bool, bool>);
impl BERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAIL` reader - Failure"]
pub struct FAIL_R(crate::FieldReader<bool, bool>);
impl FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK` reader - Lock"]
pub struct LCK_R(crate::FieldReader<bool, bool>);
impl LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` reader - Enabled"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT` reader - Protected"]
pub struct PROT_R(crate::FieldReader<bool, bool>);
impl PROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGP` reader - Debugger Present"]
pub struct DBGP_R(crate::FieldReader<bool, bool>);
impl DBGP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` reader - State"]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Operation done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hold Core reset"]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protected"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Debugger Present"]
    #[inline(always)]
    pub fn dbgp(&self) -> DBGP_R {
        DBGP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 0x07) as u8)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
