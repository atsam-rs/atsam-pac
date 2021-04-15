#[doc = "Register `UDINT` reader"]
pub struct R(crate::R<UDINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UDINT_SPEC>> for R {
    fn from(reader: crate::R<UDINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSP` reader - Suspend Interrupt"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSOF` reader - Micro Start of Frame Interrupt"]
pub struct MSOF_R(crate::FieldReader<bool, bool>);
impl MSOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` reader - Start of Frame Interrupt"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORST` reader - End of Reset Interrupt"]
pub struct EORST_R(crate::FieldReader<bool, bool>);
impl EORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` reader - Wake-Up Interrupt"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORSM` reader - End Of Resume Interrupt"]
pub struct EORSM_R(crate::FieldReader<bool, bool>);
impl EORSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EORSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt"]
pub struct UPRSM_R(crate::FieldReader<bool, bool>);
impl UPRSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP0INT` reader - Endpoint 0 Interrupt"]
pub struct EP0INT_R(crate::FieldReader<bool, bool>);
impl EP0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1INT` reader - Endpoint 1 Interrupt"]
pub struct EP1INT_R(crate::FieldReader<bool, bool>);
impl EP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2INT` reader - Endpoint 2 Interrupt"]
pub struct EP2INT_R(crate::FieldReader<bool, bool>);
impl EP2INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3INT` reader - Endpoint 3 Interrupt"]
pub struct EP3INT_R(crate::FieldReader<bool, bool>);
impl EP3INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4INT` reader - Endpoint 4 Interrupt"]
pub struct EP4INT_R(crate::FieldReader<bool, bool>);
impl EP4INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5INT` reader - Endpoint 5 Interrupt"]
pub struct EP5INT_R(crate::FieldReader<bool, bool>);
impl EP5INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6INT` reader - Endpoint 6 Interrupt"]
pub struct EP6INT_R(crate::FieldReader<bool, bool>);
impl EP6INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP6INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7INT` reader - Endpoint 7 Interrupt"]
pub struct EP7INT_R(crate::FieldReader<bool, bool>);
impl EP7INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP7INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Suspend Interrupt"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "Device Global Interupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udint](index.html) module"]
pub struct UDINT_SPEC;
impl crate::RegisterSpec for UDINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udint::R](R) reader structure"]
impl crate::Readable for UDINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UDINT to value 0"]
impl crate::Resettable for UDINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
