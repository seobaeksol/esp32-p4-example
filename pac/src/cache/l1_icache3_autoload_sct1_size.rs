#[doc = "Register `L1_ICACHE3_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<L1Icache3AutoloadSct1SizeSpec>;
#[doc = "Field `L1_ICACHE3_AUTOLOAD_SCT1_SIZE` reader - Reserved"]
pub type L1Icache3AutoloadSct1SizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_autoload_sct1_size(&self) -> L1Icache3AutoloadSct1SizeR {
        L1Icache3AutoloadSct1SizeR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "L1 instruction Cache 3 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache3_autoload_sct1_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache3AutoloadSct1SizeSpec;
impl crate::RegisterSpec for L1Icache3AutoloadSct1SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache3_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for L1Icache3AutoloadSct1SizeSpec {}
#[doc = "`reset()` method sets L1_ICACHE3_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L1Icache3AutoloadSct1SizeSpec {}
