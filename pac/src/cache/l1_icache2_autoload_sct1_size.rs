#[doc = "Register `L1_ICACHE2_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<L1Icache2AutoloadSct1SizeSpec>;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type L1Icache2AutoloadSct1SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct1_size(&self) -> L1Icache2AutoloadSct1SizeR {
        L1Icache2AutoloadSct1SizeR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "L1 instruction Cache 2 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_autoload_sct1_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2AutoloadSct1SizeSpec;
impl crate::RegisterSpec for L1Icache2AutoloadSct1SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for L1Icache2AutoloadSct1SizeSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L1Icache2AutoloadSct1SizeSpec {}
