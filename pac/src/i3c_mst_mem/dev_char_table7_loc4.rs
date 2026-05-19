#[doc = "Register `DEV_CHAR_TABLE7_LOC4` reader"]
pub type R = crate::R<DevCharTable7Loc4Spec>;
#[doc = "Field `DCT_DEV7_LOC4` reader - NA"]
pub type DctDev7Loc4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev7_loc4(&self) -> DctDev7Loc4R {
        DctDev7Loc4R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table7_loc4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCharTable7Loc4Spec;
impl crate::RegisterSpec for DevCharTable7Loc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table7_loc4::R`](R) reader structure"]
impl crate::Readable for DevCharTable7Loc4Spec {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE7_LOC4 to value 0"]
impl crate::Resettable for DevCharTable7Loc4Spec {}
