#[doc = "Register `USBFSM` reader"]
pub struct R(crate::R<USBFSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBFSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBFSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBFSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DualRoleDevice state\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRDSTATE_A {
    #[doc = "0: `0`"]
    A_IDLE = 0,
    #[doc = "1: `1`"]
    A_WAIT_VRISE = 1,
    #[doc = "2: `10`"]
    A_WAIT_BCON = 2,
    #[doc = "3: `11`"]
    A_HOST = 3,
    #[doc = "4: `100`"]
    A_SUSPEND = 4,
    #[doc = "5: `101`"]
    A_PERIPHERAL = 5,
    #[doc = "6: `110`"]
    A_WAIT_VFALL = 6,
    #[doc = "7: `111`"]
    A_VBUS_ERR = 7,
    #[doc = "8: `1000`"]
    A_WAIT_DISCHARGE = 8,
    #[doc = "9: `1001`"]
    B_IDLE = 9,
    #[doc = "10: `1010`"]
    B_PERIPHERAL = 10,
    #[doc = "11: `1011`"]
    B_WAIT_BEGIN_HNP = 11,
    #[doc = "12: `1100`"]
    B_WAIT_DISCHARGE = 12,
    #[doc = "13: `1101`"]
    B_WAIT_ACON = 13,
    #[doc = "14: `1110`"]
    B_HOST = 14,
    #[doc = "15: `1111`"]
    B_SRP_INIT = 15,
}
impl From<DRDSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRDSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRDSTATE` reader - DualRoleDevice state"]
pub struct DRDSTATE_R(crate::FieldReader<u8, DRDSTATE_A>);
impl DRDSTATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRDSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDSTATE_A {
        match self.bits {
            0 => DRDSTATE_A::A_IDLE,
            1 => DRDSTATE_A::A_WAIT_VRISE,
            2 => DRDSTATE_A::A_WAIT_BCON,
            3 => DRDSTATE_A::A_HOST,
            4 => DRDSTATE_A::A_SUSPEND,
            5 => DRDSTATE_A::A_PERIPHERAL,
            6 => DRDSTATE_A::A_WAIT_VFALL,
            7 => DRDSTATE_A::A_VBUS_ERR,
            8 => DRDSTATE_A::A_WAIT_DISCHARGE,
            9 => DRDSTATE_A::B_IDLE,
            10 => DRDSTATE_A::B_PERIPHERAL,
            11 => DRDSTATE_A::B_WAIT_BEGIN_HNP,
            12 => DRDSTATE_A::B_WAIT_DISCHARGE,
            13 => DRDSTATE_A::B_WAIT_ACON,
            14 => DRDSTATE_A::B_HOST,
            15 => DRDSTATE_A::B_SRP_INIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A_IDLE`"]
    #[inline(always)]
    pub fn is_a_idle(&self) -> bool {
        **self == DRDSTATE_A::A_IDLE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VRISE`"]
    #[inline(always)]
    pub fn is_a_wait_vrise(&self) -> bool {
        **self == DRDSTATE_A::A_WAIT_VRISE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_BCON`"]
    #[inline(always)]
    pub fn is_a_wait_bcon(&self) -> bool {
        **self == DRDSTATE_A::A_WAIT_BCON
    }
    #[doc = "Checks if the value of the field is `A_HOST`"]
    #[inline(always)]
    pub fn is_a_host(&self) -> bool {
        **self == DRDSTATE_A::A_HOST
    }
    #[doc = "Checks if the value of the field is `A_SUSPEND`"]
    #[inline(always)]
    pub fn is_a_suspend(&self) -> bool {
        **self == DRDSTATE_A::A_SUSPEND
    }
    #[doc = "Checks if the value of the field is `A_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_a_peripheral(&self) -> bool {
        **self == DRDSTATE_A::A_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VFALL`"]
    #[inline(always)]
    pub fn is_a_wait_vfall(&self) -> bool {
        **self == DRDSTATE_A::A_WAIT_VFALL
    }
    #[doc = "Checks if the value of the field is `A_VBUS_ERR`"]
    #[inline(always)]
    pub fn is_a_vbus_err(&self) -> bool {
        **self == DRDSTATE_A::A_VBUS_ERR
    }
    #[doc = "Checks if the value of the field is `A_WAIT_DISCHARGE`"]
    #[inline(always)]
    pub fn is_a_wait_discharge(&self) -> bool {
        **self == DRDSTATE_A::A_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_IDLE`"]
    #[inline(always)]
    pub fn is_b_idle(&self) -> bool {
        **self == DRDSTATE_A::B_IDLE
    }
    #[doc = "Checks if the value of the field is `B_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_b_peripheral(&self) -> bool {
        **self == DRDSTATE_A::B_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `B_WAIT_BEGIN_HNP`"]
    #[inline(always)]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        **self == DRDSTATE_A::B_WAIT_BEGIN_HNP
    }
    #[doc = "Checks if the value of the field is `B_WAIT_DISCHARGE`"]
    #[inline(always)]
    pub fn is_b_wait_discharge(&self) -> bool {
        **self == DRDSTATE_A::B_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_WAIT_ACON`"]
    #[inline(always)]
    pub fn is_b_wait_acon(&self) -> bool {
        **self == DRDSTATE_A::B_WAIT_ACON
    }
    #[doc = "Checks if the value of the field is `B_HOST`"]
    #[inline(always)]
    pub fn is_b_host(&self) -> bool {
        **self == DRDSTATE_A::B_HOST
    }
    #[doc = "Checks if the value of the field is `B_SRP_INIT`"]
    #[inline(always)]
    pub fn is_b_srp_init(&self) -> bool {
        **self == DRDSTATE_A::B_SRP_INIT
    }
}
impl core::ops::Deref for DRDSTATE_R {
    type Target = crate::FieldReader<u8, DRDSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - DualRoleDevice state"]
    #[inline(always)]
    pub fn drdstate(&self) -> DRDSTATE_R {
        DRDSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "USB internal finite state machine\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfsm](index.html) module"]
pub struct USBFSM_SPEC;
impl crate::RegisterSpec for USBFSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbfsm::R](R) reader structure"]
impl crate::Readable for USBFSM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBFSM to value 0x09"]
impl crate::Resettable for USBFSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09
    }
}
