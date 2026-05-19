#[doc = "Register `SET_MESSAGE_ING` writer"]
pub type W = crate::W<SetMessageIngSpec>;
#[doc = "Field `SET_TEXT_ING` writer - Configures whether or not there are unprocessed message blocks. \\\\0: No unprocessed message block \\\\1: There are still some message blocks to be processed."]
pub type SetTextIngW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not there are unprocessed message blocks. \\\\0: No unprocessed message block \\\\1: There are still some message blocks to be processed."]
    #[inline(always)]
    pub fn set_text_ing(&mut self) -> SetTextIngW<'_, SetMessageIngSpec> {
        SetTextIngW::new(self, 0)
    }
}
#[doc = "HMAC message continue register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_message_ing::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetMessageIngSpec;
impl crate::RegisterSpec for SetMessageIngSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_message_ing::W`](W) writer structure"]
impl crate::Writable for SetMessageIngSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_MESSAGE_ING to value 0"]
impl crate::Resettable for SetMessageIngSpec {}
