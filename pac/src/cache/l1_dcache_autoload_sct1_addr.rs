#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT1_ADDR` reader"]
pub type R = crate::R<L1DcacheAutoloadSct1AddrSpec>;
#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT1_ADDR` writer"]
pub type W = crate::W<L1DcacheAutoloadSct1AddrSpec>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT1_ADDR` reader - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_SIZE and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
pub type L1DcacheAutoloadSct1AddrR = crate::FieldReader<u32>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT1_ADDR` writer - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_SIZE and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
pub type L1DcacheAutoloadSct1AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_SIZE and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct1_addr(&self) -> L1DcacheAutoloadSct1AddrR {
        L1DcacheAutoloadSct1AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_SIZE and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct1_addr(
        &mut self,
    ) -> L1DcacheAutoloadSct1AddrW<'_, L1DcacheAutoloadSct1AddrSpec> {
        L1DcacheAutoloadSct1AddrW::new(self, 0)
    }
}
#[doc = "L1 data Cache autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_autoload_sct1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcacheAutoloadSct1AddrSpec;
impl crate::RegisterSpec for L1DcacheAutoloadSct1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_autoload_sct1_addr::R`](R) reader structure"]
impl crate::Readable for L1DcacheAutoloadSct1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_autoload_sct1_addr::W`](W) writer structure"]
impl crate::Writable for L1DcacheAutoloadSct1AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for L1DcacheAutoloadSct1AddrSpec {}
