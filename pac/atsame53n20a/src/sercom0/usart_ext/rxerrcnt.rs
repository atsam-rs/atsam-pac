#[doc = "Reader of register RXERRCNT"]
pub type R = crate::R<u8, super::RXERRCNT>;
#[doc = "Reader of field `RXERRCNT`"]
pub type RXERRCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Error Count"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RXERRCNT_R {
        RXERRCNT_R::new((self.bits & 0xff) as u8)
    }
}
