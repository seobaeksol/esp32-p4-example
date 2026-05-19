#[doc = "Register `L1_ICACHE2_AUTOLOAD_SCT0_ADDR` reader"]
pub type R = crate::R<L1Icache2AutoloadSct0AddrSpec>;
#[doc = "Field `L1_ICACHE2_AUTOLOAD_SCT0_ADDR` reader - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type L1Icache2AutoloadSct0AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_icache2_autoload_sct0_addr(&self) -> L1Icache2AutoloadSct0AddrR {
        L1Icache2AutoloadSct0AddrR::new(self.bits)
    }
}
#[doc = "L1 instruction Cache 2 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_autoload_sct0_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2AutoloadSct0AddrSpec;
impl crate::RegisterSpec for L1Icache2AutoloadSct0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_autoload_sct0_addr::R`](R) reader structure"]
impl crate::Readable for L1Icache2AutoloadSct0AddrSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_AUTOLOAD_SCT0_ADDR to value 0"]
impl crate::Resettable for L1Icache2AutoloadSct0AddrSpec {}
