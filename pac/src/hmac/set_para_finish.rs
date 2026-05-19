#[doc = "Register `SET_PARA_FINISH` writer"]
pub type W = crate::W<SetParaFinishSpec>;
#[doc = "Field `SET_PARA_END` writer - Configures whether to finish HMAC configuration. \\\\0: No effect \\\\1: Finish configuration"]
pub type SetParaEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether to finish HMAC configuration. \\\\0: No effect \\\\1: Finish configuration"]
    #[inline(always)]
    pub fn set_para_end(&mut self) -> SetParaEndW<'_, SetParaFinishSpec> {
        SetParaEndW::new(self, 0)
    }
}
#[doc = "HMAC configuration completion register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_para_finish::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetParaFinishSpec;
impl crate::RegisterSpec for SetParaFinishSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_para_finish::W`](W) writer structure"]
impl crate::Writable for SetParaFinishSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_PARA_FINISH to value 0"]
impl crate::Resettable for SetParaFinishSpec {}
