#[doc = "Register `CSR%s` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR%s` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOLSELECT_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOLSELECT_A {
    #[doc = "0: The inactive state value of SPCK is logic level zero."]
    _0 = 0,
    #[doc = "1: The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    _1 = 1,
}
impl From<CPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::_0,
            true => CPOLSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOLSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOLSELECT_A::_1
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CPOLSELECT_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "The inactive state value of SPCK is logic level zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::_0)
    }
    #[doc = "The inactive state value of SPCK is logic level one.CPOL is used to determine the inactive state value of the serial clock (SPCK). It is used with NCPHA to produce therequired clock/data relationship between master and slave devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::_1)
    }
}
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub type NCPHA_R = crate::BitReader<NCPHASELECT_A>;
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCPHASELECT_A {
    #[doc = "0: Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    _0 = 0,
    #[doc = "1: Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    _1 = 1,
}
impl From<NCPHASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NCPHASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NCPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCPHASELECT_A {
        match self.bits {
            false => NCPHASELECT_A::_0,
            true => NCPHASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCPHASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCPHASELECT_A::_1
    }
}
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub type NCPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, NCPHASELECT_A, O>;
impl<'a, const O: u8> NCPHA_W<'a, O> {
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCPHASELECT_A::_0)
    }
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK.NCPHA determines which edge of SPCK causes data to change and which edge causes data to be captured. NCPHA isused with CPOL to produce the required clock/data relationship between master and slave devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCPHASELECT_A::_1)
    }
}
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer"]
pub type CSNAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer"]
pub type CSNAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub type CSAAT_R = crate::BitReader<CSAATSELECT_A>;
#[doc = "Chip Select Active After Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSAATSELECT_A {
    #[doc = "0: The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    _0 = 0,
    #[doc = "1: The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    _1 = 1,
}
impl From<CSAATSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CSAATSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CSAAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSAATSELECT_A {
        match self.bits {
            false => CSAATSELECT_A::_0,
            true => CSAATSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSAATSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSAATSELECT_A::_1
    }
}
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub type CSAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CSAATSELECT_A, O>;
impl<'a, const O: u8> CSAAT_W<'a, O> {
    #[doc = "The Peripheral Chip Select Line rises as soon as the last transfer is achieved."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSAATSELECT_A::_0)
    }
    #[doc = "The Peripheral Chip Select does not rise after the last transfer is achieved. It remains active until a new transfer isrequested on a different chip select."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSAATSELECT_A::_1)
    }
}
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub type BITS_R = crate::FieldReader<u8, BITSSELECT_A>;
#[doc = "Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITSSELECT_A {
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
impl From<BITSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BITSSELECT_A) -> Self {
        variant as _
    }
}
impl BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITSSELECT_A> {
        match self.bits {
            0 => Some(BITSSELECT_A::_8_BPT),
            1 => Some(BITSSELECT_A::_9_BPT),
            2 => Some(BITSSELECT_A::_10_BPT),
            3 => Some(BITSSELECT_A::_11_BPT),
            4 => Some(BITSSELECT_A::_12_BPT),
            5 => Some(BITSSELECT_A::_13_BPT),
            6 => Some(BITSSELECT_A::_14_BPT),
            7 => Some(BITSSELECT_A::_15_BPT),
            8 => Some(BITSSELECT_A::_16_BPT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BPT`"]
    #[inline(always)]
    pub fn is_8_bpt(&self) -> bool {
        *self == BITSSELECT_A::_8_BPT
    }
    #[doc = "Checks if the value of the field is `_9_BPT`"]
    #[inline(always)]
    pub fn is_9_bpt(&self) -> bool {
        *self == BITSSELECT_A::_9_BPT
    }
    #[doc = "Checks if the value of the field is `_10_BPT`"]
    #[inline(always)]
    pub fn is_10_bpt(&self) -> bool {
        *self == BITSSELECT_A::_10_BPT
    }
    #[doc = "Checks if the value of the field is `_11_BPT`"]
    #[inline(always)]
    pub fn is_11_bpt(&self) -> bool {
        *self == BITSSELECT_A::_11_BPT
    }
    #[doc = "Checks if the value of the field is `_12_BPT`"]
    #[inline(always)]
    pub fn is_12_bpt(&self) -> bool {
        *self == BITSSELECT_A::_12_BPT
    }
    #[doc = "Checks if the value of the field is `_13_BPT`"]
    #[inline(always)]
    pub fn is_13_bpt(&self) -> bool {
        *self == BITSSELECT_A::_13_BPT
    }
    #[doc = "Checks if the value of the field is `_14_BPT`"]
    #[inline(always)]
    pub fn is_14_bpt(&self) -> bool {
        *self == BITSSELECT_A::_14_BPT
    }
    #[doc = "Checks if the value of the field is `_15_BPT`"]
    #[inline(always)]
    pub fn is_15_bpt(&self) -> bool {
        *self == BITSSELECT_A::_15_BPT
    }
    #[doc = "Checks if the value of the field is `_16_BPT`"]
    #[inline(always)]
    pub fn is_16_bpt(&self) -> bool {
        *self == BITSSELECT_A::_16_BPT
    }
}
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, BITSSELECT_A, 4, O>;
impl<'a, const O: u8> BITS_W<'a, O> {
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn _8_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_8_BPT)
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn _9_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_9_BPT)
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn _10_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_10_BPT)
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn _11_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_11_BPT)
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn _12_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_12_BPT)
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn _13_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_13_BPT)
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn _14_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_14_BPT)
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn _15_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_15_BPT)
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn _16_bpt(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_16_BPT)
    }
}
#[doc = "Field `SCBR` reader - Serial Clock Baud Rate"]
pub type SCBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBR` writer - Serial Clock Baud Rate"]
pub type SCBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub type DLYBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub type DLYBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NCPHA_R {
        NCPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer"]
    #[inline(always)]
    pub fn csnaat(&self) -> CSNAAT_R {
        CSNAAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CSAAT_R {
        CSAAT_R::new(((self.bits >> 3) & 1) != 0)
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
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<0> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ncpha(&mut self) -> NCPHA_W<1> {
        NCPHA_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn csnaat(&mut self) -> CSNAAT_W<2> {
        CSNAAT_W::new(self)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn csaat(&mut self) -> CSAAT_W<3> {
        CSAAT_W::new(self)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BITS_W<4> {
        BITS_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn scbr(&mut self) -> SCBR_W<8> {
        SCBR_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs(&mut self) -> DLYBS_W<16> {
        DLYBS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DLYBCT_W<24> {
        DLYBCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR%s to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
