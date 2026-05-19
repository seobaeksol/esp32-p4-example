#[doc = "Register `SYNC_CTRL` reader"]
pub type R = crate::R<SyncCtrlSpec>;
#[doc = "Register `SYNC_CTRL` writer"]
pub type W = crate::W<SyncCtrlSpec>;
#[doc = "Field `INVALIDATE_ENA` reader - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type InvalidateEnaR = crate::BitReader;
#[doc = "Field `INVALIDATE_ENA` writer - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type InvalidateEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAN_ENA` reader - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CleanEnaR = crate::BitReader;
#[doc = "Field `CLEAN_ENA` writer - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CleanEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEBACK_ENA` reader - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WritebackEnaR = crate::BitReader;
#[doc = "Field `WRITEBACK_ENA` writer - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WritebackEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEBACK_INVALIDATE_ENA` reader - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WritebackInvalidateEnaR = crate::BitReader;
#[doc = "Field `WRITEBACK_INVALIDATE_ENA` writer - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WritebackInvalidateEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_DONE` reader - The bit is used to indicate whether sync operation (invalidate, clean, writeback, writeback_invalidate) is finished or not. 0: not finished. 1: finished."]
pub type SyncDoneR = crate::BitReader;
#[doc = "Field `SYNC_RGID` reader - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
pub type SyncRgidR = crate::FieldReader;
#[doc = "Field `SYNC_RGID` writer - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
pub type SyncRgidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn invalidate_ena(&self) -> InvalidateEnaR {
        InvalidateEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn clean_ena(&self) -> CleanEnaR {
        CleanEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn writeback_ena(&self) -> WritebackEnaR {
        WritebackEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn writeback_invalidate_ena(&self) -> WritebackInvalidateEnaR {
        WritebackInvalidateEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate whether sync operation (invalidate, clean, writeback, writeback_invalidate) is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn sync_done(&self) -> SyncDoneR {
        SyncDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
    #[inline(always)]
    pub fn sync_rgid(&self) -> SyncRgidR {
        SyncRgidR::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn invalidate_ena(&mut self) -> InvalidateEnaW<'_, SyncCtrlSpec> {
        InvalidateEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn clean_ena(&mut self) -> CleanEnaW<'_, SyncCtrlSpec> {
        CleanEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn writeback_ena(&mut self) -> WritebackEnaW<'_, SyncCtrlSpec> {
        WritebackEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn writeback_invalidate_ena(&mut self) -> WritebackInvalidateEnaW<'_, SyncCtrlSpec> {
        WritebackInvalidateEnaW::new(self, 3)
    }
    #[doc = "Bits 5:8 - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
    #[inline(always)]
    pub fn sync_rgid(&mut self) -> SyncRgidW<'_, SyncCtrlSpec> {
        SyncRgidW::new(self, 5)
    }
}
#[doc = "Sync-class operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncCtrlSpec;
impl crate::RegisterSpec for SyncCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_ctrl::R`](R) reader structure"]
impl crate::Readable for SyncCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_ctrl::W`](W) writer structure"]
impl crate::Writable for SyncCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC_CTRL to value 0x01"]
impl crate::Resettable for SyncCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
