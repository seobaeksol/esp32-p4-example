#[doc = "Register `B_SYS_CONF` reader"]
pub type R = crate::R<BSysConfSpec>;
#[doc = "Register `B_SYS_CONF` writer"]
pub type W = crate::W<BSysConfSpec>;
#[doc = "Field `B_DB_TMP_READY_TRIGGER_MB_NUM` reader - Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
pub type BDbTmpReadyTriggerMbNumR = crate::FieldReader;
#[doc = "Field `B_DB_TMP_READY_TRIGGER_MB_NUM` writer - Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
pub type BDbTmpReadyTriggerMbNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_REC_READY_TRIGGER_MB_LINES` reader - Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
pub type BRecReadyTriggerMbLinesR = crate::FieldReader;
#[doc = "Field `B_REC_READY_TRIGGER_MB_LINES` writer - Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
pub type BRecReadyTriggerMbLinesW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_INTRA_COST_CMP_OFFSET` reader - Configures video B intra cost offset when I MB compared with P MB."]
pub type BIntraCostCmpOffsetR = crate::FieldReader<u16>;
#[doc = "Field `B_INTRA_COST_CMP_OFFSET` writer - Configures video B intra cost offset when I MB compared with P MB."]
pub type BIntraCostCmpOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
    #[inline(always)]
    pub fn b_db_tmp_ready_trigger_mb_num(&self) -> BDbTmpReadyTriggerMbNumR {
        BDbTmpReadyTriggerMbNumR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
    #[inline(always)]
    pub fn b_rec_ready_trigger_mb_lines(&self) -> BRecReadyTriggerMbLinesR {
        BRecReadyTriggerMbLinesR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:29 - Configures video B intra cost offset when I MB compared with P MB."]
    #[inline(always)]
    pub fn b_intra_cost_cmp_offset(&self) -> BIntraCostCmpOffsetR {
        BIntraCostCmpOffsetR::new(((self.bits >> 14) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3."]
    #[inline(always)]
    pub fn b_db_tmp_ready_trigger_mb_num(&mut self) -> BDbTmpReadyTriggerMbNumW<'_, BSysConfSpec> {
        BDbTmpReadyTriggerMbNumW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4."]
    #[inline(always)]
    pub fn b_rec_ready_trigger_mb_lines(&mut self) -> BRecReadyTriggerMbLinesW<'_, BSysConfSpec> {
        BRecReadyTriggerMbLinesW::new(self, 7)
    }
    #[doc = "Bits 14:29 - Configures video B intra cost offset when I MB compared with P MB."]
    #[inline(always)]
    pub fn b_intra_cost_cmp_offset(&mut self) -> BIntraCostCmpOffsetW<'_, BSysConfSpec> {
        BIntraCostCmpOffsetW::new(self, 14)
    }
}
#[doc = "Video B system level configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_sys_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_sys_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSysConfSpec;
impl crate::RegisterSpec for BSysConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_sys_conf::R`](R) reader structure"]
impl crate::Readable for BSysConfSpec {}
#[doc = "`write(|w| ..)` method takes [`b_sys_conf::W`](W) writer structure"]
impl crate::Writable for BSysConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_SYS_CONF to value 0x0203"]
impl crate::Resettable for BSysConfSpec {
    const RESET_VALUE: u32 = 0x0203;
}
