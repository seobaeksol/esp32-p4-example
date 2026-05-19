#[doc = "Register `DEV_CHAR_TABLE3_LOC1` reader"]
pub type R = crate::R<DevCharTable3Loc1Spec>;
#[doc = "Field `DCT_DEV3_LOC1` reader - NA"]
pub type DctDev3Loc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev3_loc1(&self) -> DctDev3Loc1R {
        DctDev3Loc1R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table3_loc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCharTable3Loc1Spec;
impl crate::RegisterSpec for DevCharTable3Loc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table3_loc1::R`](R) reader structure"]
impl crate::Readable for DevCharTable3Loc1Spec {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE3_LOC1 to value 0"]
impl crate::Resettable for DevCharTable3Loc1Spec {}
