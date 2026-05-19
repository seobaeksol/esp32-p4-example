#[doc = "Register `REF_DEBUG_CONG` reader"]
pub type R = crate::R<RefDebugCongSpec>;
#[doc = "Register `REF_DEBUG_CONG` writer"]
pub type W = crate::W<RefDebugCongSpec>;
#[doc = "Field `DBG_REPLACE_REF_DATA_EN` reader - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
pub type DbgReplaceRefDataEnR = crate::BitReader;
#[doc = "Field `DBG_REPLACE_REF_DATA_EN` writer - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
pub type DbgReplaceRefDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_REPLACE_REF_DATA` reader - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
pub type DbgReplaceRefDataR = crate::FieldReader<u32>;
#[doc = "Field `DBG_REPLACE_REF_DATA` writer - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
pub type DbgReplaceRefDataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_ref_data_en(&self) -> DbgReplaceRefDataEnR {
        DbgReplaceRefDataEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
    #[inline(always)]
    pub fn dbg_replace_ref_data(&self) -> DbgReplaceRefDataR {
        DbgReplaceRefDataR::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether to replace reference picture pixels.\\\\0: not replace\\\\1: replace"]
    #[inline(always)]
    pub fn dbg_replace_ref_data_en(&mut self) -> DbgReplaceRefDataEnW<'_, RefDebugCongSpec> {
        DbgReplaceRefDataEnW::new(self, 0)
    }
    #[doc = "Bits 1:24 - Configure reference picture pixels to be replaced.byte0~2 is VUY"]
    #[inline(always)]
    pub fn dbg_replace_ref_data(&mut self) -> DbgReplaceRefDataW<'_, RefDebugCongSpec> {
        DbgReplaceRefDataW::new(self, 1)
    }
}
#[doc = "Deblocking filter final data debug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_debug_cong::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_debug_cong::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefDebugCongSpec;
impl crate::RegisterSpec for RefDebugCongSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_debug_cong::R`](R) reader structure"]
impl crate::Readable for RefDebugCongSpec {}
#[doc = "`write(|w| ..)` method takes [`ref_debug_cong::W`](W) writer structure"]
impl crate::Writable for RefDebugCongSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_DEBUG_CONG to value 0"]
impl crate::Resettable for RefDebugCongSpec {}
