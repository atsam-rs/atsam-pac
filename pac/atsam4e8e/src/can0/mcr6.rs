#[doc = "Register `MCR6` writer"]
pub struct W(crate::W<MCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR6_SPEC>;
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
impl From<crate::W<MCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDLC` writer - Mailbox Data Length Code"]
pub type MDLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR6_SPEC, u8, u8, 4, O>;
#[doc = "Field `MRTR` writer - Mailbox Remote Transmission Request"]
pub type MRTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR6_SPEC, bool, O>;
#[doc = "Field `MACR` writer - Abort Request for Mailbox x"]
pub type MACR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR6_SPEC, bool, O>;
#[doc = "Field `MTCR` writer - Mailbox Transfer Command"]
pub type MTCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR6_SPEC, bool, O>;
impl W {
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn mdlc(&mut self) -> MDLC_W<16> {
        MDLC_W::new(self)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    #[must_use]
    pub fn mrtr(&mut self) -> MRTR_W<20> {
        MRTR_W::new(self)
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    #[must_use]
    pub fn macr(&mut self) -> MACR_W<22> {
        MACR_W::new(self)
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn mtcr(&mut self) -> MTCR_W<23> {
        MTCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Control Register (MB = 6)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr6](index.html) module"]
pub struct MCR6_SPEC;
impl crate::RegisterSpec for MCR6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mcr6::W](W) writer structure"]
impl crate::Writable for MCR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
