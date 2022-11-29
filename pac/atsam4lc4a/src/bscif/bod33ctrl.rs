#[doc = "Register `BOD33CTRL` reader"]
pub struct R(crate::R<BOD33CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33CTRL` writer"]
pub struct W(crate::W<BOD33CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33CTRL_SPEC>;
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
impl From<crate::W<BOD33CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33CTRL_SPEC, bool, O>;
#[doc = "Field `HYST` reader - BOD Hysteresis"]
pub type HYST_R = crate::BitReader<bool>;
#[doc = "Field `HYST` writer - BOD Hysteresis"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33CTRL_SPEC, bool, O>;
#[doc = "Field `ACTION` reader - Action"]
pub type ACTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTION` writer - Action"]
pub type ACTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD33CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - Operation modes"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Operation modes"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33CTRL_SPEC, bool, O>;
#[doc = "Field `FCD` reader - BOD Fuse Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - BOD Fuse Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33CTRL_SPEC, bool, O>;
#[doc = "Field `SFV` reader - BOD Control Register Store Final Value"]
pub type SFV_R = crate::BitReader<bool>;
#[doc = "Field `SFV` writer - BOD Control Register Store Final Value"]
pub type SFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Action"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Operation modes"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - BOD Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BOD Control Register Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - BOD Hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<1> {
        HYST_W::new(self)
    }
    #[doc = "Bits 8:9 - Action"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<8> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 16 - Operation modes"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    #[doc = "Bit 30 - BOD Fuse Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<30> {
        FCD_W::new(self)
    }
    #[doc = "Bit 31 - BOD Control Register Store Final Value"]
    #[inline(always)]
    #[must_use]
    pub fn sfv(&mut self) -> SFV_W<31> {
        SFV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD33 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33ctrl](index.html) module"]
pub struct BOD33CTRL_SPEC;
impl crate::RegisterSpec for BOD33CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33ctrl::R](R) reader structure"]
impl crate::Readable for BOD33CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33ctrl::W](W) writer structure"]
impl crate::Writable for BOD33CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD33CTRL to value 0"]
impl crate::Resettable for BOD33CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
