#[doc = "Register `HP2LP_INTR_GROUP2_ST` reader"]
pub type R = crate::R<Hp2lpIntrGroup2StSpec>;
#[doc = "Field `H2LP_INTR_GROUP2_ST` reader - Each bit indicates the status of corresponding peripheral interrupt to LP CPU."]
pub type H2lpIntrGroup2StR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit indicates the status of corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group2_st(&self) -> H2lpIntrGroup2StR {
        H2lpIntrGroup2StR::new(self.bits)
    }
}
#[doc = "HpP2LP Interrupt Enable Register Group2\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group2_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hp2lpIntrGroup2StSpec;
impl crate::RegisterSpec for Hp2lpIntrGroup2StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_intr_group2_st::R`](R) reader structure"]
impl crate::Readable for Hp2lpIntrGroup2StSpec {}
#[doc = "`reset()` method sets HP2LP_INTR_GROUP2_ST to value 0"]
impl crate::Resettable for Hp2lpIntrGroup2StSpec {}
