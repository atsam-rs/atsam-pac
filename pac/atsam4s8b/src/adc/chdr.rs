#[doc = "Register `CHDR` writer"]
pub struct W(crate::W<CHDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDR_SPEC>;
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
impl From<crate::W<CHDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` writer - Channel 0 Disable"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH1` writer - Channel 1 Disable"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH2` writer - Channel 2 Disable"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH3` writer - Channel 3 Disable"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH4` writer - Channel 4 Disable"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH5` writer - Channel 5 Disable"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH6` writer - Channel 6 Disable"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH7` writer - Channel 7 Disable"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH8` writer - Channel 8 Disable"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH9` writer - Channel 9 Disable"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH10` writer - Channel 10 Disable"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH11` writer - Channel 11 Disable"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH12` writer - Channel 12 Disable"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH13` writer - Channel 13 Disable"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH14` writer - Channel 14 Disable"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `CH15` writer - Channel 15 Disable"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](index.html) module"]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chdr::W](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
