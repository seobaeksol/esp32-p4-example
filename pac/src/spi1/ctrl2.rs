#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `SYNC_RESET` writer - The FSM will be reset."]
pub type SyncResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SyncResetW<'_, Ctrl2Spec> {
        SyncResetW::new(self, 31)
    }
}
#[doc = "SPI1 control2 register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {}
