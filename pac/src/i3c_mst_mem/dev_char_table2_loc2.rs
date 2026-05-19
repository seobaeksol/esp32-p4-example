#[doc = "Register `DEV_CHAR_TABLE2_LOC2` reader"]
pub type R = crate::R<DevCharTable2Loc2Spec>;
#[doc = "Field `DCT_DEV2_LOC2` reader - NA"]
pub type DctDev2Loc2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev2_loc2(&self) -> DctDev2Loc2R {
        DctDev2Loc2R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table2_loc2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCharTable2Loc2Spec;
impl crate::RegisterSpec for DevCharTable2Loc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table2_loc2::R`](R) reader structure"]
impl crate::Readable for DevCharTable2Loc2Spec {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE2_LOC2 to value 0"]
impl crate::Resettable for DevCharTable2Loc2Spec {}
