#[doc = "Register `IADR` writer"]
pub struct W(crate::W<IADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IADR_SPEC>;
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
impl From<crate::W<IADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Segments Value"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DMASK` writer - Data Mask"]
pub struct DMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `OFF` writer - Byte Offset"]
pub struct OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Segments Value"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Mask"]
    #[inline(always)]
    pub fn dmask(&mut self) -> DMASK_W {
        DMASK_W { w: self }
    }
    #[doc = "Bits 16:20 - Byte Offset"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W {
        OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Access Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadr](index.html) module"]
pub struct IADR_SPEC;
impl crate::RegisterSpec for IADR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iadr::W](W) writer structure"]
impl crate::Writable for IADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IADR to value 0"]
impl crate::Resettable for IADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
