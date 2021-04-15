#[doc = "Reader of register UPCFG5"]
pub type R = crate::R<u32, super::UPCFG5>;
#[doc = "Writer for register UPCFG5"]
pub type W = crate::W<u32, super::UPCFG5>;
#[doc = "Register UPCFG5 `reset()`'s with value 0"]
impl crate::ResetValue for super::UPCFG5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `PBK`"]
pub type PBK_R = crate::R<bool, PBK_A>;
impl PBK_R {
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
        *self == PBK_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == PBK_A::DOUBLE
    }
}
#[doc = "Write proxy for field `PBK`"]
pub struct PBK_W<'a> {
    w: &'a mut W,
}
impl<'a> PBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Reader of field `PSIZE`"]
pub type PSIZE_R = crate::R<u8, PSIZE_A>;
impl PSIZE_R {
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
        *self == PSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PSIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PSIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PSIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PSIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PSIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PSIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PSIZE_A::_1024
    }
}
#[doc = "Write proxy for field `PSIZE`"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
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
#[doc = "Reader of field `PTOKEN`"]
pub type PTOKEN_R = crate::R<u8, PTOKEN_A>;
impl PTOKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTOKEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTOKEN_A::SETUP),
            1 => Val(PTOKEN_A::IN),
            2 => Val(PTOKEN_A::OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == PTOKEN_A::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == PTOKEN_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == PTOKEN_A::OUT
    }
}
#[doc = "Write proxy for field `PTOKEN`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
#[doc = "Reader of field `PTYPE`"]
pub type PTYPE_R = crate::R<u8, PTYPE_A>;
impl PTYPE_R {
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
        *self == PTYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == PTYPE_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == PTYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == PTYPE_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `PTYPE`"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PINGEN`"]
pub type PINGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINGEN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BINTERVAL`"]
pub type BINTERVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BINTERVAL`"]
pub struct BINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
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
}
