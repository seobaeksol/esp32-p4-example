#[doc = "Register `SAR2_DATA_STATUS` reader"]
pub type R = crate::R<Sar2DataStatusSpec>;
#[doc = "Field `APB_SARADC2_DATA` reader - need_des"]
pub type ApbSaradc2DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn apb_saradc2_data(&self) -> ApbSaradc2DataR {
        ApbSaradc2DataR::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_data_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar2DataStatusSpec;
impl crate::RegisterSpec for Sar2DataStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_data_status::R`](R) reader structure"]
impl crate::Readable for Sar2DataStatusSpec {}
#[doc = "`reset()` method sets SAR2_DATA_STATUS to value 0"]
impl crate::Resettable for Sar2DataStatusSpec {}
