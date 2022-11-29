#[doc = "Register `UECFG6` reader"]
pub struct R(crate::R<UECFG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UECFG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UECFG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UECFG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UECFG6` writer"]
pub struct W(crate::W<UECFG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECFG6_SPEC>;
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
impl From<crate::W<UECFG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECFG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPBK` reader - Endpoint Bank"]
pub type EPBK_R = crate::BitReader<EPBKSELECT_A>;
#[doc = "Endpoint Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPBKSELECT_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DOUBLE = 1,
}
impl From<EPBKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: EPBKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl EPBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPBKSELECT_A {
        match self.bits {
            false => EPBKSELECT_A::SINGLE,
            true => EPBKSELECT_A::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == EPBKSELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == EPBKSELECT_A::DOUBLE
    }
}
#[doc = "Field `EPBK` writer - Endpoint Bank"]
pub type EPBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECFG6_SPEC, EPBKSELECT_A, O>;
impl<'a, const O: u8> EPBK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(EPBKSELECT_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(EPBKSELECT_A::DOUBLE)
    }
}
#[doc = "Field `EPSIZE` reader - Endpoint Size"]
pub type EPSIZE_R = crate::FieldReader<u8, EPSIZESELECT_A>;
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPSIZESELECT_A {
    #[doc = "0: `0`"]
    _8 = 0,
    #[doc = "1: `1`"]
    _16 = 1,
    #[doc = "2: `10`"]
    _32 = 2,
    #[doc = "3: `11`"]
    _64 = 3,
    #[doc = "4: `100`"]
    _128 = 4,
    #[doc = "5: `101`"]
    _256 = 5,
    #[doc = "6: `110`"]
    _512 = 6,
    #[doc = "7: `111`"]
    _1024 = 7,
}
impl From<EPSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSIZESELECT_A) -> Self {
        variant as _
    }
}
impl EPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSIZESELECT_A {
        match self.bits {
            0 => EPSIZESELECT_A::_8,
            1 => EPSIZESELECT_A::_16,
            2 => EPSIZESELECT_A::_32,
            3 => EPSIZESELECT_A::_64,
            4 => EPSIZESELECT_A::_128,
            5 => EPSIZESELECT_A::_256,
            6 => EPSIZESELECT_A::_512,
            7 => EPSIZESELECT_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == EPSIZESELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == EPSIZESELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == EPSIZESELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == EPSIZESELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == EPSIZESELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == EPSIZESELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == EPSIZESELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == EPSIZESELECT_A::_1024
    }
}
#[doc = "Field `EPSIZE` writer - Endpoint Size"]
pub type EPSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, UECFG6_SPEC, u8, EPSIZESELECT_A, 3, O>;
impl<'a, const O: u8> EPSIZE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_32)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_64)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_128)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_256)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_512)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(EPSIZESELECT_A::_1024)
    }
}
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EPDIR_R = crate::BitReader<EPDIRSELECT_A>;
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPDIRSELECT_A {
    #[doc = "0: `0`"]
    OUT = 0,
    #[doc = "1: `1`"]
    IN = 1,
}
impl From<EPDIRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl EPDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDIRSELECT_A {
        match self.bits {
            false => EPDIRSELECT_A::OUT,
            true => EPDIRSELECT_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EPDIRSELECT_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EPDIRSELECT_A::IN
    }
}
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, UECFG6_SPEC, EPDIRSELECT_A, O>;
impl<'a, const O: u8> EPDIR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EPDIRSELECT_A::OUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EPDIRSELECT_A::IN)
    }
}
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<u8, EPTYPESELECT_A>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPESELECT_A {
    #[doc = "0: `0`"]
    CONTROL = 0,
    #[doc = "1: `1`"]
    ISOCHRONOUS = 1,
    #[doc = "2: `10`"]
    BULK = 2,
    #[doc = "3: `11`"]
    INTERRUPT = 3,
}
impl From<EPTYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPESELECT_A) -> Self {
        variant as _
    }
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPESELECT_A {
        match self.bits {
            0 => EPTYPESELECT_A::CONTROL,
            1 => EPTYPESELECT_A::ISOCHRONOUS,
            2 => EPTYPESELECT_A::BULK,
            3 => EPTYPESELECT_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EPTYPESELECT_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == EPTYPESELECT_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPESELECT_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EPTYPESELECT_A::INTERRUPT
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, UECFG6_SPEC, u8, EPTYPESELECT_A, 2, O>;
impl<'a, const O: u8> EPTYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::CONTROL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::BULK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EPTYPESELECT_A::INTERRUPT)
    }
}
#[doc = "Field `REPNB` reader - Redirected Endpoint Number"]
pub type REPNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPNB` writer - Redirected Endpoint Number"]
pub type REPNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UECFG6_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 2 - Endpoint Bank"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Redirected Endpoint Number"]
    #[inline(always)]
    pub fn repnb(&self) -> REPNB_R {
        REPNB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Endpoint Bank"]
    #[inline(always)]
    #[must_use]
    pub fn epbk(&mut self) -> EPBK_W<2> {
        EPBK_W::new(self)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    #[must_use]
    pub fn epsize(&mut self) -> EPSIZE_W<4> {
        EPSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<8> {
        EPDIR_W::new(self)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<11> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 16:19 - Redirected Endpoint Number"]
    #[inline(always)]
    #[must_use]
    pub fn repnb(&mut self) -> REPNB_W<16> {
        REPNB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecfg6](index.html) module"]
pub struct UECFG6_SPEC;
impl crate::RegisterSpec for UECFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uecfg6::R](R) reader structure"]
impl crate::Readable for UECFG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uecfg6::W](W) writer structure"]
impl crate::Writable for UECFG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECFG6 to value 0"]
impl crate::Resettable for UECFG6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
