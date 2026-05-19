#[doc = "Register `L1_ICACHE0_PRELOAD_SIZE` reader"]
pub type R = crate::R<L1Icache0PreloadSizeSpec>;
#[doc = "Register `L1_ICACHE0_PRELOAD_SIZE` writer"]
pub type W = crate::W<L1Icache0PreloadSizeSpec>;
#[doc = "Field `L1_ICACHE0_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
pub type L1Icache0PreloadSizeR = crate::FieldReader<u16>;
#[doc = "Field `L1_ICACHE0_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
pub type L1Icache0PreloadSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache0_preload_size(&self) -> L1Icache0PreloadSizeR {
        L1Icache0PreloadSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache0_preload_size(
        &mut self,
    ) -> L1Icache0PreloadSizeW<'_, L1Icache0PreloadSizeSpec> {
        L1Icache0PreloadSizeW::new(self, 0)
    }
}
#[doc = "L1 instruction Cache 0 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache0_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_icache0_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache0PreloadSizeSpec;
impl crate::RegisterSpec for L1Icache0PreloadSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_preload_size::R`](R) reader structure"]
impl crate::Readable for L1Icache0PreloadSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_icache0_preload_size::W`](W) writer structure"]
impl crate::Writable for L1Icache0PreloadSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_ICACHE0_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L1Icache0PreloadSizeSpec {}
