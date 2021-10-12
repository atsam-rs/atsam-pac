#[doc = "Register `MATRIX_SCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_SCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_SCFG_SPEC>;
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
impl From<crate::W<MATRIX_SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Bus Grant Duration for Masters"]
pub struct SLOT_CYCLE_R(crate::FieldReader<u16, u16>);
impl SLOT_CYCLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        SLOT_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOT_CYCLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOT_CYCLE` writer - Maximum Bus Grant Duration for Masters"]
pub struct SLOT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub struct DEFMSTR_TYPE_R(crate::FieldReader<u8, u8>);
impl DEFMSTR_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEFMSTR_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFMSTR_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub struct DEFMSTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFMSTR_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub struct FIXED_DEFMSTR_R(crate::FieldReader<u8, u8>);
impl FIXED_DEFMSTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIXED_DEFMSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXED_DEFMSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub struct FIXED_DEFMSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_DEFMSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W {
        SLOT_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W {
        DEFMSTR_TYPE_W { w: self }
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W {
        FIXED_DEFMSTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_scfg](index.html) module"]
pub struct MATRIX_SCFG_SPEC;
impl crate::RegisterSpec for MATRIX_SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_scfg::R](R) reader structure"]
impl crate::Readable for MATRIX_SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_scfg::W](W) writer structure"]
impl crate::Writable for MATRIX_SCFG_SPEC {
    type Writer = W;
}
