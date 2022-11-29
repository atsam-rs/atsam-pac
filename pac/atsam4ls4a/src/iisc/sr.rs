#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RXEN_R = crate::BitReader<RXENSELECT_A>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXENSELECT_A {
    #[doc = "0: Receiver is effectively disabled, following a CR.RXDIS or CR.SWRST request"]
    OFF = 0,
    #[doc = "1: Receiver is effectively enabled, following a CR.RXEN request"]
    ON = 1,
}
impl From<RXENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXENSELECT_A {
        match self.bits {
            false => RXENSELECT_A::OFF,
            true => RXENSELECT_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RXENSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RXENSELECT_A::ON
    }
}
#[doc = "Field `RXRDY` reader - Receive Ready"]
pub type RXRDY_R = crate::BitReader<RXRDYSELECT_A>;
#[doc = "Receive Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_A {
    #[doc = "0: The register RHR is empty and can't be read"]
    EMPTY = 0,
    #[doc = "1: The register RHR is full and is ready to be read"]
    FULL = 1,
}
impl From<RXRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDYSELECT_A {
        match self.bits {
            false => RXRDYSELECT_A::EMPTY,
            true => RXRDYSELECT_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXRDYSELECT_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXRDYSELECT_A::FULL
    }
}
#[doc = "Field `RXOR` reader - Receive Overrun"]
pub type RXOR_R = crate::BitReader<RXORSELECT_A>;
#[doc = "Receive Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORSELECT_A {
    #[doc = "0: No overrun"]
    NO = 0,
    #[doc = "1: The previous received data has not been read. This data is lost"]
    YES = 1,
}
impl From<RXORSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXORSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXORSELECT_A {
        match self.bits {
            false => RXORSELECT_A::NO,
            true => RXORSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == RXORSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == RXORSELECT_A::YES
    }
}
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub type TXEN_R = crate::BitReader<TXENSELECT_A>;
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENSELECT_A {
    #[doc = "0: Transmitter is effectively disabled, following a CR.TXDIS or CR.SWRST request"]
    OFF = 0,
    #[doc = "1: Transmitter is effectively enabled, following a CR.TXEN request"]
    ON = 1,
}
impl From<TXENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENSELECT_A {
        match self.bits {
            false => TXENSELECT_A::OFF,
            true => TXENSELECT_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == TXENSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == TXENSELECT_A::ON
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TXRDY_R = crate::BitReader<TXRDYSELECT_A>;
#[doc = "Transmit Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_A {
    #[doc = "0: The register THR is full and can't be written"]
    FULL = 0,
    #[doc = "1: The register THR is empty and is ready to be written"]
    EMPTY = 1,
}
impl From<TXRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDYSELECT_A {
        match self.bits {
            false => TXRDYSELECT_A::FULL,
            true => TXRDYSELECT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXRDYSELECT_A::FULL
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXRDYSELECT_A::EMPTY
    }
}
#[doc = "Field `TXUR` reader - Transmit Underrun"]
pub type TXUR_R = crate::BitReader<TXURSELECT_A>;
#[doc = "Transmit Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURSELECT_A {
    #[doc = "0: No underrun"]
    NO = 0,
    #[doc = "1: The last bit of the last data written to the register THR has been set. Until the next write to THR, data will be sent according to MR.TXSAME field"]
    YES = 1,
}
impl From<TXURSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXURSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXURSELECT_A {
        match self.bits {
            false => TXURSELECT_A::NO,
            true => TXURSELECT_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TXURSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TXURSELECT_A::YES
    }
}
#[doc = "Field `RXORCH` reader - Receive Overrun Channels"]
pub type RXORCH_R = crate::FieldReader<u8, RXORCHSELECT_A>;
#[doc = "Receive Overrun Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXORCHSELECT_A {
    #[doc = "0: Overrun first occurred on left channel"]
    LEFT = 0,
    #[doc = "1: Overrun first occurred on right channel"]
    RIGHT = 1,
}
impl From<RXORCHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXORCHSELECT_A) -> Self {
        variant as _
    }
}
impl RXORCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXORCHSELECT_A> {
        match self.bits {
            0 => Some(RXORCHSELECT_A::LEFT),
            1 => Some(RXORCHSELECT_A::RIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == RXORCHSELECT_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == RXORCHSELECT_A::RIGHT
    }
}
#[doc = "Field `TXURCH` reader - Transmit Underrun Channels"]
pub type TXURCH_R = crate::FieldReader<u8, TXURCHSELECT_A>;
#[doc = "Transmit Underrun Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXURCHSELECT_A {
    #[doc = "0: Underrun first occurred on left channel"]
    LEFT = 0,
    #[doc = "1: Underrun first occurred on right channel"]
    RIGHT = 1,
}
impl From<TXURCHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXURCHSELECT_A) -> Self {
        variant as _
    }
}
impl TXURCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXURCHSELECT_A> {
        match self.bits {
            0 => Some(TXURCHSELECT_A::LEFT),
            1 => Some(TXURCHSELECT_A::RIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == TXURCHSELECT_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == TXURCHSELECT_A::RIGHT
    }
}
impl R {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channels"]
    #[inline(always)]
    pub fn rxorch(&self) -> RXORCH_R {
        RXORCH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channels"]
    #[inline(always)]
    pub fn txurch(&self) -> TXURCH_R {
        TXURCH_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
