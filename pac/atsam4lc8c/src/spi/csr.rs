#[doc = "Reader of register CSR%s"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR%s"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SPCK is logic level zero."]
    _0 = 0,
    #[doc = "1: The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The inactive state value of SPCK is logic level zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCPHA_A {
    #[doc = "0: Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    _0 = 0,
    #[doc = "1: Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    _1 = 1,
}
impl From<NCPHA_A> for bool {
    #[inline(always)]
    fn from(variant: NCPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCPHA`"]
pub type NCPHA_R = crate::R<bool, NCPHA_A>;
impl NCPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCPHA_A {
        match self.bits {
            false => NCPHA_A::_0,
            true => NCPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCPHA_A::_1
    }
}
#[doc = "Write proxy for field `NCPHA`"]
pub struct NCPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> NCPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCPHA_A::_0)
    }
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCPHA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CSNAAT`"]
pub type CSNAAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSNAAT`"]
pub struct CSNAAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSNAAT_W<'a> {
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
#[doc = "Chip Select Active After Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSAAT_A {
    #[doc = "0: The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    _0 = 0,
    #[doc = "1: The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    _1 = 1,
}
impl From<CSAAT_A> for bool {
    #[inline(always)]
    fn from(variant: CSAAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSAAT`"]
pub type CSAAT_R = crate::R<bool, CSAAT_A>;
impl CSAAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSAAT_A {
        match self.bits {
            false => CSAAT_A::_0,
            true => CSAAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSAAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSAAT_A::_1
    }
}
#[doc = "Write proxy for field `CSAAT`"]
pub struct CSAAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSAAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSAAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSAAT_A::_0)
    }
    #[doc = "The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSAAT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BITS_A {
    #[doc = "0: 8 bits per transfer"]
    _8_BPT = 0,
    #[doc = "1: 9 bits per transfer"]
    _9_BPT = 1,
    #[doc = "2: 10 bits per transfer"]
    _10_BPT = 2,
    #[doc = "3: 11 bits per transfer"]
    _11_BPT = 3,
    #[doc = "4: 12 bits per transfer"]
    _12_BPT = 4,
    #[doc = "5: 13 bits per transfer"]
    _13_BPT = 5,
    #[doc = "6: 14 bits per transfer"]
    _14_BPT = 6,
    #[doc = "7: 15 bits per transfer"]
    _15_BPT = 7,
    #[doc = "8: 16 bits per transfer"]
    _16_BPT = 8,
}
impl From<BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BITS`"]
pub type BITS_R = crate::R<u8, BITS_A>;
impl BITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BITS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BITS_A::_8_BPT),
            1 => Val(BITS_A::_9_BPT),
            2 => Val(BITS_A::_10_BPT),
            3 => Val(BITS_A::_11_BPT),
            4 => Val(BITS_A::_12_BPT),
            5 => Val(BITS_A::_13_BPT),
            6 => Val(BITS_A::_14_BPT),
            7 => Val(BITS_A::_15_BPT),
            8 => Val(BITS_A::_16_BPT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BPT`"]
    #[inline(always)]
    pub fn is_8_bpt(&self) -> bool {
        *self == BITS_A::_8_BPT
    }
    #[doc = "Checks if the value of the field is `_9_BPT`"]
    #[inline(always)]
    pub fn is_9_bpt(&self) -> bool {
        *self == BITS_A::_9_BPT
    }
    #[doc = "Checks if the value of the field is `_10_BPT`"]
    #[inline(always)]
    pub fn is_10_bpt(&self) -> bool {
        *self == BITS_A::_10_BPT
    }
    #[doc = "Checks if the value of the field is `_11_BPT`"]
    #[inline(always)]
    pub fn is_11_bpt(&self) -> bool {
        *self == BITS_A::_11_BPT
    }
    #[doc = "Checks if the value of the field is `_12_BPT`"]
    #[inline(always)]
    pub fn is_12_bpt(&self) -> bool {
        *self == BITS_A::_12_BPT
    }
    #[doc = "Checks if the value of the field is `_13_BPT`"]
    #[inline(always)]
    pub fn is_13_bpt(&self) -> bool {
        *self == BITS_A::_13_BPT
    }
    #[doc = "Checks if the value of the field is `_14_BPT`"]
    #[inline(always)]
    pub fn is_14_bpt(&self) -> bool {
        *self == BITS_A::_14_BPT
    }
    #[doc = "Checks if the value of the field is `_15_BPT`"]
    #[inline(always)]
    pub fn is_15_bpt(&self) -> bool {
        *self == BITS_A::_15_BPT
    }
    #[doc = "Checks if the value of the field is `_16_BPT`"]
    #[inline(always)]
    pub fn is_16_bpt(&self) -> bool {
        *self == BITS_A::_16_BPT
    }
}
#[doc = "Write proxy for field `BITS`"]
pub struct BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> BITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn _8_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_8_BPT)
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn _9_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_9_BPT)
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn _10_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_10_BPT)
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn _11_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_11_BPT)
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn _12_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_12_BPT)
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn _13_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_13_BPT)
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn _14_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_14_BPT)
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn _15_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_15_BPT)
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn _16_bpt(self) -> &'a mut W {
        self.variant(BITS_A::_16_BPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCBR`"]
pub type SCBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCBR`"]
pub struct SCBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DLYBS`"]
pub type DLYBS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYBS`"]
pub struct DLYBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DLYBCT`"]
pub type DLYBCT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYBCT`"]
pub struct DLYBCT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NCPHA_R {
        NCPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer"]
    #[inline(always)]
    pub fn csnaat(&self) -> CSNAAT_R {
        CSNAAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CSAAT_R {
        CSAAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> SCBR_R {
        SCBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&mut self) -> NCPHA_W {
        NCPHA_W { w: self }
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer"]
    #[inline(always)]
    pub fn csnaat(&mut self) -> CSNAAT_W {
        CSNAAT_W { w: self }
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&mut self) -> CSAAT_W {
        CSAAT_W { w: self }
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W {
        BITS_W { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&mut self) -> SCBR_W {
        SCBR_W { w: self }
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&mut self) -> DLYBS_W {
        DLYBS_W { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DLYBCT_W {
        DLYBCT_W { w: self }
    }
}
