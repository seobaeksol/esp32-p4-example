#[doc = "Register `IN_CH1_DBG_DATA_L` reader"]
pub type R = crate::R<InCh1DbgDataLSpec>;
#[doc = "Register `IN_CH1_DBG_DATA_L` writer"]
pub type W = crate::W<InCh1DbgDataLSpec>;
#[doc = "Field `H264_IN_CH1_DBG_DATA_L` reader - configures in channel 1 debug data bit 31-0"]
pub type H264InCh1DbgDataLR = crate::FieldReader<u32>;
#[doc = "Field `H264_IN_CH1_DBG_DATA_L` writer - configures in channel 1 debug data bit 31-0"]
pub type H264InCh1DbgDataLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures in channel 1 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_in_ch1_dbg_data_l(&self) -> H264InCh1DbgDataLR {
        H264InCh1DbgDataLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures in channel 1 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_in_ch1_dbg_data_l(&mut self) -> H264InCh1DbgDataLW<'_, InCh1DbgDataLSpec> {
        H264InCh1DbgDataLW::new(self, 0)
    }
}
#[doc = "in channel 1 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ch1_dbg_data_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ch1_dbg_data_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCh1DbgDataLSpec;
impl crate::RegisterSpec for InCh1DbgDataLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ch1_dbg_data_l::R`](R) reader structure"]
impl crate::Readable for InCh1DbgDataLSpec {}
#[doc = "`write(|w| ..)` method takes [`in_ch1_dbg_data_l::W`](W) writer structure"]
impl crate::Writable for InCh1DbgDataLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CH1_DBG_DATA_L to value 0"]
impl crate::Resettable for InCh1DbgDataLSpec {}
