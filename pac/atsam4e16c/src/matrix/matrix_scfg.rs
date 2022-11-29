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
pub type SLOT_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Bus Grant Duration for Masters"]
pub type SLOT_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u16, u16, 9, O>;
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DEFMSTR_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DEFMSTR_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FIXED_DEFMSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FIXED_DEFMSTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:8 - Maximum Bus Grant Duration for Masters"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 3) as u8)
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
    #[must_use]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W<0> {
        SLOT_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    #[must_use]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W<16> {
        DEFMSTR_TYPE_W::new(self)
    }
    #[doc = "Bits 18:21 - Fixed Default Master"]
    #[inline(always)]
    #[must_use]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W<18> {
        FIXED_DEFMSTR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
