#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader<TRGEN_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger mode disabled. DACC in free running mode."]
    FREE_RUN = 0,
    #[doc = "1: External trigger mode enabled."]
    EXT_TRIG = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::FREE_RUN,
            true => TRGEN_A::EXT_TRIG,
        }
    }
    #[doc = "Checks if the value of the field is `FREE_RUN`"]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == TRGEN_A::FREE_RUN
    }
    #[doc = "Checks if the value of the field is `EXT_TRIG`"]
    #[inline(always)]
    pub fn is_ext_trig(&self) -> bool {
        *self == TRGEN_A::EXT_TRIG
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TRGEN_A, O>;
impl<'a, const O: u8> TRGEN_W<'a, O> {
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut W {
        self.variant(TRGEN_A::FREE_RUN)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn ext_trig(self) -> &'a mut W {
        self.variant(TRGEN_A::EXT_TRIG)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader<u8, TRGSEL_A>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: External trigger"]
    TRGSEL0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    TRGSEL1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    TRGSEL2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    TRGSEL3 = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            0 => Some(TRGSEL_A::TRGSEL0),
            1 => Some(TRGSEL_A::TRGSEL1),
            2 => Some(TRGSEL_A::TRGSEL2),
            3 => Some(TRGSEL_A::TRGSEL3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL_A::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL_A::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL_A::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL_A::TRGSEL3
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, TRGSEL_A, 3, O>;
impl<'a, const O: u8> TRGSEL_W<'a, O> {
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL3)
    }
}
#[doc = "Field `DACEN` reader - DAC enable"]
pub type DACEN_R = crate::BitReader<DACEN_A>;
#[doc = "DAC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACEN_A {
    #[doc = "0: DAC disabled."]
    DISABLED = 0,
    #[doc = "1: DAC enabled."]
    ENABLED = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::DISABLED,
            true => DACEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DACEN_A::ENABLED
    }
}
#[doc = "Field `DACEN` writer - DAC enable"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, DACEN_A, O>;
impl<'a, const O: u8> DACEN_W<'a, O> {
    #[doc = "DAC disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACEN_A::DISABLED)
    }
    #[doc = "DAC enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACEN_A::ENABLED)
    }
}
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WORD_R = crate::BitReader<WORD_A>;
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WORD_A {
    #[doc = "0: Half-Word transfer."]
    HALF_WORD = 0,
    #[doc = "1: Word Transfer."]
    WORD = 1,
}
impl From<WORD_A> for bool {
    #[inline(always)]
    fn from(variant: WORD_A) -> Self {
        variant as u8 != 0
    }
}
impl WORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_A {
        match self.bits {
            false => WORD_A::HALF_WORD,
            true => WORD_A::WORD,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == WORD_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == WORD_A::WORD
    }
}
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, WORD_A, O>;
impl<'a, const O: u8> WORD_W<'a, O> {
    #[doc = "Half-Word transfer."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(WORD_A::HALF_WORD)
    }
    #[doc = "Word Transfer."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(WORD_A::WORD)
    }
}
#[doc = "Field `STARTUP` reader - Start-up Time Selection"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Start-up Time Selection"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKDIV` reader - DAC Clock Divider for Internal Trigger"]
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKDIV` writer - DAC Clock Divider for Internal Trigger"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - DAC enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Start-up Time Selection"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - DAC Clock Divider for Internal Trigger"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<0> {
        TRGEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<1> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bit 4 - DAC enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<4> {
        DACEN_W::new(self)
    }
    #[doc = "Bit 5 - Word Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WORD_W<5> {
        WORD_W::new(self)
    }
    #[doc = "Bits 8:15 - Start-up Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bits 16:31 - DAC Clock Divider for Internal Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<16> {
        CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
