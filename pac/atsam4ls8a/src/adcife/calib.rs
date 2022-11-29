#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIB` writer"]
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIB` reader - Calibration Value"]
pub type CALIB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB` writer - Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIB_SPEC, u8, u8, 8, O>;
#[doc = "Field `BIASSEL` reader - Select bias mode"]
pub type BIASSEL_R = crate::BitReader<bool>;
#[doc = "Field `BIASSEL` writer - Select bias mode"]
pub type BIASSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
#[doc = "Field `BIASCAL` reader - Bias Calibration"]
pub type BIASCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASCAL` writer - Bias Calibration"]
pub type BIASCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIB_SPEC, u8, u8, 4, O>;
#[doc = "Field `FCD` reader - Flash Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - Flash Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Select bias mode"]
    #[inline(always)]
    pub fn biassel(&self) -> BIASSEL_R {
        BIASSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Bias Calibration"]
    #[inline(always)]
    pub fn biascal(&self) -> BIASCAL_R {
        BIASCAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<0> {
        CALIB_W::new(self)
    }
    #[doc = "Bit 8 - Select bias mode"]
    #[inline(always)]
    #[must_use]
    pub fn biassel(&mut self) -> BIASSEL_W<8> {
        BIASSEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Bias Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn biascal(&mut self) -> BIASCAL_W<12> {
        BIASCAL_W::new(self)
    }
    #[doc = "Bit 16 - Flash Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<16> {
        FCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calib::W](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
