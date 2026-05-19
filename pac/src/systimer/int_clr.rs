#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `TARGET(0-2)` writer - interupt%s clear"]
pub type TargetW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "interupt(0-2) clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET0` field.</div>"]
    #[inline(always)]
    pub fn target(&mut self, n: u8) -> TargetW<'_, IntClrSpec> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TargetW::new(self, n)
    }
    #[doc = "Bit 0 - interupt0 clear"]
    #[inline(always)]
    pub fn target0(&mut self) -> TargetW<'_, IntClrSpec> {
        TargetW::new(self, 0)
    }
    #[doc = "Bit 1 - interupt1 clear"]
    #[inline(always)]
    pub fn target1(&mut self) -> TargetW<'_, IntClrSpec> {
        TargetW::new(self, 1)
    }
    #[doc = "Bit 2 - interupt2 clear"]
    #[inline(always)]
    pub fn target2(&mut self) -> TargetW<'_, IntClrSpec> {
        TargetW::new(self, 2)
    }
}
#[doc = "systimer interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
