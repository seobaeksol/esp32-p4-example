#[doc = "Register `IN_CH2_DBG_DATA_H` reader"]
pub type R = crate::R<InCh2DbgDataHSpec>;
#[doc = "Register `IN_CH2_DBG_DATA_H` writer"]
pub type W = crate::W<InCh2DbgDataHSpec>;
#[doc = "Field `H264_IN_CH2_DBG_DATA_H` reader - configures in channel 2 debug data bit 63-32"]
pub type H264InCh2DbgDataHR = crate::FieldReader<u32>;
#[doc = "Field `H264_IN_CH2_DBG_DATA_H` writer - configures in channel 2 debug data bit 63-32"]
pub type H264InCh2DbgDataHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures in channel 2 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_in_ch2_dbg_data_h(&self) -> H264InCh2DbgDataHR {
        H264InCh2DbgDataHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - configures in channel 2 debug data bit 63-32"]
    #[inline(always)]
    pub fn h264_in_ch2_dbg_data_h(&mut self) -> H264InCh2DbgDataHW<'_, InCh2DbgDataHSpec> {
        H264InCh2DbgDataHW::new(self, 0)
    }
}
#[doc = "in channel 2 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ch2_dbg_data_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ch2_dbg_data_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InCh2DbgDataHSpec;
impl crate::RegisterSpec for InCh2DbgDataHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ch2_dbg_data_h::R`](R) reader structure"]
impl crate::Readable for InCh2DbgDataHSpec {}
#[doc = "`write(|w| ..)` method takes [`in_ch2_dbg_data_h::W`](W) writer structure"]
impl crate::Writable for InCh2DbgDataHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CH2_DBG_DATA_H to value 0"]
impl crate::Resettable for InCh2DbgDataHSpec {}
