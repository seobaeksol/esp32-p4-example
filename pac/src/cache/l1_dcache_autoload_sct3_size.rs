#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT3_SIZE` reader"]
pub type R = crate::R<L1DcacheAutoloadSct3SizeSpec>;
#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT3_SIZE` writer"]
pub type W = crate::W<L1DcacheAutoloadSct3SizeSpec>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT3_SIZE` reader - Those bits are used to configure the size of the fourth section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT3_ADDR and L1_DCACHE_AUTOLOAD_SCT3_ENA."]
pub type L1DcacheAutoloadSct3SizeR = crate::FieldReader<u32>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT3_SIZE` writer - Those bits are used to configure the size of the fourth section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT3_ADDR and L1_DCACHE_AUTOLOAD_SCT3_ENA."]
pub type L1DcacheAutoloadSct3SizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the fourth section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT3_ADDR and L1_DCACHE_AUTOLOAD_SCT3_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct3_size(&self) -> L1DcacheAutoloadSct3SizeR {
        L1DcacheAutoloadSct3SizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the fourth section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT3_ADDR and L1_DCACHE_AUTOLOAD_SCT3_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct3_size(
        &mut self,
    ) -> L1DcacheAutoloadSct3SizeW<'_, L1DcacheAutoloadSct3SizeSpec> {
        L1DcacheAutoloadSct3SizeW::new(self, 0)
    }
}
#[doc = "L1 data Cache autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_autoload_sct3_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct3_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcacheAutoloadSct3SizeSpec;
impl crate::RegisterSpec for L1DcacheAutoloadSct3SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_autoload_sct3_size::R`](R) reader structure"]
impl crate::Readable for L1DcacheAutoloadSct3SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_autoload_sct3_size::W`](W) writer structure"]
impl crate::Writable for L1DcacheAutoloadSct3SizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_AUTOLOAD_SCT3_SIZE to value 0"]
impl crate::Resettable for L1DcacheAutoloadSct3SizeSpec {}
