#[doc = "Register `VCR0` reader"]
pub struct R(crate::R<VCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCR0` writer"]
pub struct W(crate::W<VCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCR0_SPEC>;
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
impl From<crate::W<VCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOLUME` reader - Volume Control"]
pub type VOLUME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VOLUME` writer - Volume Control"]
pub type VOLUME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCR0_SPEC, u16, u16, 15, O>;
#[doc = "Field `MUTE` reader - Mute"]
pub type MUTE_R = crate::BitReader<bool>;
#[doc = "Field `MUTE` writer - Mute"]
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Volume Control"]
    #[inline(always)]
    pub fn volume(&self) -> VOLUME_R {
        VOLUME_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Mute"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Volume Control"]
    #[inline(always)]
    #[must_use]
    pub fn volume(&mut self) -> VOLUME_W<0> {
        VOLUME_W::new(self)
    }
    #[doc = "Bit 31 - Mute"]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<31> {
        MUTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Volume Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcr0](index.html) module"]
pub struct VCR0_SPEC;
impl crate::RegisterSpec for VCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vcr0::R](R) reader structure"]
impl crate::Readable for VCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vcr0::W](W) writer structure"]
impl crate::Writable for VCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VCR0 to value 0"]
impl crate::Resettable for VCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
