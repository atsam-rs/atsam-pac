#[doc = "Register `PBCSEL` reader"]
pub struct R(crate::R<PBCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBCSEL` writer"]
pub struct W(crate::W<PBCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBCSEL_SPEC>;
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
impl From<crate::W<PBCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBSEL` reader - PBC Clock Select"]
pub struct PBSEL_R(crate::FieldReader<u8, u8>);
impl PBSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBSEL` writer - PBC Clock Select"]
pub struct PBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `PBDIV` reader - PBC Division Select"]
pub struct PBDIV_R(crate::FieldReader<bool, bool>);
impl PBDIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBDIV` writer - PBC Division Select"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PBC Clock Select"]
    #[inline(always)]
    pub fn pbsel(&self) -> PBSEL_R {
        PBSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - PBC Division Select"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PBC Clock Select"]
    #[inline(always)]
    pub fn pbsel(&mut self) -> PBSEL_W {
        PBSEL_W { w: self }
    }
    #[doc = "Bit 7 - PBC Division Select"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBC Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcsel](index.html) module"]
pub struct PBCSEL_SPEC;
impl crate::RegisterSpec for PBCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbcsel::R](R) reader structure"]
impl crate::Readable for PBCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbcsel::W](W) writer structure"]
impl crate::Writable for PBCSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBCSEL to value 0"]
impl crate::Resettable for PBCSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
