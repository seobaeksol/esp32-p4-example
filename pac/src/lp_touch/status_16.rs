#[doc = "Register `STATUS_16` reader"]
pub type R = crate::R<Status16Spec>;
#[doc = "Field `APPROACH_PAD2_CNT` reader - need_des"]
pub type ApproachPad2CntR = crate::FieldReader;
#[doc = "Field `APPROACH_PAD1_CNT` reader - need_des"]
pub type ApproachPad1CntR = crate::FieldReader;
#[doc = "Field `APPROACH_PAD0_CNT` reader - need_des"]
pub type ApproachPad0CntR = crate::FieldReader;
#[doc = "Field `SLP_APPROACH_CNT` reader - need_des"]
pub type SlpApproachCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn approach_pad2_cnt(&self) -> ApproachPad2CntR {
        ApproachPad2CntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn approach_pad1_cnt(&self) -> ApproachPad1CntR {
        ApproachPad1CntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn approach_pad0_cnt(&self) -> ApproachPad0CntR {
        ApproachPad0CntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn slp_approach_cnt(&self) -> SlpApproachCntR {
        SlpApproachCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status16Spec;
impl crate::RegisterSpec for Status16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_16::R`](R) reader structure"]
impl crate::Readable for Status16Spec {}
#[doc = "`reset()` method sets STATUS_16 to value 0"]
impl crate::Resettable for Status16Spec {}
