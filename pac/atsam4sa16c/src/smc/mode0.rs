#[doc = "Register `MODE0` reader"]
pub struct R(crate::R<MODE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE0` writer"]
pub struct W(crate::W<MODE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE0_SPEC>;
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
impl From<crate::W<MODE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_MODE` reader - "]
pub type READ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `READ_MODE` writer - "]
pub type READ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, bool, O>;
#[doc = "Field `WRITE_MODE` reader - "]
pub type WRITE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_MODE` writer - "]
pub type WRITE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, bool, O>;
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type EXNW_MODE_R = crate::FieldReader<u8, EXNW_MODE_A>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Frozen Mode"]
    FROZEN = 2,
    #[doc = "3: Ready Mode"]
    READY = 3,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        variant as _
    }
}
impl EXNW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXNW_MODE_A> {
        match self.bits {
            0 => Some(EXNW_MODE_A::DISABLED),
            2 => Some(EXNW_MODE_A::FROZEN),
            3 => Some(EXNW_MODE_A::READY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODE_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODE_A::READY
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type EXNW_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MODE0_SPEC, u8, EXNW_MODE_A, 2, O>;
impl<'a, const O: u8> EXNW_MODE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::READY)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TDF_CYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TDF_CYCLES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TDF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TDF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, bool, O>;
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub type PMEN_R = crate::BitReader<bool>;
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub type PMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE0_SPEC, bool, O>;
#[doc = "Field `PS` reader - Page Size"]
pub type PS_R = crate::FieldReader<u8, PS_A>;
#[doc = "Page Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: 4-byte page"]
    _4_BYTE = 0,
    #[doc = "1: 8-byte page"]
    _8_BYTE = 1,
    #[doc = "2: 16-byte page"]
    _16_BYTE = 2,
    #[doc = "3: 32-byte page"]
    _32_BYTE = 3,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_4_BYTE,
            1 => PS_A::_8_BYTE,
            2 => PS_A::_16_BYTE,
            3 => PS_A::_32_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == PS_A::_4_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PS_A::_32_BYTE
    }
}
#[doc = "Field `PS` writer - Page Size"]
pub type PS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MODE0_SPEC, u8, PS_A, 2, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(PS_A::_4_BYTE)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PS_A::_8_BYTE)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PS_A::_16_BYTE)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PS_A::_32_BYTE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> READ_MODE_W<0> {
        READ_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn write_mode(&mut self) -> WRITE_MODE_W<1> {
        WRITE_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W<4> {
        EXNW_MODE_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W<16> {
        TDF_CYCLES_W::new(self)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W<20> {
        TDF_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pmen(&mut self) -> PMEN_W<24> {
        PMEN_W::new(self)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<28> {
        PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Mode Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode0](index.html) module"]
pub struct MODE0_SPEC;
impl crate::RegisterSpec for MODE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode0::R](R) reader structure"]
impl crate::Readable for MODE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode0::W](W) writer structure"]
impl crate::Writable for MODE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE0 to value 0x1000_0003"]
impl crate::Resettable for MODE0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0003;
}
