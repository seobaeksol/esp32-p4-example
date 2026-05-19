#[doc = "Register `CLUT_CNT` reader"]
pub type R = crate::R<ClutCntSpec>;
#[doc = "Field `BLEND0_CLUT_CNT` reader - The write data counter of BLEND0 CLUT in fifo mode."]
pub type Blend0ClutCntR = crate::FieldReader<u16>;
#[doc = "Field `BLEND1_CLUT_CNT` reader - The write data counter of BLEND1 CLUT in fifo mode."]
pub type Blend1ClutCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - The write data counter of BLEND0 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend0_clut_cnt(&self) -> Blend0ClutCntR {
        Blend0ClutCntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - The write data counter of BLEND1 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend1_clut_cnt(&self) -> Blend1ClutCntR {
        Blend1ClutCntR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[doc = "BLEND CLUT write counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`clut_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClutCntSpec;
impl crate::RegisterSpec for ClutCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clut_cnt::R`](R) reader structure"]
impl crate::Readable for ClutCntSpec {}
#[doc = "`reset()` method sets CLUT_CNT to value 0"]
impl crate::Resettable for ClutCntSpec {}
