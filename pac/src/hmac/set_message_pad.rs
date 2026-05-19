#[doc = "Register `SET_MESSAGE_PAD` writer"]
pub type W = crate::W<SetMessagePadSpec>;
#[doc = "Field `SET_TEXT_PAD` writer - Configures whether or not the padding is applied by software. \\\\0: Not applied by software \\\\1: Applied by software"]
pub type SetTextPadW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not the padding is applied by software. \\\\0: Not applied by software \\\\1: Applied by software"]
    #[inline(always)]
    pub fn set_text_pad(&mut self) -> SetTextPadW<'_, SetMessagePadSpec> {
        SetTextPadW::new(self, 0)
    }
}
#[doc = "Software padding register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_message_pad::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetMessagePadSpec;
impl crate::RegisterSpec for SetMessagePadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_message_pad::W`](W) writer structure"]
impl crate::Writable for SetMessagePadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_MESSAGE_PAD to value 0"]
impl crate::Resettable for SetMessagePadSpec {}
