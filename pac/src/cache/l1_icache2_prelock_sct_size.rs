#[doc = "Register `L1_ICACHE2_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<L1Icache2PrelockSctSizeSpec>;
#[doc = "Field `L1_ICACHE2_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT0_ADDR_REG"]
pub type L1Icache2PrelockSct0SizeR = crate::FieldReader<u16>;
#[doc = "Field `L1_ICACHE2_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT1_ADDR_REG"]
pub type L1Icache2PrelockSct1SizeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache2_prelock_sct0_size(&self) -> L1Icache2PrelockSct0SizeR {
        L1Icache2PrelockSct0SizeR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache2_prelock_sct1_size(&self) -> L1Icache2PrelockSct1SizeR {
        L1Icache2PrelockSct1SizeR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "L1 instruction Cache 2 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_prelock_sct_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2PrelockSctSizeSpec;
impl crate::RegisterSpec for L1Icache2PrelockSctSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for L1Icache2PrelockSctSizeSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_PRELOCK_SCT_SIZE to value 0x3fff_3fff"]
impl crate::Resettable for L1Icache2PrelockSctSizeSpec {
    const RESET_VALUE: u32 = 0x3fff_3fff;
}
