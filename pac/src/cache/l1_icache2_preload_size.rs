#[doc = "Register `L1_ICACHE2_PRELOAD_SIZE` reader"]
pub type R = crate::R<L1Icache2PreloadSizeSpec>;
#[doc = "Field `L1_ICACHE2_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_ADDR_REG"]
pub type L1Icache2PreloadSizeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache2_preload_size(&self) -> L1Icache2PreloadSizeR {
        L1Icache2PreloadSizeR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "L1 instruction Cache 2 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_preload_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2PreloadSizeSpec;
impl crate::RegisterSpec for L1Icache2PreloadSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_preload_size::R`](R) reader structure"]
impl crate::Readable for L1Icache2PreloadSizeSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L1Icache2PreloadSizeSpec {}
