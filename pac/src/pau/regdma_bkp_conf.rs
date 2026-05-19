#[doc = "Register `REGDMA_BKP_CONF` reader"]
pub type R = crate::R<RegdmaBkpConfSpec>;
#[doc = "Register `REGDMA_BKP_CONF` writer"]
pub type W = crate::W<RegdmaBkpConfSpec>;
#[doc = "Field `READ_INTERVAL` reader - Link read_interval"]
pub type ReadIntervalR = crate::FieldReader;
#[doc = "Field `READ_INTERVAL` writer - Link read_interval"]
pub type ReadIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LINK_TOUT_THRES` reader - link wait timeout threshold"]
pub type LinkToutThresR = crate::FieldReader<u16>;
#[doc = "Field `LINK_TOUT_THRES` writer - link wait timeout threshold"]
pub type LinkToutThresW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BURST_LIMIT` reader - burst limit"]
pub type BurstLimitR = crate::FieldReader;
#[doc = "Field `BURST_LIMIT` writer - burst limit"]
pub type BurstLimitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BACKUP_TOUT_THRES` reader - Backup timeout threshold"]
pub type BackupToutThresR = crate::FieldReader<u16>;
#[doc = "Field `BACKUP_TOUT_THRES` writer - Backup timeout threshold"]
pub type BackupToutThresW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:6 - Link read_interval"]
    #[inline(always)]
    pub fn read_interval(&self) -> ReadIntervalR {
        ReadIntervalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:16 - link wait timeout threshold"]
    #[inline(always)]
    pub fn link_tout_thres(&self) -> LinkToutThresR {
        LinkToutThresR::new(((self.bits >> 7) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:21 - burst limit"]
    #[inline(always)]
    pub fn burst_limit(&self) -> BurstLimitR {
        BurstLimitR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:31 - Backup timeout threshold"]
    #[inline(always)]
    pub fn backup_tout_thres(&self) -> BackupToutThresR {
        BackupToutThresR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Link read_interval"]
    #[inline(always)]
    pub fn read_interval(&mut self) -> ReadIntervalW<'_, RegdmaBkpConfSpec> {
        ReadIntervalW::new(self, 0)
    }
    #[doc = "Bits 7:16 - link wait timeout threshold"]
    #[inline(always)]
    pub fn link_tout_thres(&mut self) -> LinkToutThresW<'_, RegdmaBkpConfSpec> {
        LinkToutThresW::new(self, 7)
    }
    #[doc = "Bits 17:21 - burst limit"]
    #[inline(always)]
    pub fn burst_limit(&mut self) -> BurstLimitW<'_, RegdmaBkpConfSpec> {
        BurstLimitW::new(self, 17)
    }
    #[doc = "Bits 22:31 - Backup timeout threshold"]
    #[inline(always)]
    pub fn backup_tout_thres(&mut self) -> BackupToutThresW<'_, RegdmaBkpConfSpec> {
        BackupToutThresW::new(self, 22)
    }
}
#[doc = "backup config\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_bkp_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_bkp_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegdmaBkpConfSpec;
impl crate::RegisterSpec for RegdmaBkpConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_bkp_conf::R`](R) reader structure"]
impl crate::Readable for RegdmaBkpConfSpec {}
#[doc = "`write(|w| ..)` method takes [`regdma_bkp_conf::W`](W) writer structure"]
impl crate::Writable for RegdmaBkpConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_BKP_CONF to value 0x7d10_1920"]
impl crate::Resettable for RegdmaBkpConfSpec {
    const RESET_VALUE: u32 = 0x7d10_1920;
}
