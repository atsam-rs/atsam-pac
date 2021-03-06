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
#[doc = "Pipe Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBK_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DOUBLE = 1,
}
impl From<PBK_A> for bool {
    #[inline(always)]
    fn from(variant: PBK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBK` reader - Pipe Banks"]
pub struct PBK_R(crate::FieldReader<bool, PBK_A>);
impl PBK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBK_A {
        match self.bits {
            false => PBK_A::SINGLE,
            true => PBK_A::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == PBK_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        **self == PBK_A::DOUBLE
    }
}
impl core::ops::Deref for PBK_R {
    type Target = crate::FieldReader<bool, PBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub struct PBK_W<'a> {
    w: &'a mut W,
}
impl<'a> PBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(PBK_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(PBK_A::DOUBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Pipe Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSIZE_A {
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
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub struct PSIZE_R(crate::FieldReader<u8, PSIZE_A>);
impl PSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSIZE_A {
        match self.bits {
            0 => PSIZE_A::_8,
            1 => PSIZE_A::_16,
            2 => PSIZE_A::_32,
            3 => PSIZE_A::_64,
            4 => PSIZE_A::_128,
            5 => PSIZE_A::_256,
            6 => PSIZE_A::_512,
            7 => PSIZE_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == PSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == PSIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == PSIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == PSIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == PSIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == PSIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == PSIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == PSIZE_A::_1024
    }
}
impl core::ops::Deref for PSIZE_R {
    type Target = crate::FieldReader<u8, PSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSIZE_A::_8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSIZE_A::_16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSIZE_A::_32)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSIZE_A::_64)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSIZE_A::_128)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSIZE_A::_256)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSIZE_A::_512)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSIZE_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Pipe Token\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTOKEN_A {
    #[doc = "0: `0`"]
    SETUP = 0,
    #[doc = "1: `1`"]
    IN = 1,
    #[doc = "2: `10`"]
    OUT = 2,
}
impl From<PTOKEN_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOKEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub struct PTOKEN_R(crate::FieldReader<u8, PTOKEN_A>);
impl PTOKEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTOKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTOKEN_A> {
        match self.bits {
            0 => Some(PTOKEN_A::SETUP),
            1 => Some(PTOKEN_A::IN),
            2 => Some(PTOKEN_A::OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        **self == PTOKEN_A::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == PTOKEN_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == PTOKEN_A::OUT
    }
}
impl core::ops::Deref for PTOKEN_R {
    type Target = crate::FieldReader<u8, PTOKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub struct PTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTOKEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKEN_A::SETUP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKEN_A::IN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKEN_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Pipe Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: `0`"]
    CONTROL = 0,
    #[doc = "1: `1`"]
    ISOCHRONOUS = 1,
    #[doc = "2: `10`"]
    BULK = 2,
    #[doc = "3: `11`"]
    INTERRUPT = 3,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub struct PTYPE_R(crate::FieldReader<u8, PTYPE_A>);
impl PTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTYPE_A {
        match self.bits {
            0 => PTYPE_A::CONTROL,
            1 => PTYPE_A::ISOCHRONOUS,
            2 => PTYPE_A::BULK,
            3 => PTYPE_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        **self == PTYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        **self == PTYPE_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        **self == PTYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == PTYPE_A::INTERRUPT
    }
}
impl core::ops::Deref for PTYPE_R {
    type Target = crate::FieldReader<u8, PTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(PTYPE_A::CONTROL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(PTYPE_A::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(PTYPE_A::BULK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PTYPE_A::INTERRUPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `PINGEN` reader - Ping Enable"]
pub struct PINGEN_R(crate::FieldReader<bool, bool>);
impl PINGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINGEN` writer - Ping Enable"]
pub struct PINGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINGEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `BINTERVAL` reader - binterval parameter"]
pub struct BINTERVAL_R(crate::FieldReader<u8, u8>);
impl BINTERVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BINTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BINTERVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BINTERVAL` writer - binterval parameter"]
pub struct BINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PBK_R {
        PBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&self) -> PINGEN_R {
        PINGEN_R::new(((self.bits >> 20) & 0x01) != 0)
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
    pub fn pbk(&mut self) -> PBK_W {
        PBK_W { w: self }
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PTOKEN_W {
        PTOKEN_W { w: self }
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&mut self) -> PINGEN_W {
        PINGEN_W { w: self }
    }
    #[doc = "Bits 24:31 - binterval parameter"]
    #[inline(always)]
    pub fn binterval(&mut self) -> BINTERVAL_W {
        BINTERVAL_W { w: self }
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
}
#[doc = "`reset()` method sets UPCFG4 to value 0"]
impl crate::Resettable for UPCFG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
