#[doc = "Register `WPCR` writer"]
pub struct W(crate::W<WPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR_SPEC>;
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
impl From<crate::W<WPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write Protect Command"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WPCMD_AW {
    #[doc = "0: Disable the Software Write Protect of the register groups of which the bit WPRGx is at '1'."]
    DISABLE_SW_PROT = 0,
    #[doc = "1: Enable the Software Write Protect of the register groups of which the bit WPRGx is at '1'."]
    ENABLE_SW_PROT = 1,
    #[doc = "2: Enable the Hardware Write Protect of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protect. Moreover, to meet security requirements, the PIO lines associated with PWM can not be configured through the PIO interface."]
    ENABLE_HW_PROT = 2,
}
impl From<WPCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: WPCMD_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WPCMD` writer - Write Protect Command"]
pub type WPCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR_SPEC, u8, WPCMD_AW, 2, O>;
impl<'a, const O: u8> WPCMD_W<'a, O> {
    #[doc = "Disable the Software Write Protect of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn disable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMD_AW::DISABLE_SW_PROT)
    }
    #[doc = "Enable the Software Write Protect of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn enable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMD_AW::ENABLE_SW_PROT)
    }
    #[doc = "Enable the Hardware Write Protect of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protect. Moreover, to meet security requirements, the PIO lines associated with PWM can not be configured through the PIO interface."]
    #[inline(always)]
    pub fn enable_hw_prot(self) -> &'a mut W {
        self.variant(WPCMD_AW::ENABLE_HW_PROT)
    }
}
#[doc = "Field `WPRG0` writer - Write Protect Register Group 0"]
pub type WPRG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG1` writer - Write Protect Register Group 1"]
pub type WPRG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG2` writer - Write Protect Register Group 2"]
pub type WPRG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG3` writer - Write Protect Register Group 3"]
pub type WPRG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG4` writer - Write Protect Register Group 4"]
pub type WPRG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPRG5` writer - Write Protect Register Group 5"]
pub type WPRG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Write Protect Key"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEY_AW {
    #[doc = "5265229: Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    PASSWD = 5265229,
}
impl From<WPKEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: WPKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WPKEY` writer - Write Protect Key"]
pub type WPKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR_SPEC, u32, WPKEY_AW, 24, O>;
impl<'a, const O: u8> WPKEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEY_AW::PASSWD)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Protect Command"]
    #[inline(always)]
    #[must_use]
    pub fn wpcmd(&mut self) -> WPCMD_W<0> {
        WPCMD_W::new(self)
    }
    #[doc = "Bit 2 - Write Protect Register Group 0"]
    #[inline(always)]
    #[must_use]
    pub fn wprg0(&mut self) -> WPRG0_W<2> {
        WPRG0_W::new(self)
    }
    #[doc = "Bit 3 - Write Protect Register Group 1"]
    #[inline(always)]
    #[must_use]
    pub fn wprg1(&mut self) -> WPRG1_W<3> {
        WPRG1_W::new(self)
    }
    #[doc = "Bit 4 - Write Protect Register Group 2"]
    #[inline(always)]
    #[must_use]
    pub fn wprg2(&mut self) -> WPRG2_W<4> {
        WPRG2_W::new(self)
    }
    #[doc = "Bit 5 - Write Protect Register Group 3"]
    #[inline(always)]
    #[must_use]
    pub fn wprg3(&mut self) -> WPRG3_W<5> {
        WPRG3_W::new(self)
    }
    #[doc = "Bit 6 - Write Protect Register Group 4"]
    #[inline(always)]
    #[must_use]
    pub fn wprg4(&mut self) -> WPRG4_W<6> {
        WPRG4_W::new(self)
    }
    #[doc = "Bit 7 - Write Protect Register Group 5"]
    #[inline(always)]
    #[must_use]
    pub fn wprg5(&mut self) -> WPRG5_W<7> {
        WPRG5_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protect Key"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Write Protection Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](index.html) module"]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wpcr::W](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
