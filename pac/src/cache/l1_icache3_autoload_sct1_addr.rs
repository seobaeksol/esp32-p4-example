#[doc = "Register `L1_ICACHE3_AUTOLOAD_SCT1_ADDR` reader"]
pub type R = crate::R<L1Icache3AutoloadSct1AddrSpec>;
#[doc = "Field `L1_ICACHE3_AUTOLOAD_SCT1_ADDR` reader - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-ICache3. Note that it should be used together with L1_ICACHE3_AUTOLOAD_SCT1_SIZE and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type L1Icache3AutoloadSct1AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section for autoload operation on L1-ICache3. Note that it should be used together with L1_ICACHE3_AUTOLOAD_SCT1_SIZE and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_icache3_autoload_sct1_addr(&self) -> L1Icache3AutoloadSct1AddrR {
        L1Icache3AutoloadSct1AddrR::new(self.bits)
    }
}
#[doc = "L1 instruction Cache 3 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache3_autoload_sct1_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache3AutoloadSct1AddrSpec;
impl crate::RegisterSpec for L1Icache3AutoloadSct1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache3_autoload_sct1_addr::R`](R) reader structure"]
impl crate::Readable for L1Icache3AutoloadSct1AddrSpec {}
#[doc = "`reset()` method sets L1_ICACHE3_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for L1Icache3AutoloadSct1AddrSpec {}
