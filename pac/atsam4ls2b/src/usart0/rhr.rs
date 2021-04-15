#[doc = "Reader of register RHR"]
pub type R = crate::R<u32, super::RHR>;
#[doc = "Reader of field `RXCHR`"]
pub type RXCHR_R = crate::R<u16, u16>;
#[doc = "Received Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSYNH_A {
    #[doc = "0: Last character received is a Data"]
    _0 = 0,
    #[doc = "1: Last character received is a Command"]
    _1 = 1,
}
impl From<RXSYNH_A> for bool {
    #[inline(always)]
    fn from(variant: RXSYNH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXSYNH`"]
pub type RXSYNH_R = crate::R<bool, RXSYNH_A>;
impl RXSYNH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSYNH_A {
        match self.bits {
            false => RXSYNH_A::_0,
            true => RXSYNH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXSYNH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXSYNH_A::_1
    }
}
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RXSYNH_R {
        RXSYNH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
