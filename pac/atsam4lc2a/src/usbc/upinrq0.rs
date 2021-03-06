#[doc = "Register `UPINRQ0` reader"]
pub struct R(crate::R<UPINRQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPINRQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPINRQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPINRQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPINRQ0` writer"]
pub struct W(crate::W<UPINRQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPINRQ0_SPEC>;
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
impl From<crate::W<UPINRQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPINRQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub struct INRQ_R(crate::FieldReader<u8, u8>);
impl INRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        INRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub struct INRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub struct INMODE_R(crate::FieldReader<bool, bool>);
impl INMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub struct INMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> INMODE_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> INMODE_R {
        INMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&mut self) -> INRQ_W {
        INRQ_W { w: self }
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&mut self) -> INMODE_W {
        INMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq0](index.html) module"]
pub struct UPINRQ0_SPEC;
impl crate::RegisterSpec for UPINRQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upinrq0::R](R) reader structure"]
impl crate::Readable for UPINRQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upinrq0::W](W) writer structure"]
impl crate::Writable for UPINRQ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPINRQ0 to value 0x01"]
impl crate::Resettable for UPINRQ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
