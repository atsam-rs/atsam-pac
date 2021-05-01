#[doc = "Reader of register UECFG7"]
pub type R = crate::R<u32, super::UECFG7>;
#[doc = "Writer for register UECFG7"]
pub type W = crate::W<u32, super::UECFG7>;
#[doc = "Register UECFG7 `reset()`'s with value 0"]
impl crate::ResetValue for super::UECFG7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Endpoint Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPBK_A {
    #[doc = "0: `0`"]
    SINGLE = 0,
    #[doc = "1: `1`"]
    DOUBLE = 1,
}
impl From<EPBK_A> for bool {
    #[inline(always)]
    fn from(variant: EPBK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPBK`"]
pub type EPBK_R = crate::R<bool, EPBK_A>;
impl EPBK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPBK_A {
        match self.bits {
            false => EPBK_A::SINGLE,
            true => EPBK_A::DOUBLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == EPBK_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DOUBLE`"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == EPBK_A::DOUBLE
    }
}
#[doc = "Write proxy for field `EPBK`"]
pub struct EPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPBK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(EPBK_A::SINGLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn double(self) -> &'a mut W {
        self.variant(EPBK_A::DOUBLE)
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
#[doc = "Endpoint Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSIZE_A {
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
impl From<EPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EPSIZE`"]
pub type EPSIZE_R = crate::R<u8, EPSIZE_A>;
impl EPSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSIZE_A {
        match self.bits {
            0 => EPSIZE_A::_8,
            1 => EPSIZE_A::_16,
            2 => EPSIZE_A::_32,
            3 => EPSIZE_A::_64,
            4 => EPSIZE_A::_128,
            5 => EPSIZE_A::_256,
            6 => EPSIZE_A::_512,
            7 => EPSIZE_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == EPSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == EPSIZE_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == EPSIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == EPSIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == EPSIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == EPSIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == EPSIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == EPSIZE_A::_1024
    }
}
#[doc = "Write proxy for field `EPSIZE`"]
pub struct EPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(EPSIZE_A::_8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(EPSIZE_A::_16)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(EPSIZE_A::_32)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(EPSIZE_A::_64)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(EPSIZE_A::_128)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(EPSIZE_A::_256)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(EPSIZE_A::_512)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(EPSIZE_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIR_A {
    #[doc = "0: `0`"]
    OUT = 0,
    #[doc = "1: `1`"]
    IN = 1,
}
impl From<EPDIR_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPDIR`"]
pub type EPDIR_R = crate::R<bool, EPDIR_A>;
impl EPDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDIR_A {
        match self.bits {
            false => EPDIR_A::OUT,
            true => EPDIR_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EPDIR_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == EPDIR_A::IN
    }
}
#[doc = "Write proxy for field `EPDIR`"]
pub struct EPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EPDIR_A::OUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EPDIR_A::IN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: `0`"]
    CONTROL = 0,
    #[doc = "1: `1`"]
    ISOCHRONOUS = 1,
    #[doc = "2: `10`"]
    BULK = 2,
    #[doc = "3: `11`"]
    INTERRUPT = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EPTYPE`"]
pub type EPTYPE_R = crate::R<u8, EPTYPE_A>;
impl EPTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::CONTROL,
            1 => EPTYPE_A::ISOCHRONOUS,
            2 => EPTYPE_A::BULK,
            3 => EPTYPE_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EPTYPE_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISOCHRONOUS`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == EPTYPE_A::ISOCHRONOUS
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EPTYPE_A::BULK
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EPTYPE_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `EPTYPE`"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EPTYPE_A::CONTROL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISOCHRONOUS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EPTYPE_A::INTERRUPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `REPNB`"]
pub type REPNB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REPNB`"]
pub struct REPNB_W<'a> {
    w: &'a mut W,
}
impl<'a> REPNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Endpoint Bank"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 11) & 0x03) as u8)
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
    pub fn epbk(&mut self) -> EPBK_W {
        EPBK_W { w: self }
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&mut self) -> EPSIZE_W {
        EPSIZE_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W { w: self }
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bits 16:19 - Redirected Endpoint Number"]
    #[inline(always)]
    pub fn repnb(&mut self) -> REPNB_W {
        REPNB_W { w: self }
    }
}
