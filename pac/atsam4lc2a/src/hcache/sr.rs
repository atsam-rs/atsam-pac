#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `CSTS` reader - Cache Controller Status"]
pub struct CSTS_R(crate::FieldReader<bool, CSTS_A>);
impl CSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTS_R(crate::FieldReader::new(bits))
    }
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
        **self == CSTS_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CSTS_A::EN
    }
}
impl core::ops::Deref for CSTS_R {
    type Target = crate::FieldReader<bool, CSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Cache Controller Status"]
    #[inline(always)]
    pub fn csts(&self) -> CSTS_R {
        CSTS_R::new((self.bits & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
