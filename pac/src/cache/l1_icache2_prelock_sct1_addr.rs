#[doc = "Register `L1_ICACHE2_PRELOCK_SCT1_ADDR` reader"]
pub type R = crate::R<L1Icache2PrelockSct1AddrSpec>;
#[doc = "Field `L1_ICACHE2_PRELOCK_SCT1_ADDR` reader - Those bits are used to configure the start virtual address of the second section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT1_SIZE_REG"]
pub type L1Icache2PrelockSct1AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn l1_icache2_prelock_sct1_addr(&self) -> L1Icache2PrelockSct1AddrR {
        L1Icache2PrelockSct1AddrR::new(self.bits)
    }
}
#[doc = "L1 instruction Cache 2 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_prelock_sct1_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2PrelockSct1AddrSpec;
impl crate::RegisterSpec for L1Icache2PrelockSct1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_prelock_sct1_addr::R`](R) reader structure"]
impl crate::Readable for L1Icache2PrelockSct1AddrSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_PRELOCK_SCT1_ADDR to value 0"]
impl crate::Resettable for L1Icache2PrelockSct1AddrSpec {}
