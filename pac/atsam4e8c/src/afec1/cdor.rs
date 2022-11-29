#[doc = "Register `CDOR` reader"]
pub struct R(crate::R<CDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDOR` writer"]
pub struct W(crate::W<CDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDOR_SPEC>;
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
impl From<crate::W<CDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFF0` reader - Offset for Channel 0, used in Automatic Calibration Procedure"]
pub type OFF0_R = crate::BitReader<bool>;
#[doc = "Field `OFF0` writer - Offset for Channel 0, used in Automatic Calibration Procedure"]
pub type OFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF1` reader - Offset for Channel 1, used in Automatic Calibration Procedure"]
pub type OFF1_R = crate::BitReader<bool>;
#[doc = "Field `OFF1` writer - Offset for Channel 1, used in Automatic Calibration Procedure"]
pub type OFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF2` reader - Offset for Channel 2, used in Automatic Calibration Procedure"]
pub type OFF2_R = crate::BitReader<bool>;
#[doc = "Field `OFF2` writer - Offset for Channel 2, used in Automatic Calibration Procedure"]
pub type OFF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF3` reader - Offset for Channel 3, used in Automatic Calibration Procedure"]
pub type OFF3_R = crate::BitReader<bool>;
#[doc = "Field `OFF3` writer - Offset for Channel 3, used in Automatic Calibration Procedure"]
pub type OFF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF4` reader - Offset for Channel 4, used in Automatic Calibration Procedure"]
pub type OFF4_R = crate::BitReader<bool>;
#[doc = "Field `OFF4` writer - Offset for Channel 4, used in Automatic Calibration Procedure"]
pub type OFF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF5` reader - Offset for Channel 5, used in Automatic Calibration Procedure"]
pub type OFF5_R = crate::BitReader<bool>;
#[doc = "Field `OFF5` writer - Offset for Channel 5, used in Automatic Calibration Procedure"]
pub type OFF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF6` reader - Offset for Channel 6, used in Automatic Calibration Procedure"]
pub type OFF6_R = crate::BitReader<bool>;
#[doc = "Field `OFF6` writer - Offset for Channel 6, used in Automatic Calibration Procedure"]
pub type OFF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF7` reader - Offset for Channel 7, used in Automatic Calibration Procedure"]
pub type OFF7_R = crate::BitReader<bool>;
#[doc = "Field `OFF7` writer - Offset for Channel 7, used in Automatic Calibration Procedure"]
pub type OFF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF8` reader - Offset for Channel 8, used in Automatic Calibration Procedure"]
pub type OFF8_R = crate::BitReader<bool>;
#[doc = "Field `OFF8` writer - Offset for Channel 8, used in Automatic Calibration Procedure"]
pub type OFF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF9` reader - Offset for Channel 9, used in Automatic Calibration Procedure"]
pub type OFF9_R = crate::BitReader<bool>;
#[doc = "Field `OFF9` writer - Offset for Channel 9, used in Automatic Calibration Procedure"]
pub type OFF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF10` reader - Offset for Channel 10, used in Automatic Calibration Procedure"]
pub type OFF10_R = crate::BitReader<bool>;
#[doc = "Field `OFF10` writer - Offset for Channel 10, used in Automatic Calibration Procedure"]
pub type OFF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF11` reader - Offset for Channel 11, used in Automatic Calibration Procedure"]
pub type OFF11_R = crate::BitReader<bool>;
#[doc = "Field `OFF11` writer - Offset for Channel 11, used in Automatic Calibration Procedure"]
pub type OFF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF12` reader - Offset for Channel 12, used in Automatic Calibration Procedure"]
pub type OFF12_R = crate::BitReader<bool>;
#[doc = "Field `OFF12` writer - Offset for Channel 12, used in Automatic Calibration Procedure"]
pub type OFF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF13` reader - Offset for Channel 13, used in Automatic Calibration Procedure"]
pub type OFF13_R = crate::BitReader<bool>;
#[doc = "Field `OFF13` writer - Offset for Channel 13, used in Automatic Calibration Procedure"]
pub type OFF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF14` reader - Offset for Channel 14, used in Automatic Calibration Procedure"]
pub type OFF14_R = crate::BitReader<bool>;
#[doc = "Field `OFF14` writer - Offset for Channel 14, used in Automatic Calibration Procedure"]
pub type OFF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
#[doc = "Field `OFF15` reader - Offset for Channel 15, used in Automatic Calibration Procedure"]
pub type OFF15_R = crate::BitReader<bool>;
#[doc = "Field `OFF15` writer - Offset for Channel 15, used in Automatic Calibration Procedure"]
pub type OFF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Offset for Channel 0, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off0(&self) -> OFF0_R {
        OFF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Offset for Channel 1, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off1(&self) -> OFF1_R {
        OFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Offset for Channel 2, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off2(&self) -> OFF2_R {
        OFF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Offset for Channel 3, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off3(&self) -> OFF3_R {
        OFF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Offset for Channel 4, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off4(&self) -> OFF4_R {
        OFF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Offset for Channel 5, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off5(&self) -> OFF5_R {
        OFF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Offset for Channel 6, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off6(&self) -> OFF6_R {
        OFF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Offset for Channel 7, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off7(&self) -> OFF7_R {
        OFF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Offset for Channel 8, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off8(&self) -> OFF8_R {
        OFF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Offset for Channel 9, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off9(&self) -> OFF9_R {
        OFF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Offset for Channel 10, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off10(&self) -> OFF10_R {
        OFF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Offset for Channel 11, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off11(&self) -> OFF11_R {
        OFF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Offset for Channel 12, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off12(&self) -> OFF12_R {
        OFF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Offset for Channel 13, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off13(&self) -> OFF13_R {
        OFF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Offset for Channel 14, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off14(&self) -> OFF14_R {
        OFF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Offset for Channel 15, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off15(&self) -> OFF15_R {
        OFF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Offset for Channel 0, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off0(&mut self) -> OFF0_W<0> {
        OFF0_W::new(self)
    }
    #[doc = "Bit 1 - Offset for Channel 1, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off1(&mut self) -> OFF1_W<1> {
        OFF1_W::new(self)
    }
    #[doc = "Bit 2 - Offset for Channel 2, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off2(&mut self) -> OFF2_W<2> {
        OFF2_W::new(self)
    }
    #[doc = "Bit 3 - Offset for Channel 3, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off3(&mut self) -> OFF3_W<3> {
        OFF3_W::new(self)
    }
    #[doc = "Bit 4 - Offset for Channel 4, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off4(&mut self) -> OFF4_W<4> {
        OFF4_W::new(self)
    }
    #[doc = "Bit 5 - Offset for Channel 5, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off5(&mut self) -> OFF5_W<5> {
        OFF5_W::new(self)
    }
    #[doc = "Bit 6 - Offset for Channel 6, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off6(&mut self) -> OFF6_W<6> {
        OFF6_W::new(self)
    }
    #[doc = "Bit 7 - Offset for Channel 7, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off7(&mut self) -> OFF7_W<7> {
        OFF7_W::new(self)
    }
    #[doc = "Bit 8 - Offset for Channel 8, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off8(&mut self) -> OFF8_W<8> {
        OFF8_W::new(self)
    }
    #[doc = "Bit 9 - Offset for Channel 9, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off9(&mut self) -> OFF9_W<9> {
        OFF9_W::new(self)
    }
    #[doc = "Bit 10 - Offset for Channel 10, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off10(&mut self) -> OFF10_W<10> {
        OFF10_W::new(self)
    }
    #[doc = "Bit 11 - Offset for Channel 11, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off11(&mut self) -> OFF11_W<11> {
        OFF11_W::new(self)
    }
    #[doc = "Bit 12 - Offset for Channel 12, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off12(&mut self) -> OFF12_W<12> {
        OFF12_W::new(self)
    }
    #[doc = "Bit 13 - Offset for Channel 13, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off13(&mut self) -> OFF13_W<13> {
        OFF13_W::new(self)
    }
    #[doc = "Bit 14 - Offset for Channel 14, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off14(&mut self) -> OFF14_W<14> {
        OFF14_W::new(self)
    }
    #[doc = "Bit 15 - Offset for Channel 15, used in Automatic Calibration Procedure"]
    #[inline(always)]
    #[must_use]
    pub fn off15(&mut self) -> OFF15_W<15> {
        OFF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Calibration DC Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdor](index.html) module"]
pub struct CDOR_SPEC;
impl crate::RegisterSpec for CDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdor::R](R) reader structure"]
impl crate::Readable for CDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdor::W](W) writer structure"]
impl crate::Writable for CDOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDOR to value 0"]
impl crate::Resettable for CDOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
