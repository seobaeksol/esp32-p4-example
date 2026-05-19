#[doc = "Register `DEV_CHAR_TABLE11_LOC3` reader"]
pub type R = crate::R<DevCharTable11Loc3Spec>;
#[doc = "Field `DCT_DEV11_LOC3` reader - NA"]
pub type DctDev11Loc3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev11_loc3(&self) -> DctDev11Loc3R {
        DctDev11Loc3R::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table11_loc3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCharTable11Loc3Spec;
impl crate::RegisterSpec for DevCharTable11Loc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table11_loc3::R`](R) reader structure"]
impl crate::Readable for DevCharTable11Loc3Spec {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE11_LOC3 to value 0"]
impl crate::Resettable for DevCharTable11Loc3Spec {}
