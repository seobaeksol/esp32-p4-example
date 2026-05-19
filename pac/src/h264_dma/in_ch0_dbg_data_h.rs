#[doc = "Register `IN_CH0_DBG_DATA_H` reader"]
pub type R = crate::R<InCh0DbgDataHSpec>;
#[doc = "Register `IN_CH0_DBG_DATA_H` writer"]
pub type W = crate::W<InCh0DbgDataHSpec>;
#[doc = "Field `H264_IN_CH0_DBG_DATA_H` reader - configures in channel 0 debug data bit 63-32"]
pub type H264InCh0DbgDataHR = crate::FieldReader<u32>;
#[doc = "Field `H264_IN_CH0_DBG_DATA_H` writer - configures in channel 0 debug data bit 63-32"]
pub type H264InCh0DbgDataHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures in channel 0 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_in_ch0_dbg_data_h(&self) -> H264InCh0DbgDataHR {
        H264InCh0DbgDataHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures in channel 0 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_in_ch0_dbg_data_h(&mut self) -> H264InCh0DbgDataHW<'_, InCh0DbgDataHSpec> {
        H264InCh0DbgDataHW::new(self, 0)
    }
}
#[doc = "in channel 0 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ch0_dbg_data_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ch0_dbg_data_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCh0DbgDataHSpec;
impl crate::RegisterSpec for InCh0DbgDataHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ch0_dbg_data_h::R`](R) reader structure"]
impl crate::Readable for InCh0DbgDataHSpec {}
#[doc = "`write(|w| ..)` method takes [`in_ch0_dbg_data_h::W`](W) writer structure"]
impl crate::Writable for InCh0DbgDataHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CH0_DBG_DATA_H to value 0"]
impl crate::Resettable for InCh0DbgDataHSpec {}
