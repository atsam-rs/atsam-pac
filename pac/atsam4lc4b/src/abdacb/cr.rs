#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<ENSELECT_A>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSELECT_A {
    #[doc = "0: Audio DAC is disabled"]
    _0 = 0,
    #[doc = "1: Audio DAC is enabled"]
    _1 = 1,
}
impl From<ENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSELECT_A {
        match self.bits {
            false => ENSELECT_A::_0,
            true => ENSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENSELECT_A::_1
    }
}
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ENSELECT_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Audio DAC is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENSELECT_A::_0)
    }
    #[doc = "Audio DAC is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENSELECT_A::_1)
    }
}
#[doc = "Field `SWAP` reader - Swap Channels"]
pub type SWAP_R = crate::BitReader<SWAPSELECT_A>;
#[doc = "Swap Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAPSELECT_A {
    #[doc = "0: The CHANNEL0 and CHANNEL1 samples will not be swapped when writing the Audio DAC Sample Data Register (SDR)"]
    _0 = 0,
    #[doc = "1: The CHANNEL0 and CHANNEL1 samples will be swapped when writing the Audio DAC Sample Data Register (SDR)"]
    _1 = 1,
}
impl From<SWAPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWAPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAPSELECT_A {
        match self.bits {
            false => SWAPSELECT_A::_0,
            true => SWAPSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAPSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAPSELECT_A::_1
    }
}
#[doc = "Field `SWAP` writer - Swap Channels"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SWAPSELECT_A, O>;
impl<'a, const O: u8> SWAP_W<'a, O> {
    #[doc = "The CHANNEL0 and CHANNEL1 samples will not be swapped when writing the Audio DAC Sample Data Register (SDR)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWAPSELECT_A::_0)
    }
    #[doc = "The CHANNEL0 and CHANNEL1 samples will be swapped when writing the Audio DAC Sample Data Register (SDR)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWAPSELECT_A::_1)
    }
}
#[doc = "Field `ALTUPR` reader - Alternative up-sampling ratio"]
pub type ALTUPR_R = crate::BitReader<bool>;
#[doc = "Field `ALTUPR` writer - Alternative up-sampling ratio"]
pub type ALTUPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CMOC` reader - Common mode offset control"]
pub type CMOC_R = crate::BitReader<bool>;
#[doc = "Field `CMOC` writer - Common mode offset control"]
pub type CMOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MONO` reader - Mono mode"]
pub type MONO_R = crate::BitReader<bool>;
#[doc = "Field `MONO` writer - Mono mode"]
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DATAFORMAT` reader - Data word format"]
pub type DATAFORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAFORMAT` writer - Data word format"]
pub type DATAFORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS` reader - Sampling frequency"]
pub type FS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS` writer - Sampling frequency"]
pub type FS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Swap Channels"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternative up-sampling ratio"]
    #[inline(always)]
    pub fn altupr(&self) -> ALTUPR_R {
        ALTUPR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Common mode offset control"]
    #[inline(always)]
    pub fn cmoc(&self) -> CMOC_R {
        CMOC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mono mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Data word format"]
    #[inline(always)]
    pub fn dataformat(&self) -> DATAFORMAT_R {
        DATAFORMAT_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Sampling frequency"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Swap Channels"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<1> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 3 - Alternative up-sampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn altupr(&mut self) -> ALTUPR_W<3> {
        ALTUPR_W::new(self)
    }
    #[doc = "Bit 4 - Common mode offset control"]
    #[inline(always)]
    #[must_use]
    pub fn cmoc(&mut self) -> CMOC_W<4> {
        CMOC_W::new(self)
    }
    #[doc = "Bit 5 - Mono mode"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<5> {
        MONO_W::new(self)
    }
    #[doc = "Bit 7 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Bits 16:18 - Data word format"]
    #[inline(always)]
    #[must_use]
    pub fn dataformat(&mut self) -> DATAFORMAT_W<16> {
        DATAFORMAT_W::new(self)
    }
    #[doc = "Bits 24:27 - Sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<24> {
        FS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
