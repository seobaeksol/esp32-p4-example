#[doc = "Register `SYNC_SIZE` reader"]
pub type R = crate::R<SyncSizeSpec>;
#[doc = "Register `SYNC_SIZE` writer"]
pub type W = crate::W<SyncSizeSpec>;
#[doc = "Field `SYNC_SIZE` reader - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
pub type SyncSizeR = crate::FieldReader<u32>;
#[doc = "Field `SYNC_SIZE` writer - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
pub type SyncSizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
    #[inline(always)]
    pub fn sync_size(&self) -> SyncSizeR {
        SyncSizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
    #[inline(always)]
    pub fn sync_size(&mut self) -> SyncSizeW<'_, SyncSizeSpec> {
        SyncSizeW::new(self, 0)
    }
}
#[doc = "Sync size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSizeSpec;
impl crate::RegisterSpec for SyncSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_size::R`](R) reader structure"]
impl crate::Readable for SyncSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_size::W`](W) writer structure"]
impl crate::Writable for SyncSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC_SIZE to value 0"]
impl crate::Resettable for SyncSizeSpec {}
