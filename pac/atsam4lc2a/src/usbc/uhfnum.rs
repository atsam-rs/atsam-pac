#[doc = "Register `UHFNUM` reader"]
pub struct R(crate::R<UHFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UHFNUM_SPEC>> for R {
    fn from(reader: crate::R<UHFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UHFNUM` writer"]
pub struct W(crate::W<UHFNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHFNUM_SPEC>;
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
impl core::convert::From<crate::W<UHFNUM_SPEC>> for W {
    fn from(writer: crate::W<UHFNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub struct MFNUM_R(crate::FieldReader<u8, u8>);
impl MFNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        MFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNUM` reader - Frame Number"]
pub struct FNUM_R(crate::FieldReader<u16, u16>);
impl FNUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNUM` writer - Frame Number"]
pub struct FNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | ((value as u32 & 0x07ff) << 3);
        self.w
    }
}
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub struct FLENHIGH_R(crate::FieldReader<u8, u8>);
impl FLENHIGH_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLENHIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLENHIGH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FNUM_W {
        FNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhfnum](index.html) module"]
pub struct UHFNUM_SPEC;
impl crate::RegisterSpec for UHFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhfnum::R](R) reader structure"]
impl crate::Readable for UHFNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhfnum::W](W) writer structure"]
impl crate::Writable for UHFNUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHFNUM to value 0"]
impl crate::Resettable for UHFNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
