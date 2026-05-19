#[doc = "Register `DB_RD_TEMP_DEBUG_CONG` reader"]
pub type R = crate::R<DbRdTempDebugCongSpec>;
#[doc = "Register `DB_RD_TEMP_DEBUG_CONG` writer"]
pub type W = crate::W<DbRdTempDebugCongSpec>;
#[doc = "Field `DBG_REPLACE_RD_DB_TEMP_DATA_EN` reader - Configure deblocking fliter whether or not to replace read temp data.\\\\0: not replace\\\\1: replace"]
pub type DbgReplaceRdDbTempDataEnR = crate::BitReader;
#[doc = "Field `DBG_REPLACE_RD_DB_TEMP_DATA_EN` writer - Configure deblocking fliter whether or not to replace read temp data.\\\\0: not replace\\\\1: replace"]
pub type DbgReplaceRdDbTempDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_REPLACE_RD_DB_TEMP_DATA` reader - Configure deblocking filter read temp data to be replaced.byte0~2 is VUY"]
pub type DbgReplaceRdDbTempDataR = crate::FieldReader<u32>;
#[doc = "Field `DBG_REPLACE_RD_DB_TEMP_DATA` writer - Configure deblocking filter read temp data to be replaced.byte0~2 is VUY"]
pub type DbgReplaceRdDbTempDataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Configure deblocking fliter whether or not to replace read temp data.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_rd_db_temp_data_en(&self) -> DbgReplaceRdDbTempDataEnR {
        DbgReplaceRdDbTempDataEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - Configure deblocking filter read temp data to be replaced.byte0~2 is VUY"]
    #[inline(always)]
    pub fn dbg_replace_rd_db_temp_data(&self) -> DbgReplaceRdDbTempDataR {
        DbgReplaceRdDbTempDataR::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Configure deblocking fliter whether or not to replace read temp data.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_rd_db_temp_data_en(
        &mut self,
    ) -> DbgReplaceRdDbTempDataEnW<'_, DbRdTempDebugCongSpec> {
        DbgReplaceRdDbTempDataEnW::new(self, 0)
    }
    #[doc = "Bits 1:24 - Configure deblocking filter read temp data to be replaced.byte0~2 is VUY"]
    #[inline(always)]
    pub fn dbg_replace_rd_db_temp_data(
        &mut self,
    ) -> DbgReplaceRdDbTempDataW<'_, DbRdTempDebugCongSpec> {
        DbgReplaceRdDbTempDataW::new(self, 1)
    }
}
#[doc = "Deblocking filter read temp debug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`db_rd_temp_debug_cong::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_rd_temp_debug_cong::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbRdTempDebugCongSpec;
impl crate::RegisterSpec for DbRdTempDebugCongSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db_rd_temp_debug_cong::R`](R) reader structure"]
impl crate::Readable for DbRdTempDebugCongSpec {}
#[doc = "`write(|w| ..)` method takes [`db_rd_temp_debug_cong::W`](W) writer structure"]
impl crate::Writable for DbRdTempDebugCongSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DB_RD_TEMP_DEBUG_CONG to value 0"]
impl crate::Resettable for DbRdTempDebugCongSpec {}
