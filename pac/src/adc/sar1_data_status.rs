#[doc = "Register `SAR1_DATA_STATUS` reader"]
pub type R = crate::R<Sar1DataStatusSpec>;
#[doc = "Field `APB_SARADC1_DATA` reader - need_des"]
pub type ApbSaradc1DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn apb_saradc1_data(&self) -> ApbSaradc1DataR {
        ApbSaradc1DataR::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_data_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar1DataStatusSpec;
impl crate::RegisterSpec for Sar1DataStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_data_status::R`](R) reader structure"]
impl crate::Readable for Sar1DataStatusSpec {}
#[doc = "`reset()` method sets SAR1_DATA_STATUS to value 0"]
impl crate::Resettable for Sar1DataStatusSpec {}
