#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Cache Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTS_A {
    #[doc = "0: Cache Controller Disabled"]
    DIS = 0,
    #[doc = "1: Cache Controller Enabled"]
    EN = 1,
}
impl From<CSTS_A> for bool {
    #[inline(always)]
    fn from(variant: CSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSTS`"]
pub type CSTS_R = crate::R<bool, CSTS_A>;
impl CSTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTS_A {
        match self.bits {
            false => CSTS_A::DIS,
            true => CSTS_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CSTS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CSTS_A::EN
    }
}
impl R {
    #[doc = "Bit 0 - Cache Controller Status"]
    #[inline(always)]
    pub fn csts(&self) -> CSTS_R {
        CSTS_R::new((self.bits & 0x01) != 0)
    }
}
