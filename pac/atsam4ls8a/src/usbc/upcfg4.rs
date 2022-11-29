#[doc = "Register `UPCFG4` reader"]
pub struct R(crate::R<UPCFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPCFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPCFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPCFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPCFG4` writer"]
pub struct W(crate::W<UPCFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPCFG4_SPEC>;
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
impl From<crate::W<UPCFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPCFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBK` reader - Pipe Banks"]
pub type PBK_R = crate::BitReader<PBKSELECT_A>;
#[doc = "Pipe Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBKSELECT_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DOUBLE = 1,
}
impl From<PBKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PBKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBKSELECT_A {
        match self.bits {
            false => PBKSELECT_A::SINGLE,
            true => PBKSELECT_A::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PBKSELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == PBKSELECT_A::DOUBLE
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub type PBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCFG4_SPEC, PBKSELECT_A, O>;
impl<'a, const O: u8> PBK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(PBKSELECT_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(PBKSELECT_A::DOUBLE)
    }
}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub type PSIZE_R = crate::FieldReader<u8, PSIZESELECT_A>;
#[doc = "Pipe Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZESELECT_A {
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
impl From<PSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZESELECT_A) -> Self {
        variant as _
    }
}
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSIZESELECT_A {
        match self.bits {
            0 => PSIZESELECT_A::_8,
            1 => PSIZESELECT_A::_16,
            2 => PSIZESELECT_A::_32,
            3 => PSIZESELECT_A::_64,
            4 => PSIZESELECT_A::_128,
            5 => PSIZESELECT_A::_256,
            6 => PSIZESELECT_A::_512,
            7 => PSIZESELECT_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PSIZESELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PSIZESELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PSIZESELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PSIZESELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PSIZESELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PSIZESELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PSIZESELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PSIZESELECT_A::_1024
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub type PSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, UPCFG4_SPEC, u8, PSIZESELECT_A, 3, O>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_32)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_64)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_128)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_256)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_512)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_1024)
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PTOKEN_R = crate::FieldReader<u8, PTOKENSELECT_A>;
#[doc = "Pipe Token\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTOKENSELECT_A {
    #[doc = "0: `0`"]
    SETUP = 0,
    #[doc = "1: `1`"]
    IN = 1,
    #[doc = "2: `10`"]
    OUT = 2,
}
impl From<PTOKENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOKENSELECT_A) -> Self {
        variant as _
    }
}
impl PTOKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTOKENSELECT_A> {
        match self.bits {
            0 => Some(PTOKENSELECT_A::SETUP),
            1 => Some(PTOKENSELECT_A::IN),
            2 => Some(PTOKENSELECT_A::OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == PTOKENSELECT_A::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == PTOKENSELECT_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == PTOKENSELECT_A::OUT
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PTOKEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UPCFG4_SPEC, u8, PTOKENSELECT_A, 2, O>;
impl<'a, const O: u8> PTOKEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKENSELECT_A::SETUP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKENSELECT_A::IN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKENSELECT_A::OUT)
    }
}
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PTYPE_R = crate::FieldReader<u8, PTYPESELECT_A>;
#[doc = "Pipe Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTYPESELECT_A {
    #[doc = "0: `0`"]
    CONTROL = 0,
    #[doc = "1: `1`"]
    ISOCHRONOUS = 1,
    #[doc = "2: `10`"]
    BULK = 2,
    #[doc = "3: `11`"]
    INTERRUPT = 3,
}
impl From<PTYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPESELECT_A) -> Self {
        variant as _
    }
}
impl PTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTYPESELECT_A {
        match self.bits {
            0 => PTYPESELECT_A::CONTROL,
            1 => PTYPESELECT_A::ISOCHRONOUS,
            2 => PTYPESELECT_A::BULK,
            3 => PTYPESELECT_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == PTYPESELECT_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == PTYPESELECT_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == PTYPESELECT_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PTYPESELECT_A::INTERRUPT
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PTYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, UPCFG4_SPEC, u8, PTYPESELECT_A, 2, O>;
impl<'a, const O: u8> PTYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::CONTROL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::BULK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::INTERRUPT)
    }
}
#[doc = "Field `PINGEN` reader - Ping Enable"]
pub type PINGEN_R = crate::BitReader<bool>;
#[doc = "Field `PINGEN` writer - Ping Enable"]
pub type PINGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPCFG4_SPEC, bool, O>;
#[doc = "Field `BINTERVAL` reader - binterval parameter"]
pub type BINTERVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BINTERVAL` writer - binterval parameter"]
pub type BINTERVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UPCFG4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 2 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PBK_R {
        PBK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&self) -> PINGEN_R {
        PINGEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - binterval parameter"]
    #[inline(always)]
    pub fn binterval(&self) -> BINTERVAL_R {
        BINTERVAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Pipe Banks"]
    #[inline(always)]
    #[must_use]
    pub fn pbk(&mut self) -> PBK_W<2> {
        PBK_W::new(self)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<4> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    #[must_use]
    pub fn ptoken(&mut self) -> PTOKEN_W<8> {
        PTOKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    #[must_use]
    pub fn ptype(&mut self) -> PTYPE_W<12> {
        PTYPE_W::new(self)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pingen(&mut self) -> PINGEN_W<20> {
        PINGEN_W::new(self)
    }
    #[doc = "Bits 24:31 - binterval parameter"]
    #[inline(always)]
    #[must_use]
    pub fn binterval(&mut self) -> BINTERVAL_W<24> {
        BINTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcfg4](index.html) module"]
pub struct UPCFG4_SPEC;
impl crate::RegisterSpec for UPCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upcfg4::R](R) reader structure"]
impl crate::Readable for UPCFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upcfg4::W](W) writer structure"]
impl crate::Writable for UPCFG4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPCFG4 to value 0"]
impl crate::Resettable for UPCFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
