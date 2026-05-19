#[doc = "Register `CHN_TMP_STATUS` reader"]
pub type R = crate::R<ChnTmpStatusSpec>;
#[doc = "Field `PAD_INACTIVE_STATUS` reader - need_des"]
pub type PadInactiveStatusR = crate::FieldReader<u16>;
#[doc = "Field `PAD_ACTIVE_STATUS` reader - need_des"]
pub type PadActiveStatusR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - need_des"]
    #[inline(always)]
    pub fn pad_inactive_status(&self) -> PadInactiveStatusR {
        PadInactiveStatusR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:29 - need_des"]
    #[inline(always)]
    pub fn pad_active_status(&self) -> PadActiveStatusR {
        PadActiveStatusR::new(((self.bits >> 15) & 0x7fff) as u16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`chn_tmp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnTmpStatusSpec;
impl crate::RegisterSpec for ChnTmpStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chn_tmp_status::R`](R) reader structure"]
impl crate::Readable for ChnTmpStatusSpec {}
#[doc = "`reset()` method sets CHN_TMP_STATUS to value 0"]
impl crate::Resettable for ChnTmpStatusSpec {}
