#[doc = "Register `BCFG` reader"]
pub struct R(crate::R<BCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCFG` writer"]
pub struct W(crate::W<BCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCFG_SPEC>;
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
impl From<crate::W<BCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Blinking Mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Blinking Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCFG_SPEC, bool, O>;
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub type FCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BSS0` reader - Blink Segment Selection 0"]
pub type BSS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSS0` writer - Blink Segment Selection 0"]
pub type BSS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `BSS1` reader - Blink Segment Selection 1"]
pub type BSS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSS1` writer - Blink Segment Selection 1"]
pub type BSS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Blink Segment Selection 0"]
    #[inline(always)]
    pub fn bss0(&self) -> BSS0_R {
        BSS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Blink Segment Selection 1"]
    #[inline(always)]
    pub fn bss1(&self) -> BSS1_R {
        BSS1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<1> {
        FCS_W::new(self)
    }
    #[doc = "Bits 8:11 - Blink Segment Selection 0"]
    #[inline(always)]
    #[must_use]
    pub fn bss0(&mut self) -> BSS0_W<8> {
        BSS0_W::new(self)
    }
    #[doc = "Bits 12:15 - Blink Segment Selection 1"]
    #[inline(always)]
    #[must_use]
    pub fn bss1(&mut self) -> BSS1_W<12> {
        BSS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Blink Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfg](index.html) module"]
pub struct BCFG_SPEC;
impl crate::RegisterSpec for BCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcfg::R](R) reader structure"]
impl crate::Readable for BCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcfg::W](W) writer structure"]
impl crate::Writable for BCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCFG to value 0"]
impl crate::Resettable for BCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
