#[doc = "Register `DLY_NUM_REC4` reader"]
pub type R = crate::R<DlyNumRec4Spec>;
#[doc = "Field `SITE0_DELAY_NUM_VT0_MIN_RECORD` reader - needs field desc"]
pub type Site0DelayNumVt0MinRecordR = crate::FieldReader;
#[doc = "Field `SITE0_DELAY_NUM_VT1_MIN_RECORD` reader - needs field desc"]
pub type Site0DelayNumVt1MinRecordR = crate::FieldReader;
#[doc = "Field `SITE0_DELAY_NUM_VT2_MIN_RECORD` reader - needs field desc"]
pub type Site0DelayNumVt2MinRecordR = crate::FieldReader;
#[doc = "Field `SITE0_DELAY_NUM_VT3_MIN_RECORD` reader - needs field desc"]
pub type Site0DelayNumVt3MinRecordR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - needs field desc"]
    #[inline(always)]
    pub fn site0_delay_num_vt0_min_record(&self) -> Site0DelayNumVt0MinRecordR {
        Site0DelayNumVt0MinRecordR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - needs field desc"]
    #[inline(always)]
    pub fn site0_delay_num_vt1_min_record(&self) -> Site0DelayNumVt1MinRecordR {
        Site0DelayNumVt1MinRecordR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - needs field desc"]
    #[inline(always)]
    pub fn site0_delay_num_vt2_min_record(&self) -> Site0DelayNumVt2MinRecordR {
        Site0DelayNumVt2MinRecordR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - needs field desc"]
    #[inline(always)]
    pub fn site0_delay_num_vt3_min_record(&self) -> Site0DelayNumVt3MinRecordR {
        Site0DelayNumVt3MinRecordR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dly_num_rec4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlyNumRec4Spec;
impl crate::RegisterSpec for DlyNumRec4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dly_num_rec4::R`](R) reader structure"]
impl crate::Readable for DlyNumRec4Spec {}
#[doc = "`reset()` method sets DLY_NUM_REC4 to value 0"]
impl crate::Resettable for DlyNumRec4Spec {}
