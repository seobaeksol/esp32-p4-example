#[doc = "Register `L1_ICACHE2_ACS_FAIL_ADDR` reader"]
pub type R = crate::R<L1Icache2AcsFailAddrSpec>;
#[doc = "Field `L1_ICACHE2_FAIL_ADDR` reader - The register records the address of fail-access when cache2 accesses L1-ICache."]
pub type L1Icache2FailAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the address of fail-access when cache2 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache2_fail_addr(&self) -> L1Icache2FailAddrR {
        L1Icache2FailAddrR::new(self.bits)
    }
}
#[doc = "L1-ICache0 Access Fail Address information register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_acs_fail_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2AcsFailAddrSpec;
impl crate::RegisterSpec for L1Icache2AcsFailAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_acs_fail_addr::R`](R) reader structure"]
impl crate::Readable for L1Icache2AcsFailAddrSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_ACS_FAIL_ADDR to value 0"]
impl crate::Resettable for L1Icache2AcsFailAddrSpec {}
