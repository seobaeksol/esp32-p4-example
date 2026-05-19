#[doc = "Register `L1_ICACHE1_PRELOAD_ADDR` reader"]
pub type R = crate::R<L1Icache1PreloadAddrSpec>;
#[doc = "Register `L1_ICACHE1_PRELOAD_ADDR` writer"]
pub type W = crate::W<L1Icache1PreloadAddrSpec>;
#[doc = "Field `L1_ICACHE1_PRELOAD_ADDR` reader - Those bits are used to configure the start virtual address of preload on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_SIZE_REG"]
pub type L1Icache1PreloadAddrR = crate::FieldReader<u32>;
#[doc = "Field `L1_ICACHE1_PRELOAD_ADDR` writer - Those bits are used to configure the start virtual address of preload on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_SIZE_REG"]
pub type L1Icache1PreloadAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of preload on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_SIZE_REG"]
    #[inline(always)]
    pub fn l1_icache1_preload_addr(&self) -> L1Icache1PreloadAddrR {
        L1Icache1PreloadAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of preload on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_SIZE_REG"]
    #[inline(always)]
    pub fn l1_icache1_preload_addr(
        &mut self,
    ) -> L1Icache1PreloadAddrW<'_, L1Icache1PreloadAddrSpec> {
        L1Icache1PreloadAddrW::new(self, 0)
    }
}
#[doc = "L1 instruction Cache 1 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache1_preload_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_icache1_preload_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache1PreloadAddrSpec;
impl crate::RegisterSpec for L1Icache1PreloadAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache1_preload_addr::R`](R) reader structure"]
impl crate::Readable for L1Icache1PreloadAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_icache1_preload_addr::W`](W) writer structure"]
impl crate::Writable for L1Icache1PreloadAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_ICACHE1_PRELOAD_ADDR to value 0"]
impl crate::Resettable for L1Icache1PreloadAddrSpec {}
