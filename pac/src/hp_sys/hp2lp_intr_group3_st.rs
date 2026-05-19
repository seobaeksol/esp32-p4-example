#[doc = "Register `HP2LP_INTR_GROUP3_ST` reader"]
pub type R = crate::R<Hp2lpIntrGroup3StSpec>;
#[doc = "Field `H2LP_INTR_GROUP3_ST` reader - Each bit indicates the status of corresponding peripheral interrupt to LP CPU."]
pub type H2lpIntrGroup3StR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Each bit indicates the status of corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group3_st(&self) -> H2lpIntrGroup3StR {
        H2lpIntrGroup3StR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "HpP2LP Interrupt Enable Register Group3\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group3_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hp2lpIntrGroup3StSpec;
impl crate::RegisterSpec for Hp2lpIntrGroup3StSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_intr_group3_st::R`](R) reader structure"]
impl crate::Readable for Hp2lpIntrGroup3StSpec {}
#[doc = "`reset()` method sets HP2LP_INTR_GROUP3_ST to value 0"]
impl crate::Resettable for Hp2lpIntrGroup3StSpec {}
