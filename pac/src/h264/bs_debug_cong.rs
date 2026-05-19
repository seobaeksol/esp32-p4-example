#[doc = "Register `BS_DEBUG_CONG` reader"]
pub type R = crate::R<BsDebugCongSpec>;
#[doc = "Register `BS_DEBUG_CONG` writer"]
pub type W = crate::W<BsDebugCongSpec>;
#[doc = "Field `DBG_REPLACE_WR_BS_DATA_EN` reader - Configures whether or not to replace bs data.\\\\0: not replace\\\\1: replace"]
pub type DbgReplaceWrBsDataEnR = crate::BitReader;
#[doc = "Field `DBG_REPLACE_WR_BS_DATA_EN` writer - Configures whether or not to replace bs data.\\\\0: not replace\\\\1: replace"]
pub type DbgReplaceWrBsDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_REPLACE_WR_BS_DATA` reader - Configures bs data to be replaced"]
pub type DbgReplaceWrBsDataR = crate::FieldReader;
#[doc = "Field `DBG_REPLACE_WR_BS_DATA` writer - Configures bs data to be replaced"]
pub type DbgReplaceWrBsDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to replace bs data.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_wr_bs_data_en(&self) -> DbgReplaceWrBsDataEnR {
        DbgReplaceWrBsDataEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Configures bs data to be replaced"]
    #[inline(always)]
    pub fn dbg_replace_wr_bs_data(&self) -> DbgReplaceWrBsDataR {
        DbgReplaceWrBsDataR::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to replace bs data.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_wr_bs_data_en(&mut self) -> DbgReplaceWrBsDataEnW<'_, BsDebugCongSpec> {
        DbgReplaceWrBsDataEnW::new(self, 0)
    }
    #[doc = "Bits 1:8 - Configures bs data to be replaced"]
    #[inline(always)]
    pub fn dbg_replace_wr_bs_data(&mut self) -> DbgReplaceWrBsDataW<'_, BsDebugCongSpec> {
        DbgReplaceWrBsDataW::new(self, 1)
    }
}
#[doc = "Encode bitstream debug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_debug_cong::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_debug_cong::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsDebugCongSpec;
impl crate::RegisterSpec for BsDebugCongSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_debug_cong::R`](R) reader structure"]
impl crate::Readable for BsDebugCongSpec {}
#[doc = "`write(|w| ..)` method takes [`bs_debug_cong::W`](W) writer structure"]
impl crate::Writable for BsDebugCongSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_DEBUG_CONG to value 0"]
impl crate::Resettable for BsDebugCongSpec {}
