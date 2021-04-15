#[doc = "Writer for register TRIER"]
pub type W = crate::W<u32, super::TRIER>;
#[doc = "Register TRIER `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TRIE`"]
pub struct TRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn trie(&mut self) -> TRIE_W {
        TRIE_W { w: self }
    }
}
