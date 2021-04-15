#[doc = "Register `CONF%s` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CONF_SPEC>> for R {
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF%s` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl core::convert::From<crate::W<CONF_SPEC>> for W {
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IS` reader - Interupt Settings"]
pub struct IS_R(crate::FieldReader<u8, u8>);
impl IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IS` writer - Interupt Settings"]
pub struct IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `MODE` reader - Analog Comparator Mode"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Analog Comparator Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `INSELN` reader - Negative Input Select"]
pub struct INSELN_R(crate::FieldReader<u8, u8>);
impl INSELN_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSELN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSELN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSELN` writer - Negative Input Select"]
pub struct INSELN_W<'a> {
    w: &'a mut W,
}
impl<'a> INSELN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `EVENN` reader - Peripheral Event Enable Negative"]
pub struct EVENN_R(crate::FieldReader<bool, bool>);
impl EVENN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVENN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENN` writer - Peripheral Event Enable Negative"]
pub struct EVENN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENN_W<'a> {
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
#[doc = "Field `EVENP` reader - Peripheral Event Enable Positive"]
pub struct EVENP_R(crate::FieldReader<bool, bool>);
impl EVENP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVENP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENP` writer - Peripheral Event Enable Positive"]
pub struct EVENP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `HYS` reader - Hysteresis Voltage Value"]
pub struct HYS_R(crate::FieldReader<u8, u8>);
impl HYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYS` writer - Hysteresis Voltage Value"]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `FAST` reader - Fast Mode Enable"]
pub struct FAST_R(crate::FieldReader<bool, bool>);
impl FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST` writer - Fast Mode Enable"]
pub struct FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `ALWAYSON` reader - Always On"]
pub struct ALWAYSON_R(crate::FieldReader<bool, bool>);
impl ALWAYSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALWAYSON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALWAYSON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWAYSON` writer - Always On"]
pub struct ALWAYSON_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYSON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Interupt Settings"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Analog Comparator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Negative Input Select"]
    #[inline(always)]
    pub fn inseln(&self) -> INSELN_R {
        INSELN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Peripheral Event Enable Negative"]
    #[inline(always)]
    pub fn evenn(&self) -> EVENN_R {
        EVENN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Peripheral Event Enable Positive"]
    #[inline(always)]
    pub fn evenp(&self) -> EVENP_R {
        EVENP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Hysteresis Voltage Value"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Fast Mode Enable"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Always On"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interupt Settings"]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W {
        IS_W { w: self }
    }
    #[doc = "Bits 4:5 - Analog Comparator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Negative Input Select"]
    #[inline(always)]
    pub fn inseln(&mut self) -> INSELN_W {
        INSELN_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Event Enable Negative"]
    #[inline(always)]
    pub fn evenn(&mut self) -> EVENN_W {
        EVENN_W { w: self }
    }
    #[doc = "Bit 17 - Peripheral Event Enable Positive"]
    #[inline(always)]
    pub fn evenp(&mut self) -> EVENP_W {
        EVENP_W { w: self }
    }
    #[doc = "Bits 24:25 - Hysteresis Voltage Value"]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
    #[doc = "Bit 26 - Fast Mode Enable"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W { w: self }
    }
    #[doc = "Bit 27 - Always On"]
    #[inline(always)]
    pub fn alwayson(&mut self) -> ALWAYSON_W {
        ALWAYSON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF%s to value 0"]
impl crate::Resettable for CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
