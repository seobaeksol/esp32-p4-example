#[doc = "Register `IN_CH3_DBG_DATA_H` reader"]
pub type R = crate::R<InCh3DbgDataHSpec>;
#[doc = "Register `IN_CH3_DBG_DATA_H` writer"]
pub type W = crate::W<InCh3DbgDataHSpec>;
#[doc = "Field `H264_IN_CH3_DBG_DATA_H` reader - configures in channel 3 debug data bit 63-32"]
pub type H264InCh3DbgDataHR = crate::FieldReader<u32>;
#[doc = "Field `H264_IN_CH3_DBG_DATA_H` writer - configures in channel 3 debug data bit 63-32"]
pub type H264InCh3DbgDataHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures in channel 3 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_in_ch3_dbg_data_h(&self) -> H264InCh3DbgDataHR {
        H264InCh3DbgDataHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures in channel 3 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_in_ch3_dbg_data_h(&mut self) -> H264InCh3DbgDataHW<'_, InCh3DbgDataHSpec> {
        H264InCh3DbgDataHW::new(self, 0)
    }
}
#[doc = "in channel 3 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ch3_dbg_data_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ch3_dbg_data_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCh3DbgDataHSpec;
impl crate::RegisterSpec for InCh3DbgDataHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ch3_dbg_data_h::R`](R) reader structure"]
impl crate::Readable for InCh3DbgDataHSpec {}
#[doc = "`write(|w| ..)` method takes [`in_ch3_dbg_data_h::W`](W) writer structure"]
impl crate::Writable for InCh3DbgDataHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CH3_DBG_DATA_H to value 0"]
impl crate::Resettable for InCh3DbgDataHSpec {}
