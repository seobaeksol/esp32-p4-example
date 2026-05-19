#[doc = "Register `DEV_CHAR_TABLE1_LOC1` reader"]
pub type R = crate::R<DevCharTable1Loc1Spec>;
#[doc = "Field `DCT_DEV1_LOC1` reader - NA"]
pub type DctDev1Loc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev1_loc1(&self) -> DctDev1Loc1R {
        DctDev1Loc1R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table1_loc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCharTable1Loc1Spec;
impl crate::RegisterSpec for DevCharTable1Loc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table1_loc1::R`](R) reader structure"]
impl crate::Readable for DevCharTable1Loc1Spec {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE1_LOC1 to value 0"]
impl crate::Resettable for DevCharTable1Loc1Spec {}
