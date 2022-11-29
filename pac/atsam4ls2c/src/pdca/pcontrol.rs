#[doc = "Register `PCONTROL` reader"]
pub struct R(crate::R<PCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONTROL` writer"]
pub struct W(crate::W<PCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONTROL_SPEC>;
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
impl From<crate::W<PCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0EN` reader - Channel 0 Enabled"]
pub type CH0EN_R = crate::BitReader<bool>;
#[doc = "Field `CH0EN` writer - Channel 0 Enabled"]
pub type CH0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONTROL_SPEC, bool, O>;
#[doc = "Field `CH1EN` reader - Channel 1 Enabled."]
pub type CH1EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1EN` writer - Channel 1 Enabled."]
pub type CH1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONTROL_SPEC, bool, O>;
#[doc = "Field `CH0OF` reader - Channel 0 Overflow Freeze"]
pub type CH0OF_R = crate::BitReader<bool>;
#[doc = "Field `CH0OF` writer - Channel 0 Overflow Freeze"]
pub type CH0OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONTROL_SPEC, bool, O>;
#[doc = "Field `CH1OF` reader - Channel 1 overflow freeze"]
pub type CH1OF_R = crate::BitReader<bool>;
#[doc = "Field `CH1OF` writer - Channel 1 overflow freeze"]
pub type CH1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONTROL_SPEC, bool, O>;
#[doc = "Field `CH0RES` reader - Channel 0 counter reset"]
pub type CH0RES_R = crate::BitReader<bool>;
#[doc = "Field `CH0RES` writer - Channel 0 counter reset"]
pub type CH0RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONTROL_SPEC, bool, O>;
#[doc = "Field `CH1RES` reader - Channel 1 counter reset"]
pub type CH1RES_R = crate::BitReader<bool>;
#[doc = "Field `CH1RES` writer - Channel 1 counter reset"]
pub type CH1RES_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCONTROL_SPEC, bool, O>;
#[doc = "Field `MON0CH` reader - PDCA Channel to monitor with counter 0"]
pub type MON0CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON0CH` writer - PDCA Channel to monitor with counter 0"]
pub type MON0CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONTROL_SPEC, u8, u8, 6, O>;
#[doc = "Field `MON1CH` reader - PDCA Channel to monitor with counter 1"]
pub type MON1CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON1CH` writer - PDCA Channel to monitor with counter 1"]
pub type MON1CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCONTROL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled."]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Overflow Freeze"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 overflow freeze"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 counter reset"]
    #[inline(always)]
    pub fn ch0res(&self) -> CH0RES_R {
        CH0RES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 counter reset"]
    #[inline(always)]
    pub fn ch1res(&self) -> CH1RES_R {
        CH1RES_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:21 - PDCA Channel to monitor with counter 0"]
    #[inline(always)]
    pub fn mon0ch(&self) -> MON0CH_R {
        MON0CH_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - PDCA Channel to monitor with counter 1"]
    #[inline(always)]
    pub fn mon1ch(&self) -> MON1CH_R {
        MON1CH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> CH0EN_W<0> {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> CH1EN_W<1> {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 4 - Channel 0 Overflow Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<4> {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 overflow freeze"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<5> {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 8 - Channel 0 counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch0res(&mut self) -> CH0RES_W<8> {
        CH0RES_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch1res(&mut self) -> CH1RES_W<9> {
        CH1RES_W::new(self)
    }
    #[doc = "Bits 16:21 - PDCA Channel to monitor with counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn mon0ch(&mut self) -> MON0CH_W<16> {
        MON0CH_W::new(self)
    }
    #[doc = "Bits 24:29 - PDCA Channel to monitor with counter 1"]
    #[inline(always)]
    #[must_use]
    pub fn mon1ch(&mut self) -> MON1CH_W<24> {
        MON1CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Performance Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcontrol](index.html) module"]
pub struct PCONTROL_SPEC;
impl crate::RegisterSpec for PCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcontrol::R](R) reader structure"]
impl crate::Readable for PCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcontrol::W](W) writer structure"]
impl crate::Writable for PCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCONTROL to value 0"]
impl crate::Resettable for PCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
