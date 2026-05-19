#[doc = "Register `IN_CH0_DBG_DATA_L` reader"]
pub type R = crate::R<InCh0DbgDataLSpec>;
#[doc = "Register `IN_CH0_DBG_DATA_L` writer"]
pub type W = crate::W<InCh0DbgDataLSpec>;
#[doc = "Field `H264_IN_CH0_DBG_DATA_L` reader - configures in channel 0 debug data bit 31-0"]
pub type H264InCh0DbgDataLR = crate::FieldReader<u32>;
#[doc = "Field `H264_IN_CH0_DBG_DATA_L` writer - configures in channel 0 debug data bit 31-0"]
pub type H264InCh0DbgDataLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures in channel 0 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_in_ch0_dbg_data_l(&self) -> H264InCh0DbgDataLR {
        H264InCh0DbgDataLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures in channel 0 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_in_ch0_dbg_data_l(&mut self) -> H264InCh0DbgDataLW<'_, InCh0DbgDataLSpec> {
        H264InCh0DbgDataLW::new(self, 0)
    }
}
#[doc = "in channel 0 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ch0_dbg_data_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ch0_dbg_data_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCh0DbgDataLSpec;
impl crate::RegisterSpec for InCh0DbgDataLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ch0_dbg_data_l::R`](R) reader structure"]
impl crate::Readable for InCh0DbgDataLSpec {}
#[doc = "`write(|w| ..)` method takes [`in_ch0_dbg_data_l::W`](W) writer structure"]
impl crate::Writable for InCh0DbgDataLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CH0_DBG_DATA_L to value 0"]
impl crate::Resettable for InCh0DbgDataLSpec {}
