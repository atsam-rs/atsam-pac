#[doc = "Register `PMC_OCR` reader"]
pub struct R(crate::R<PMC_OCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_OCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_OCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_OCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_OCR` writer"]
pub struct W(crate::W<PMC_OCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_OCR_SPEC>;
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
impl From<crate::W<PMC_OCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_OCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL12` reader - RC Oscillator Calibration bits for 12 MHz"]
pub type CAL12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL12` writer - RC Oscillator Calibration bits for 12 MHz"]
pub type CAL12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_OCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL12` reader - Selection of RC Oscillator Calibration bits for 12 MHz"]
pub type SEL12_R = crate::BitReader<bool>;
#[doc = "Field `SEL12` writer - Selection of RC Oscillator Calibration bits for 12 MHz"]
pub type SEL12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_OCR_SPEC, bool, O>;
#[doc = "Field `CAL8` reader - RC Oscillator Calibration bits for 8 MHz"]
pub type CAL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL8` writer - RC Oscillator Calibration bits for 8 MHz"]
pub type CAL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_OCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL8` reader - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub type SEL8_R = crate::BitReader<bool>;
#[doc = "Field `SEL8` writer - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub type SEL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_OCR_SPEC, bool, O>;
#[doc = "Field `CAL4` reader - RC Oscillator Calibration bits for 4 MHz"]
pub type CAL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL4` writer - RC Oscillator Calibration bits for 4 MHz"]
pub type CAL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_OCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL4` reader - Selection of RC Oscillator Calibration bits for 4 MHz"]
pub type SEL4_R = crate::BitReader<bool>;
#[doc = "Field `SEL4` writer - Selection of RC Oscillator Calibration bits for 4 MHz"]
pub type SEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_OCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> CAL12_R {
        CAL12_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> CAL4_R {
        CAL4_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal12(&mut self) -> CAL12_W<0> {
        CAL12_W::new(self)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL12_W<7> {
        SEL12_W::new(self)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal8(&mut self) -> CAL8_W<8> {
        CAL8_W::new(self)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL8_W<15> {
        SEL8_W::new(self)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal4(&mut self) -> CAL4_W<16> {
        CAL4_W::new(self)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL4_W<23> {
        SEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ocr](index.html) module"]
pub struct PMC_OCR_SPEC;
impl crate::RegisterSpec for PMC_OCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_ocr::R](R) reader structure"]
impl crate::Readable for PMC_OCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_ocr::W](W) writer structure"]
impl crate::Writable for PMC_OCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC_OCR to value 0x0040_4040"]
impl crate::Resettable for PMC_OCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_4040;
}
