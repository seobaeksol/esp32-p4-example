#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `SUPER_WDT` writer - need_des"]
pub type SuperWdtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LP_WDT` writer - need_des"]
pub type LpWdtW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn super_wdt(&mut self) -> SuperWdtW<'_, IntClrSpec> {
        SuperWdtW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_wdt(&mut self) -> LpWdtW<'_, IntClrSpec> {
        LpWdtW::new(self, 31)
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
