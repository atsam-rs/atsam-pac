#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - WDT Enable"]
pub type EN_R = crate::BitReader<ENSELECT_A>;
#[doc = "WDT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSELECT_A {
    #[doc = "0: WDT is disabled."]
    _0 = 0,
    #[doc = "1: WDT is enabled"]
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
#[doc = "Field `EN` writer - WDT Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ENSELECT_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "WDT is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENSELECT_A::_0)
    }
    #[doc = "WDT is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENSELECT_A::_1)
    }
}
#[doc = "Field `DAR` reader - WDT Disable After Reset"]
pub type DAR_R = crate::BitReader<bool>;
#[doc = "Field `DAR` writer - WDT Disable After Reset"]
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - WDT Mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - WDT Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SFV` reader - WDT Store Final Value"]
pub type SFV_R = crate::BitReader<bool>;
#[doc = "Field `SFV` writer - WDT Store Final Value"]
pub type SFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IM` reader - WDT Interruput Mode"]
pub type IM_R = crate::BitReader<bool>;
#[doc = "Field `IM` writer - WDT Interruput Mode"]
pub type IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FCD` reader - WDT Fuse Calibration Done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - WDT Fuse Calibration Done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PSEL` reader - Timeout Prescale Select"]
pub type PSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEL` writer - Timeout Prescale Select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CSSEL1` reader - Clock Source Selection1"]
pub type CSSEL1_R = crate::BitReader<bool>;
#[doc = "Field `CSSEL1` writer - Clock Source Selection1"]
pub type CSSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CSSEL` reader - Clock Source Selection0"]
pub type CSSEL_R = crate::BitReader<bool>;
#[doc = "Field `CSSEL` writer - Clock Source Selection0"]
pub type CSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TBAN` reader - TBAN Prescale Select"]
pub type TBAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBAN` writer - TBAN Prescale Select"]
pub type TBAN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `KEY` reader - Key"]
pub type KEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY` writer - Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - WDT Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Disable After Reset"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDT Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT Interruput Mode"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - WDT Fuse Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Timeout Prescale Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Clock Source Selection1"]
    #[inline(always)]
    pub fn cssel1(&self) -> CSSEL1_R {
        CSSEL1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock Source Selection0"]
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - TBAN Prescale Select"]
    #[inline(always)]
    pub fn tban(&self) -> TBAN_R {
        TBAN_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - WDT Disable After Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<1> {
        DAR_W::new(self)
    }
    #[doc = "Bit 2 - WDT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - WDT Store Final Value"]
    #[inline(always)]
    #[must_use]
    pub fn sfv(&mut self) -> SFV_W<3> {
        SFV_W::new(self)
    }
    #[doc = "Bit 4 - WDT Interruput Mode"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<4> {
        IM_W::new(self)
    }
    #[doc = "Bit 7 - WDT Fuse Calibration Done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<7> {
        FCD_W::new(self)
    }
    #[doc = "Bits 8:12 - Timeout Prescale Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<8> {
        PSEL_W::new(self)
    }
    #[doc = "Bit 14 - Clock Source Selection1"]
    #[inline(always)]
    #[must_use]
    pub fn cssel1(&mut self) -> CSSEL1_W<14> {
        CSSEL1_W::new(self)
    }
    #[doc = "Bit 16 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<16> {
        CEN_W::new(self)
    }
    #[doc = "Bit 17 - Clock Source Selection0"]
    #[inline(always)]
    #[must_use]
    pub fn cssel(&mut self) -> CSSEL_W<17> {
        CSSEL_W::new(self)
    }
    #[doc = "Bits 18:22 - TBAN Prescale Select"]
    #[inline(always)]
    #[must_use]
    pub fn tban(&mut self) -> TBAN_W<18> {
        TBAN_W::new(self)
    }
    #[doc = "Bits 24:31 - Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0001_0080"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0080;
}
