#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDV` reader - Watchdog Counter Value"]
pub struct WDV_R(crate::FieldReader<u16, u16>);
impl WDV_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDV` writer - Watchdog Counter Value"]
pub struct WDV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `WDFIEN` reader - Watchdog Fault Interrupt Enable"]
pub struct WDFIEN_R(crate::FieldReader<bool, bool>);
impl WDFIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDFIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDFIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDFIEN` writer - Watchdog Fault Interrupt Enable"]
pub struct WDFIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDFIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `WDRSTEN` reader - Watchdog Reset Enable"]
pub struct WDRSTEN_R(crate::FieldReader<bool, bool>);
impl WDRSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDRSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRSTEN` writer - Watchdog Reset Enable"]
pub struct WDRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `WDRPROC` reader - Watchdog Reset Processor"]
pub struct WDRPROC_R(crate::FieldReader<bool, bool>);
impl WDRPROC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDRPROC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRPROC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRPROC` writer - Watchdog Reset Processor"]
pub struct WDRPROC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRPROC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `WDDIS` reader - Watchdog Disable"]
pub struct WDDIS_R(crate::FieldReader<bool, bool>);
impl WDDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDDIS` writer - Watchdog Disable"]
pub struct WDDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `WDD` reader - Watchdog Delta Value"]
pub struct WDD_R(crate::FieldReader<u16, u16>);
impl WDD_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDD` writer - Watchdog Delta Value"]
pub struct WDD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `WDDBGHLT` reader - Watchdog Debug Halt"]
pub struct WDDBGHLT_R(crate::FieldReader<bool, bool>);
impl WDDBGHLT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDDBGHLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDDBGHLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDDBGHLT` writer - Watchdog Debug Halt"]
pub struct WDDBGHLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDDBGHLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `WDIDLEHLT` reader - Watchdog Idle Halt"]
pub struct WDIDLEHLT_R(crate::FieldReader<bool, bool>);
impl WDIDLEHLT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDIDLEHLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDIDLEHLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDIDLEHLT` writer - Watchdog Idle Halt"]
pub struct WDIDLEHLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIDLEHLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Watchdog Counter Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Watchdog Fault Interrupt Enable"]
    #[inline(always)]
    pub fn wdfien(&self) -> WDFIEN_R {
        WDFIEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdrsten(&self) -> WDRSTEN_R {
        WDRSTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Watchdog Reset Processor"]
    #[inline(always)]
    pub fn wdrproc(&self) -> WDRPROC_R {
        WDRPROC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Watchdog Disable"]
    #[inline(always)]
    pub fn wddis(&self) -> WDDIS_R {
        WDDIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Watchdog Delta Value"]
    #[inline(always)]
    pub fn wdd(&self) -> WDD_R {
        WDD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Watchdog Debug Halt"]
    #[inline(always)]
    pub fn wddbghlt(&self) -> WDDBGHLT_R {
        WDDBGHLT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Watchdog Idle Halt"]
    #[inline(always)]
    pub fn wdidlehlt(&self) -> WDIDLEHLT_R {
        WDIDLEHLT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog Counter Value"]
    #[inline(always)]
    pub fn wdv(&mut self) -> WDV_W {
        WDV_W { w: self }
    }
    #[doc = "Bit 12 - Watchdog Fault Interrupt Enable"]
    #[inline(always)]
    pub fn wdfien(&mut self) -> WDFIEN_W {
        WDFIEN_W { w: self }
    }
    #[doc = "Bit 13 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdrsten(&mut self) -> WDRSTEN_W {
        WDRSTEN_W { w: self }
    }
    #[doc = "Bit 14 - Watchdog Reset Processor"]
    #[inline(always)]
    pub fn wdrproc(&mut self) -> WDRPROC_W {
        WDRPROC_W { w: self }
    }
    #[doc = "Bit 15 - Watchdog Disable"]
    #[inline(always)]
    pub fn wddis(&mut self) -> WDDIS_W {
        WDDIS_W { w: self }
    }
    #[doc = "Bits 16:27 - Watchdog Delta Value"]
    #[inline(always)]
    pub fn wdd(&mut self) -> WDD_W {
        WDD_W { w: self }
    }
    #[doc = "Bit 28 - Watchdog Debug Halt"]
    #[inline(always)]
    pub fn wddbghlt(&mut self) -> WDDBGHLT_W {
        WDDBGHLT_W { w: self }
    }
    #[doc = "Bit 29 - Watchdog Idle Halt"]
    #[inline(always)]
    pub fn wdidlehlt(&mut self) -> WDIDLEHLT_W {
        WDIDLEHLT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0x3fff_2fff"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff_2fff
    }
}
