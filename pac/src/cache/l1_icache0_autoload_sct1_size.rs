#[doc = "Register `L1_ICACHE0_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<L1Icache0AutoloadSct1SizeSpec>;
#[doc = "Register `L1_ICACHE0_AUTOLOAD_SCT1_SIZE` writer"]
pub type W = crate::W<L1Icache0AutoloadSct1SizeSpec>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type L1Icache0AutoloadSct1SizeR = crate::FieldReader<u32>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT1_SIZE` writer - Those bits are used to configure the size of the second section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type L1Icache0AutoloadSct1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct1_size(&self) -> L1Icache0AutoloadSct1SizeR {
        L1Icache0AutoloadSct1SizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct1_size(
        &mut self,
    ) -> L1Icache0AutoloadSct1SizeW<'_, L1Icache0AutoloadSct1SizeSpec> {
        L1Icache0AutoloadSct1SizeW::new(self, 0)
    }
}
#[doc = "L1 instruction Cache 0 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache0_autoload_sct1_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_icache0_autoload_sct1_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache0AutoloadSct1SizeSpec;
impl crate::RegisterSpec for L1Icache0AutoloadSct1SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for L1Icache0AutoloadSct1SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_icache0_autoload_sct1_size::W`](W) writer structure"]
impl crate::Writable for L1Icache0AutoloadSct1SizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_ICACHE0_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L1Icache0AutoloadSct1SizeSpec {}
