#[doc = "Register `COMPVER0` reader"]
pub type R = crate::R<Compver0Spec>;
#[doc = "Field `DMAC_COMPVER` reader - NA"]
pub type DmacCompverR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dmac_compver(&self) -> DmacCompverR {
        DmacCompverR::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`compver0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compver0Spec;
impl crate::RegisterSpec for Compver0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compver0::R`](R) reader structure"]
impl crate::Readable for Compver0Spec {}
#[doc = "`reset()` method sets COMPVER0 to value 0x3230_302a"]
impl crate::Resettable for Compver0Spec {
    const RESET_VALUE: u32 = 0x3230_302a;
}
