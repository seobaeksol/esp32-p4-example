#[doc = "Register `REG_UPDATE` writer"]
pub type W = crate::W<RegUpdateSpec>;
#[doc = "Field `RX_REG_UPDATE` writer - Write 1 to update rx register configuration."]
pub type RxRegUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - Write 1 to update rx register configuration."]
    #[inline(always)]
    pub fn rx_reg_update(&mut self) -> RxRegUpdateW<'_, RegUpdateSpec> {
        RxRegUpdateW::new(self, 31)
    }
}
#[doc = "Parallel IO FIFO configuration register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUpdateSpec;
impl crate::RegisterSpec for RegUpdateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reg_update::W`](W) writer structure"]
impl crate::Writable for RegUpdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UPDATE to value 0"]
impl crate::Resettable for RegUpdateSpec {}
