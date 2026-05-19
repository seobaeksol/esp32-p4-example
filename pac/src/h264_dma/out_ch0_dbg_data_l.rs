#[doc = "Register `OUT_CH0_DBG_DATA_L` reader"]
pub type R = crate::R<OutCh0DbgDataLSpec>;
#[doc = "Register `OUT_CH0_DBG_DATA_L` writer"]
pub type W = crate::W<OutCh0DbgDataLSpec>;
#[doc = "Field `H264_OUT_CH0_DBG_DATA_L` reader - configures out channel 0 debug data bit 31-0"]
pub type H264OutCh0DbgDataLR = crate::FieldReader<u32>;
#[doc = "Field `H264_OUT_CH0_DBG_DATA_L` writer - configures out channel 0 debug data bit 31-0"]
pub type H264OutCh0DbgDataLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures out channel 0 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_out_ch0_dbg_data_l(&self) -> H264OutCh0DbgDataLR {
        H264OutCh0DbgDataLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures out channel 0 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_out_ch0_dbg_data_l(&mut self) -> H264OutCh0DbgDataLW<'_, OutCh0DbgDataLSpec> {
        H264OutCh0DbgDataLW::new(self, 0)
    }
}
#[doc = "out channel 0 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ch0_dbg_data_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_ch0_dbg_data_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutCh0DbgDataLSpec;
impl crate::RegisterSpec for OutCh0DbgDataLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ch0_dbg_data_l::R`](R) reader structure"]
impl crate::Readable for OutCh0DbgDataLSpec {}
#[doc = "`write(|w| ..)` method takes [`out_ch0_dbg_data_l::W`](W) writer structure"]
impl crate::Writable for OutCh0DbgDataLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CH0_DBG_DATA_L to value 0"]
impl crate::Resettable for OutCh0DbgDataLSpec {}
