#[doc = "Register `PMCON` reader"]
pub struct R(crate::R<PMCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMCON_SPEC>> for R {
    fn from(reader: crate::R<PMCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCON` writer"]
pub struct W(crate::W<PMCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<PMCON_SPEC>> for W {
    fn from(writer: crate::W<PMCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Power Scaling Configuration Value"]
pub struct PS_R(crate::FieldReader<u8, u8>);
impl PS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Power Scaling Configuration Value"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PSCREQ` reader - Power Scaling Change Request"]
pub struct PSCREQ_R(crate::FieldReader<bool, bool>);
impl PSCREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSCREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSCREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSCREQ` writer - Power Scaling Change Request"]
pub struct PSCREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PSCREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PSCM` reader - Power Scaling Change Mode"]
pub struct PSCM_R(crate::FieldReader<bool, bool>);
impl PSCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSCM` writer - Power Scaling Change Mode"]
pub struct PSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSCM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `BKUP` reader - BACKUP Mode"]
pub struct BKUP_R(crate::FieldReader<bool, bool>);
impl BKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKUP` writer - BACKUP Mode"]
pub struct BKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RET` reader - RETENTION Mode"]
pub struct RET_R(crate::FieldReader<bool, bool>);
impl RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RET` writer - RETENTION Mode"]
pub struct RET_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SLEEP` reader - SLEEP mode Configuration"]
pub struct SLEEP_R(crate::FieldReader<u8, u8>);
impl SLEEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - SLEEP mode Configuration"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CK32S` reader - 32Khz-1Khz Clock Source Selection"]
pub struct CK32S_R(crate::FieldReader<bool, bool>);
impl CK32S_R {
    pub(crate) fn new(bits: bool) -> Self {
        CK32S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK32S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK32S` writer - 32Khz-1Khz Clock Source Selection"]
pub struct CK32S_W<'a> {
    w: &'a mut W,
}
impl<'a> CK32S_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FASTWKUP` reader - Fast Wakeup"]
pub struct FASTWKUP_R(crate::FieldReader<bool, bool>);
impl FASTWKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTWKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTWKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTWKUP` writer - Fast Wakeup"]
pub struct FASTWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTWKUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Scaling Configuration Value"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Power Scaling Change Request"]
    #[inline(always)]
    pub fn pscreq(&self) -> PSCREQ_R {
        PSCREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Scaling Change Mode"]
    #[inline(always)]
    pub fn pscm(&self) -> PSCM_R {
        PSCM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BACKUP Mode"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RETENTION Mode"]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - SLEEP mode Configuration"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - 32Khz-1Khz Clock Source Selection"]
    #[inline(always)]
    pub fn ck32s(&self) -> CK32S_R {
        CK32S_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Scaling Configuration Value"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 2 - Power Scaling Change Request"]
    #[inline(always)]
    pub fn pscreq(&mut self) -> PSCREQ_W {
        PSCREQ_W { w: self }
    }
    #[doc = "Bit 3 - Power Scaling Change Mode"]
    #[inline(always)]
    pub fn pscm(&mut self) -> PSCM_W {
        PSCM_W { w: self }
    }
    #[doc = "Bit 8 - BACKUP Mode"]
    #[inline(always)]
    pub fn bkup(&mut self) -> BKUP_W {
        BKUP_W { w: self }
    }
    #[doc = "Bit 9 - RETENTION Mode"]
    #[inline(always)]
    pub fn ret(&mut self) -> RET_W {
        RET_W { w: self }
    }
    #[doc = "Bits 12:13 - SLEEP mode Configuration"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 16 - 32Khz-1Khz Clock Source Selection"]
    #[inline(always)]
    pub fn ck32s(&mut self) -> CK32S_W {
        CK32S_W { w: self }
    }
    #[doc = "Bit 24 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&mut self) -> FASTWKUP_W {
        FASTWKUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcon](index.html) module"]
pub struct PMCON_SPEC;
impl crate::RegisterSpec for PMCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmcon::R](R) reader structure"]
impl crate::Readable for PMCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmcon::W](W) writer structure"]
impl crate::Writable for PMCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCON to value 0"]
impl crate::Resettable for PMCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
