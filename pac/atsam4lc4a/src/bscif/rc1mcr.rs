#[doc = "Register `RC1MCR` reader"]
pub struct R(crate::R<RC1MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC1MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC1MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC1MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC1MCR` writer"]
pub struct W(crate::W<RC1MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC1MCR_SPEC>;
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
impl From<crate::W<RC1MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC1MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOE` reader - 1MHz RC Osc Clock Output Enable"]
pub type CLKOE_R = crate::BitReader<bool>;
#[doc = "Field `CLKOE` writer - 1MHz RC Osc Clock Output Enable"]
pub type CLKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC1MCR_SPEC, bool, O>;
#[doc = "Field `FCD` reader - Flash Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - Flash Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC1MCR_SPEC, bool, O>;
#[doc = "Field `CLKCAL` reader - 1MHz RC Osc Calibration"]
pub type CLKCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCAL` writer - 1MHz RC Osc Calibration"]
pub type CLKCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC1MCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - 1MHz RC Osc Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&self) -> CLKOE_R {
        CLKOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 1MHz RC Osc Calibration"]
    #[inline(always)]
    pub fn clkcal(&self) -> CLKCAL_R {
        CLKCAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1MHz RC Osc Clock Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkoe(&mut self) -> CLKOE_W<0> {
        CLKOE_W::new(self)
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<7> {
        FCD_W::new(self)
    }
    #[doc = "Bits 8:12 - 1MHz RC Osc Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn clkcal(&mut self) -> CLKCAL_W<8> {
        CLKCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1MHz RC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc1mcr](index.html) module"]
pub struct RC1MCR_SPEC;
impl crate::RegisterSpec for RC1MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc1mcr::R](R) reader structure"]
impl crate::Readable for RC1MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc1mcr::W](W) writer structure"]
impl crate::Writable for RC1MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC1MCR to value 0x0f00"]
impl crate::Resettable for RC1MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
