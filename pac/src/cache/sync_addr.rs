#[doc = "Register `SYNC_ADDR` reader"]
pub type R = crate::R<SyncAddrSpec>;
#[doc = "Register `SYNC_ADDR` writer"]
pub type W = crate::W<SyncAddrSpec>;
#[doc = "Field `SYNC_ADDR` reader - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
pub type SyncAddrR = crate::FieldReader<u32>;
#[doc = "Field `SYNC_ADDR` writer - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
pub type SyncAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
    #[inline(always)]
    pub fn sync_addr(&self) -> SyncAddrR {
        SyncAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
    #[inline(always)]
    pub fn sync_addr(&mut self) -> SyncAddrW<'_, SyncAddrSpec> {
        SyncAddrW::new(self, 0)
    }
}
#[doc = "Sync address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncAddrSpec;
impl crate::RegisterSpec for SyncAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_addr::R`](R) reader structure"]
impl crate::Readable for SyncAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_addr::W`](W) writer structure"]
impl crate::Writable for SyncAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC_ADDR to value 0"]
impl crate::Resettable for SyncAddrSpec {}
