#[doc = "Register `L1_ICACHE2_AUTOLOAD_SCT0_SIZE` reader"]
pub type R = crate::R<L1Icache2AutoloadSct0SizeSpec>;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT0_SIZE` reader - Those bits are used to configure the size of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type L1Icache2AutoloadSct0SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct0_size(&self) -> L1Icache2AutoloadSct0SizeR {
        L1Icache2AutoloadSct0SizeR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "L1 instruction Cache 2 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_autoload_sct0_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2AutoloadSct0SizeSpec;
impl crate::RegisterSpec for L1Icache2AutoloadSct0SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_autoload_sct0_size::R`](R) reader structure"]
impl crate::Readable for L1Icache2AutoloadSct0SizeSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for L1Icache2AutoloadSct0SizeSpec {}
