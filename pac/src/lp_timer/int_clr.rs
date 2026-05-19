#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `OVERFLOW` writer - need_des"]
pub type OverflowW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SOC_WAKEUP` writer - need_des"]
pub type SocWakeupW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, IntClrSpec> {
        OverflowW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup(&mut self) -> SocWakeupW<'_, IntClrSpec> {
        SocWakeupW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xc000_0000;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
