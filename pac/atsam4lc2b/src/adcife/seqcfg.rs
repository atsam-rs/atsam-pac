#[doc = "Register `SEQCFG` reader"]
pub struct R(crate::R<SEQCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCFG` writer"]
pub struct W(crate::W<SEQCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCFG_SPEC>;
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
impl From<crate::W<SEQCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWLA` reader - Half word left adjust"]
pub type HWLA_R = crate::BitReader<bool>;
#[doc = "Field `HWLA` writer - Half word left adjust"]
pub type HWLA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCFG_SPEC, bool, O>;
#[doc = "Field `BIPOLAR` reader - Bipolar Mode"]
pub type BIPOLAR_R = crate::BitReader<bool>;
#[doc = "Field `BIPOLAR` writer - Bipolar Mode"]
pub type BIPOLAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCFG_SPEC, bool, O>;
#[doc = "Field `GAIN` reader - Gain factor"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - Gain factor"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `GCOMP` reader - Gain Compensation"]
pub type GCOMP_R = crate::BitReader<bool>;
#[doc = "Field `GCOMP` writer - Gain Compensation"]
pub type GCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCFG_SPEC, bool, O>;
#[doc = "Field `TRGSEL` reader - Trigger selection"]
pub type TRGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGSEL` writer - Trigger selection"]
pub type TRGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `RES` reader - Resolution"]
pub type RES_R = crate::BitReader<bool>;
#[doc = "Field `RES` writer - Resolution"]
pub type RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCFG_SPEC, bool, O>;
#[doc = "Field `INTERNAL` reader - Internal Voltage Source Selection"]
pub type INTERNAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTERNAL` writer - Internal Voltage Source Selection"]
pub type INTERNAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `MUXPOS` reader - MUX selection on Positive ADC input channel"]
pub type MUXPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUXPOS` writer - MUX selection on Positive ADC input channel"]
pub type MUXPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `MUXNEG` reader - MUX selection on Negative ADC input channel"]
pub type MUXNEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUXNEG` writer - MUX selection on Negative ADC input channel"]
pub type MUXNEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ZOOMRANGE` reader - Zoom shift/unipolar reference source selection"]
pub type ZOOMRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZOOMRANGE` writer - Zoom shift/unipolar reference source selection"]
pub type ZOOMRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Half word left adjust"]
    #[inline(always)]
    pub fn hwla(&self) -> HWLA_R {
        HWLA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Bipolar Mode"]
    #[inline(always)]
    pub fn bipolar(&self) -> BIPOLAR_R {
        BIPOLAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Gain factor"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Gain Compensation"]
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Trigger selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Internal Voltage Source Selection"]
    #[inline(always)]
    pub fn internal(&self) -> INTERNAL_R {
        INTERNAL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - MUX selection on Positive ADC input channel"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - MUX selection on Negative ADC input channel"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Zoom shift/unipolar reference source selection"]
    #[inline(always)]
    pub fn zoomrange(&self) -> ZOOMRANGE_R {
        ZOOMRANGE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Half word left adjust"]
    #[inline(always)]
    #[must_use]
    pub fn hwla(&mut self) -> HWLA_W<0> {
        HWLA_W::new(self)
    }
    #[doc = "Bit 2 - Bipolar Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bipolar(&mut self) -> BIPOLAR_W<2> {
        BIPOLAR_W::new(self)
    }
    #[doc = "Bits 4:6 - Gain factor"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<4> {
        GAIN_W::new(self)
    }
    #[doc = "Bit 7 - Gain Compensation"]
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<7> {
        GCOMP_W::new(self)
    }
    #[doc = "Bits 8:10 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<8> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bit 12 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<12> {
        RES_W::new(self)
    }
    #[doc = "Bits 14:15 - Internal Voltage Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn internal(&mut self) -> INTERNAL_W<14> {
        INTERNAL_W::new(self)
    }
    #[doc = "Bits 16:19 - MUX selection on Positive ADC input channel"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<16> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bits 20:22 - MUX selection on Negative ADC input channel"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<20> {
        MUXNEG_W::new(self)
    }
    #[doc = "Bits 28:30 - Zoom shift/unipolar reference source selection"]
    #[inline(always)]
    #[must_use]
    pub fn zoomrange(&mut self) -> ZOOMRANGE_W<28> {
        ZOOMRANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqcfg](index.html) module"]
pub struct SEQCFG_SPEC;
impl crate::RegisterSpec for SEQCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqcfg::R](R) reader structure"]
impl crate::Readable for SEQCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqcfg::W](W) writer structure"]
impl crate::Writable for SEQCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCFG to value 0"]
impl crate::Resettable for SEQCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
