#[doc = "Register `DEV_CHAR_TABLE8_LOC1` reader"]
pub type R = crate::R<DevCharTable8Loc1Spec>;
#[doc = "Field `DCT_DEV8_LOC1` reader - NA"]
pub type DctDev8Loc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev8_loc1(&self) -> DctDev8Loc1R {
        DctDev8Loc1R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table8_loc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCharTable8Loc1Spec;
impl crate::RegisterSpec for DevCharTable8Loc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table8_loc1::R`](R) reader structure"]
impl crate::Readable for DevCharTable8Loc1Spec {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE8_LOC1 to value 0"]
impl crate::Resettable for DevCharTable8Loc1Spec {}
