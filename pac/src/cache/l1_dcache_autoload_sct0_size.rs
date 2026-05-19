#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT0_SIZE` reader"]
pub type R = crate::R<L1DcacheAutoloadSct0SizeSpec>;
#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT0_SIZE` writer"]
pub type W = crate::W<L1DcacheAutoloadSct0SizeSpec>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT0_SIZE` reader - Those bits are used to configure the size of the first section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT0_ADDR and L1_DCACHE_AUTOLOAD_SCT0_ENA."]
pub type L1DcacheAutoloadSct0SizeR = crate::FieldReader<u32>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT0_SIZE` writer - Those bits are used to configure the size of the first section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT0_ADDR and L1_DCACHE_AUTOLOAD_SCT0_ENA."]
pub type L1DcacheAutoloadSct0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the first section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT0_ADDR and L1_DCACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct0_size(&self) -> L1DcacheAutoloadSct0SizeR {
        L1DcacheAutoloadSct0SizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the first section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT0_ADDR and L1_DCACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct0_size(
        &mut self,
    ) -> L1DcacheAutoloadSct0SizeW<'_, L1DcacheAutoloadSct0SizeSpec> {
        L1DcacheAutoloadSct0SizeW::new(self, 0)
    }
}
#[doc = "L1 data Cache autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_autoload_sct0_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct0_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcacheAutoloadSct0SizeSpec;
impl crate::RegisterSpec for L1DcacheAutoloadSct0SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_autoload_sct0_size::R`](R) reader structure"]
impl crate::Readable for L1DcacheAutoloadSct0SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_autoload_sct0_size::W`](W) writer structure"]
impl crate::Writable for L1DcacheAutoloadSct0SizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for L1DcacheAutoloadSct0SizeSpec {}
