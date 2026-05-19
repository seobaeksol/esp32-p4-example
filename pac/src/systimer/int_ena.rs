#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `TARGET(0-2)` reader - interupt%s enable"]
pub type TargetR = crate::BitReader;
#[doc = "Field `TARGET(0-2)` writer - interupt%s enable"]
pub type TargetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "interupt(0-2) enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET0` field.</div>"]
    #[inline(always)]
    pub fn target(&self, n: u8) -> TargetR {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TargetR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "interupt(0-2) enable"]
    #[inline(always)]
    pub fn target_iter(&self) -> impl Iterator<Item = TargetR> + '_ {
        (0..3).map(move |n| TargetR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    pub fn target0(&self) -> TargetR {
        TargetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    pub fn target1(&self) -> TargetR {
        TargetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    pub fn target2(&self) -> TargetR {
        TargetR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "interupt(0-2) enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET0` field.</div>"]
    #[inline(always)]
    pub fn target(&mut self, n: u8) -> TargetW<'_, IntEnaSpec> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TargetW::new(self, n)
    }
    #[doc = "Bit 0 - interupt0 enable"]
    #[inline(always)]
    pub fn target0(&mut self) -> TargetW<'_, IntEnaSpec> {
        TargetW::new(self, 0)
    }
    #[doc = "Bit 1 - interupt1 enable"]
    #[inline(always)]
    pub fn target1(&mut self) -> TargetW<'_, IntEnaSpec> {
        TargetW::new(self, 1)
    }
    #[doc = "Bit 2 - interupt2 enable"]
    #[inline(always)]
    pub fn target2(&mut self) -> TargetW<'_, IntEnaSpec> {
        TargetW::new(self, 2)
    }
}
#[doc = "systimer interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
