#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow"]
pub struct COVFS_R(crate::FieldReader<bool, bool>);
impl COVFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        COVFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COVFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOVRS` reader - Load Overrun"]
pub struct LOVRS_R(crate::FieldReader<bool, bool>);
impl LOVRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOVRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOVRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPAS` reader - RA Compare"]
pub struct CPAS_R(crate::FieldReader<bool, bool>);
impl CPAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPBS` reader - RB Compare"]
pub struct CPBS_R(crate::FieldReader<bool, bool>);
impl CPBS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPBS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPCS` reader - RC Compare"]
pub struct CPCS_R(crate::FieldReader<bool, bool>);
impl CPCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDRAS` reader - RA Loading"]
pub struct LDRAS_R(crate::FieldReader<bool, bool>);
impl LDRAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDRAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDRBS` reader - RB Loading"]
pub struct LDRBS_R(crate::FieldReader<bool, bool>);
impl LDRBS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDRBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDRBS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRGS` reader - External Trigger"]
pub struct ETRGS_R(crate::FieldReader<bool, bool>);
impl ETRGS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETRGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETRGS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDRX` reader - End of Receiver Transfer"]
pub struct ENDRX_R(crate::FieldReader<bool, bool>);
impl ENDRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUFF` reader - Reception Buffer Full"]
pub struct RXBUFF_R(crate::FieldReader<bool, bool>);
impl RXBUFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reception Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
