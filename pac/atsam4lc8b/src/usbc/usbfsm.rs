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
#[doc = "Field `DRDSTATE` reader - DualRoleDevice state"]
pub type DRDSTATE_R = crate::FieldReader<u8, DRDSTATESELECT_A>;
#[doc = "DualRoleDevice state\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRDSTATESELECT_A {
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
impl From<DRDSTATESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DRDSTATESELECT_A) -> Self {
        variant as _
    }
}
impl DRDSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDSTATESELECT_A {
        match self.bits {
            0 => DRDSTATESELECT_A::A_IDLE,
            1 => DRDSTATESELECT_A::A_WAIT_VRISE,
            2 => DRDSTATESELECT_A::A_WAIT_BCON,
            3 => DRDSTATESELECT_A::A_HOST,
            4 => DRDSTATESELECT_A::A_SUSPEND,
            5 => DRDSTATESELECT_A::A_PERIPHERAL,
            6 => DRDSTATESELECT_A::A_WAIT_VFALL,
            7 => DRDSTATESELECT_A::A_VBUS_ERR,
            8 => DRDSTATESELECT_A::A_WAIT_DISCHARGE,
            9 => DRDSTATESELECT_A::B_IDLE,
            10 => DRDSTATESELECT_A::B_PERIPHERAL,
            11 => DRDSTATESELECT_A::B_WAIT_BEGIN_HNP,
            12 => DRDSTATESELECT_A::B_WAIT_DISCHARGE,
            13 => DRDSTATESELECT_A::B_WAIT_ACON,
            14 => DRDSTATESELECT_A::B_HOST,
            15 => DRDSTATESELECT_A::B_SRP_INIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A_IDLE`"]
    #[inline(always)]
    pub fn is_a_idle(&self) -> bool {
        *self == DRDSTATESELECT_A::A_IDLE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VRISE`"]
    #[inline(always)]
    pub fn is_a_wait_vrise(&self) -> bool {
        *self == DRDSTATESELECT_A::A_WAIT_VRISE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_BCON`"]
    #[inline(always)]
    pub fn is_a_wait_bcon(&self) -> bool {
        *self == DRDSTATESELECT_A::A_WAIT_BCON
    }
    #[doc = "Checks if the value of the field is `A_HOST`"]
    #[inline(always)]
    pub fn is_a_host(&self) -> bool {
        *self == DRDSTATESELECT_A::A_HOST
    }
    #[doc = "Checks if the value of the field is `A_SUSPEND`"]
    #[inline(always)]
    pub fn is_a_suspend(&self) -> bool {
        *self == DRDSTATESELECT_A::A_SUSPEND
    }
    #[doc = "Checks if the value of the field is `A_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_a_peripheral(&self) -> bool {
        *self == DRDSTATESELECT_A::A_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VFALL`"]
    #[inline(always)]
    pub fn is_a_wait_vfall(&self) -> bool {
        *self == DRDSTATESELECT_A::A_WAIT_VFALL
    }
    #[doc = "Checks if the value of the field is `A_VBUS_ERR`"]
    #[inline(always)]
    pub fn is_a_vbus_err(&self) -> bool {
        *self == DRDSTATESELECT_A::A_VBUS_ERR
    }
    #[doc = "Checks if the value of the field is `A_WAIT_DISCHARGE`"]
    #[inline(always)]
    pub fn is_a_wait_discharge(&self) -> bool {
        *self == DRDSTATESELECT_A::A_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_IDLE`"]
    #[inline(always)]
    pub fn is_b_idle(&self) -> bool {
        *self == DRDSTATESELECT_A::B_IDLE
    }
    #[doc = "Checks if the value of the field is `B_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_b_peripheral(&self) -> bool {
        *self == DRDSTATESELECT_A::B_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `B_WAIT_BEGIN_HNP`"]
    #[inline(always)]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        *self == DRDSTATESELECT_A::B_WAIT_BEGIN_HNP
    }
    #[doc = "Checks if the value of the field is `B_WAIT_DISCHARGE`"]
    #[inline(always)]
    pub fn is_b_wait_discharge(&self) -> bool {
        *self == DRDSTATESELECT_A::B_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_WAIT_ACON`"]
    #[inline(always)]
    pub fn is_b_wait_acon(&self) -> bool {
        *self == DRDSTATESELECT_A::B_WAIT_ACON
    }
    #[doc = "Checks if the value of the field is `B_HOST`"]
    #[inline(always)]
    pub fn is_b_host(&self) -> bool {
        *self == DRDSTATESELECT_A::B_HOST
    }
    #[doc = "Checks if the value of the field is `B_SRP_INIT`"]
    #[inline(always)]
    pub fn is_b_srp_init(&self) -> bool {
        *self == DRDSTATESELECT_A::B_SRP_INIT
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
    const RESET_VALUE: Self::Ux = 0x09;
}
