#[doc = "Register `CKGR_MOR` reader"]
pub struct R(crate::R<CKGR_MOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_MOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_MOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_MOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_MOR` writer"]
pub struct W(crate::W<CKGR_MOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_MOR_SPEC>;
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
impl From<crate::W<CKGR_MOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_MOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSCXTEN` reader - Main Crystal Oscillator Enable"]
pub type MOSCXTEN_R = crate::BitReader<bool>;
#[doc = "Field `MOSCXTEN` writer - Main Crystal Oscillator Enable"]
pub type MOSCXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `MOSCXTBY` reader - Main Crystal Oscillator Bypass"]
pub type MOSCXTBY_R = crate::BitReader<bool>;
#[doc = "Field `MOSCXTBY` writer - Main Crystal Oscillator Bypass"]
pub type MOSCXTBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `WAITMODE` reader - Wait Mode Command"]
pub type WAITMODE_R = crate::BitReader<bool>;
#[doc = "Field `WAITMODE` writer - Wait Mode Command"]
pub type WAITMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `MOSCRCEN` reader - Main On-Chip RC Oscillator Enable"]
pub type MOSCRCEN_R = crate::BitReader<bool>;
#[doc = "Field `MOSCRCEN` writer - Main On-Chip RC Oscillator Enable"]
pub type MOSCRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `MOSCRCF` reader - Main On-Chip RC Oscillator Frequency Selection"]
pub type MOSCRCF_R = crate::FieldReader<u8, MOSCRCF_A>;
#[doc = "Main On-Chip RC Oscillator Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOSCRCF_A {
    #[doc = "0: The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    _4_MHZ = 0,
    #[doc = "1: The Fast RC Oscillator Frequency is at 8 MHz"]
    _8_MHZ = 1,
    #[doc = "2: The Fast RC Oscillator Frequency is at 12 MHz"]
    _12_MHZ = 2,
}
impl From<MOSCRCF_A> for u8 {
    #[inline(always)]
    fn from(variant: MOSCRCF_A) -> Self {
        variant as _
    }
}
impl MOSCRCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MOSCRCF_A> {
        match self.bits {
            0 => Some(MOSCRCF_A::_4_MHZ),
            1 => Some(MOSCRCF_A::_8_MHZ),
            2 => Some(MOSCRCF_A::_12_MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4_MHZ`"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == MOSCRCF_A::_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_8_MHZ`"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == MOSCRCF_A::_8_MHZ
    }
    #[doc = "Checks if the value of the field is `_12_MHZ`"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == MOSCRCF_A::_12_MHZ
    }
}
#[doc = "Field `MOSCRCF` writer - Main On-Chip RC Oscillator Frequency Selection"]
pub type MOSCRCF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CKGR_MOR_SPEC, u8, MOSCRCF_A, 3, O>;
impl<'a, const O: u8> MOSCRCF_W<'a, O> {
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_4_MHZ)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_8_MHZ)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut W {
        self.variant(MOSCRCF_A::_12_MHZ)
    }
}
#[doc = "Field `MOSCXTST` reader - Main Crystal Oscillator Start-up Time"]
pub type MOSCXTST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOSCXTST` writer - Main Crystal Oscillator Start-up Time"]
pub type MOSCXTST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_MOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `KEY` reader - Write Access Password"]
pub type KEY_R = crate::FieldReader<u8, KEY_A>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "55: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 55,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            55 => Some(KEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEY_A::PASSWD
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_MOR_SPEC, u8, KEY_A, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
}
#[doc = "Field `MOSCSEL` reader - Main Oscillator Selection"]
pub type MOSCSEL_R = crate::BitReader<bool>;
#[doc = "Field `MOSCSEL` writer - Main Oscillator Selection"]
pub type MOSCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MOSCXTEN_R {
        MOSCXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MOSCXTBY_R {
        MOSCXTBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Mode Command"]
    #[inline(always)]
    pub fn waitmode(&self) -> WAITMODE_R {
        WAITMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MOSCRCEN_R {
        MOSCRCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MOSCRCF_R {
        MOSCRCF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MOSCXTST_R {
        MOSCXTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MOSCSEL_R {
        MOSCSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxten(&mut self) -> MOSCXTEN_W<0> {
        MOSCXTEN_W::new(self)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn moscxtby(&mut self) -> MOSCXTBY_W<1> {
        MOSCXTBY_W::new(self)
    }
    #[doc = "Bit 2 - Wait Mode Command"]
    #[inline(always)]
    #[must_use]
    pub fn waitmode(&mut self) -> WAITMODE_W<2> {
        WAITMODE_W::new(self)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcen(&mut self) -> MOSCRCEN_W<3> {
        MOSCRCEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcf(&mut self) -> MOSCRCF_W<4> {
        MOSCRCF_W::new(self)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn moscxtst(&mut self) -> MOSCXTST_W<8> {
        MOSCXTST_W::new(self)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn moscsel(&mut self) -> MOSCSEL_W<24> {
        MOSCSEL_W::new(self)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<25> {
        CFDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mor](index.html) module"]
pub struct CKGR_MOR_SPEC;
impl crate::RegisterSpec for CKGR_MOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_mor::R](R) reader structure"]
impl crate::Readable for CKGR_MOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_mor::W](W) writer structure"]
impl crate::Writable for CKGR_MOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKGR_MOR to value 0x08"]
impl crate::Resettable for CKGR_MOR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
